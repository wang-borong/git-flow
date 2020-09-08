//#[allow(unused)]
use crate::{
    utils::get_head,
    error::{
        Error,
        Result,
    },
};

use std:: {
    path::{
        Path,
        PathBuf,
    },
    io::{
        Write,
        stdout,
    }
};

use git2::{
    Repository,
    Signature,
    ErrorCode,
    Oid,
    ObjectType,
    RemoteCallbacks,
    AutotagOption,
    FetchOptions,
    AnnotatedCommit,
    Reference,
    build::CheckoutBuilder,
    PushOptions,
    Cred,
    RebaseOptions,
    BranchType,
};

pub struct GitcRepo(pub Repository);

impl From<Repository> for GitcRepo {
    fn from(repo: Repository) -> Self {
        Self(repo)
    }
}

impl Into<Repository> for GitcRepo {
    fn into(self) -> Repository {
        self.0
    }
}

impl GitcRepo {
    // create a new git repo
    pub fn new(p: &PathBuf) -> Self {
        let repo = Repository::open(&p);
        let repo = match repo {
            Ok(repo) => repo,
            Err(e) => {
                println!("{}, create a new one", e);
                Repository::init(&p).unwrap()
            }
        };
        GitcRepo::from(repo)
    }

    pub fn init(&self) -> Result<()> {
        let repo = &self.0;

        // Check if the git repo is empty
        if !repo.is_empty()? {
            // Just return if it is not empty
            return Ok(())
        }

        let sig = repo.signature()?;
        let tree_id = {
            let mut index = repo.index()?;
            index.write_tree()?
        };

        let tree = repo.find_tree(tree_id)?;

        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

        Ok(())
    }

    pub fn checkout(&self, branch: &str) -> Result<()> {
        let repo = &self.0;
        let refname = format!("refs/heads/{}", branch);
        repo.set_head(&refname)?;
        repo.checkout_head(None)?;

        Ok(())
    }

    pub fn branch(&self, branch: &str) -> Result<()> {
        let repo = &self.0;
        if let Some(oid) = repo.head()?.target() {
            let commit = repo.find_commit(oid)?;
            // If force is true and a reference already exists with the given name, it'll be replaced.
            // Don't replace the reference already exists in this case.
            repo.branch(branch, &commit, false)?;
        }

        Ok(())
    }

    pub fn config(&self, name: &str, value: &str) -> Result<()> {
        let mut conf = self.0.config()?;
        conf.set_str(name, value)?;

        Ok(())
    }

    pub fn get_config(&self, name: &str) -> Result<String> {
        let conf = self.0.config()?;

        Ok(conf.get_string(name)?)
    }

    pub fn get_workdir(&self) -> Result<&Path> {
        // Don't care about bare repo
        Ok(self.0.workdir().unwrap())
    }

    fn signature_allow_undefined_name(
        &self
    ) -> std::result::Result<Signature<'_>, git2::Error> {
        let repo = &self.0;
        match repo.signature() {
            Err(e) if e.code() == ErrorCode::NotFound => {
                let config = repo.config()?;
                Signature::now(
                    config.get_str("user.name").unwrap_or("unknown"),
                    config.get_str("user.email")?,
                )
            }

            v => v,
        }
    }

    pub fn commit(&self, msg: &str) -> Result<()> {
        let repo = &self.0;
        let signature = self.signature_allow_undefined_name()?;
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let parents = if let Ok(id) = get_head(repo) {
            vec![repo.find_commit(id)?]
        } else {
            Vec::new()
        };

        let parents = parents.iter().collect::<Vec<_>>();

        repo.commit(Some("HEAD"), &signature, &signature, msg, &tree, parents.as_slice())?;

        Ok(())
    }

    pub fn tag(
        &self,
        oid: Oid,
        tag: &str,
    ) -> Result<()> {
        let repo = &self.0;
        let signature = self.signature_allow_undefined_name()?;
        let target = repo.find_object(oid, Some(ObjectType::Commit))?;

        repo.tag(tag, &target, &signature, "", false)?;

        Ok(())
    }

    pub fn fetch(
        &self,
        remote_name: &str,
        refs: &[&str]
    ) -> Result<()> {
        let repo = &self.0;
        let mut remote = repo.find_remote(remote_name)?;
        let mut cb = RemoteCallbacks::new();
        cb.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                print!(
                    "Resolving deltas {}/{}\r",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                    );
            } else if stats.total_objects() > 0 {
                print!(
                    "Received {}/{} objects ({}) in {} bytes\r",
                    stats.received_objects(),
                    stats.total_objects(),
                    stats.indexed_objects(),
                    stats.received_bytes()
                    );
            }
            stdout().flush().unwrap();
            true
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        // It will also fetch all tags.
        fo.download_tags(AutotagOption::All);

        remote.fetch(refs, Some(&mut fo), None)?;

        let stats = remote.stats();
        if stats.local_objects() > 0 {
            println!(
                "\rReceived {}/{} objects in {} bytes (used {} local objects)",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes(),
                stats.local_objects()
            );
        } else {
            println!(
                "\rReceived {}/{} objects in {} bytes",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes()
            );
        }

        Ok(())
    }

    fn normal_merge(
        &self,
        commit_a: &AnnotatedCommit,
        commit_b: &AnnotatedCommit,
        msg: &str
    ) -> Result<()> {
        let repo = &self.0;
        let tree_a = repo.find_commit(commit_a.id())?.tree()?;
        let tree_b = repo.find_commit(commit_b.id())?.tree()?;
        let ancestor = repo
            .find_commit(repo.merge_base(commit_a.id(), commit_b.id())?)?
            .tree()?;
        let mut idx = repo.merge_trees(&ancestor, &tree_a, &tree_b, None)?;

        if idx.has_conflicts() {
            println!("Merge conflicts detected...");
            repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }

        let tree = repo.find_tree(idx.write_tree_to(repo)?)?;
        let signature = repo.signature()?;
        let commit_a_bar = repo.find_commit(commit_a.id())?;
        let commit_b_bar = repo.find_commit(commit_b.id())?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &msg,
            &tree,
            &[&commit_a_bar, &commit_b_bar],
        )?;

        Ok(())
    }

    fn fast_forward_merge(
        &self,
        reference: &mut Reference,
        commit: &AnnotatedCommit,
    ) -> Result<()> {
        let repo = &self.0;
        let name = match reference.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(reference.name_bytes()).to_string(),
        };
        let msg = format!("Fast-Forward: Setting {} to id: {}", name, commit.id());
        reference.set_target(commit.id(), &msg)?;
        repo.set_head(&name)?;
        repo.checkout_head(Some(
                CheckoutBuilder::default()
                // TODO add some logic to handle dirty working directory states
                .force()))?;

        Ok(())
    }

    pub fn merge(
        &self,
        branch: &str,
        commit: AnnotatedCommit,
        msg: &str
    ) -> Result<()> {
        let repo = &self.0;
        let analysis = repo.merge_analysis(&[&commit])?;

        if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch);
            match repo.find_reference(&refname) {
                Ok(mut r) => {
                    self.fast_forward_merge(&mut r, &commit)?;
                },
                Err(_) => {
                    repo.reference(
                        &refname,
                        commit.id(),
                        true,
                        &format!("Setting {} to {}", branch, commit.id()),
                    )?;
                    repo.set_head(&refname)?;
                    repo.checkout_head(Some(
                            CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
            self.normal_merge(&head_commit, &commit, msg)?;
        } else {
            println!("No merge to do...");
        }

        Ok(())
    }

    pub fn delete_branch(&self, branch: &str) -> Result<()> {
        self.0.find_branch(branch, BranchType::Local)?.delete()?;

        Ok(())
    }

    pub fn pull(&self, remote_name: &str, branch: &str) -> Result<()> {
        let repo = &self.0;

        self.fetch(&remote_name, &[&branch])?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        self.merge(&branch, fetch_commit, &format!("Pull {} from {} and merge", branch, remote_name))?;

        Ok(())
    }

    fn push(&self, remote_name: &str, branch: &str, user: &str, pass: &str) -> Result<()> {
        let repo = &self.0;
        let mut remote = repo.find_remote(remote_name)?;
        let mut callbacks = RemoteCallbacks::new();
        let mut opts = PushOptions::new();

        // TODO too simple to get credentials
        callbacks.credentials(|_url, _usernaem_from_url, _allowed_types| {
            Cred::userpass_plaintext(user, pass)
        });
        opts.remote_callbacks(callbacks);

        let refname = format!("refs/heads/{}", branch);
        remote.push(&[&refname], Some(&mut opts))?;

        Ok(())
    }

    fn rebase(&self, branch_from: &str, branch_to: &str) -> Result<()> {
        let repo = &self.0;
        let head = if let Some(head) = repo.head()?.target() {
            head
        } else {
            return Err(Error::NoHead);
        };
        let tid = repo.find_commit(head)?;
        let signature = tid.author();

        let mut opts = RebaseOptions::default();
        let refname_from = format!("refs/heads/{}", branch_from);
        let reference_from = repo.find_reference(&refname_from)?;
        let commit_from = repo.reference_to_annotated_commit(&reference_from)?;

        let refname_to = format!("refs/heads/{}", branch_to);
        let reference_to = repo.find_reference(&refname_to)?;
        let commit_to = repo.reference_to_annotated_commit(&reference_to)?;

        let mut rebase = repo.rebase(
            Some(&commit_from),
            Some(&commit_to),
            None,
            Some(&mut opts))?;

        while let Some(_operation) = rebase.next() {
            let _operation = _operation?;
            println!("Operation: {:?}", _operation.kind());
            rebase.commit(None, &signature, None)?;
        }

        rebase.finish(None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        utils::{
            is_repo,
            get_branch_name,
        },
        error::{
            Error,
            Result,
        },
    };
    use std::path::PathBuf;
    use super::{
        GitcRepo,
    };
    use std::fs::{
        remove_dir_all,
    };

    fn set_test_repo(path: &str) -> Result<GitcRepo> {
        let mut p = PathBuf::with_capacity(40);
        p.push(path);
        let repo = GitcRepo::new(&p);
        Ok(repo)
    }

    fn test_init() {
        let repo = set_test_repo("/tmp/abc");
        match repo {
            Ok(repo) => {
                match repo.init() {
                    Ok(()) => {
                        assert_eq!(
                            get_branch_name(&repo.0).unwrap().as_str(),
                            "master"
                        )
                    },
                    Err(e) => {
                        eprintln!("failed to init {}", e);
                        remove_dir_all(&"/tmp/abc");
                        assert!(false);
                    },
                }
            },
            Err(e) => {
                eprintln!("encounter {}", e);
                remove_dir_all(&"/tmp/abc");
                assert!(false);
            },
        }
    }
    fn test_commit() {
        todo!()
    }
    fn test_checkout() {
        let repo = set_test_repo("/tmp/abc");
        match repo {
            Ok(repo) => {
                match repo.checkout("develop") {
                    Ok(()) => {
                        assert_eq!(
                            get_branch_name(&repo.0).unwrap().as_str(),
                            "develop"
                        )
                    },
                    Err(e) => {
                        eprintln!("checkout develop {}", e);
                        assert!(false);
                    },
                }
            },
            Err(e) => {
                eprintln!("set_test_repo {}", e);
                assert!(false);
            }
        }
    }
    fn test_merge() {
        todo!()
    }
    fn test_config() {
        todo!()
    }
    fn test_fetch() {
        todo!()
    }
    fn test_pull() {
        todo!()
    }
    fn test_push() {

    }
    fn test_branch() {
        let repo = set_test_repo("/tmp/abc");
        match repo {
            Ok(repo) => {
                match repo.branch("develop") {
                    Ok(_) => assert!(true),
                    Err(e) => {
                        eprintln!("create branch develop {}", e);
                        assert!(false);
                    },
                }
            },
            Err(e) => {
                eprintln!("set_test_repo {}", e);
                assert!(false);
            }
        }
    }
    fn test_rebase() {
        todo!()
    }

    #[test]
    fn test_is_repo() {
        let repo = set_test_repo("/tmp/abc");
        let repo = repo.unwrap().0;
        assert!(repo.is_empty().unwrap());
    }

    #[test]
    fn test_flow() {
        test_init();
        test_branch();
        test_checkout();
    }
}
