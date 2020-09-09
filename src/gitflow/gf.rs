use std::path::PathBuf;
use super::{
    gitc::GitcRepo,
    utils::get_user_input,
    error::{
        Error,
        Result,
    },
};

// also use it as the first command
#[derive(Clone, Copy, PartialEq)]
pub enum GfBranch {
    Feature,
    Bugfix,
    Hotfix,
    Release,
    Support,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GfCmds {
    Init,
    Start,
    Finish,
}

pub struct GfWork {
    pub cmd: Option<GfBranch>,
    pub subcmd: Option<GfCmds>,
    pub repo: GitcRepo,
    pub branch_suffix: String, // passed by user
}

impl GfWork {
    pub fn new(p: &PathBuf) -> Self {
        Self {
            cmd: None,
            subcmd: None,
            repo: GitcRepo::new(p),
            branch_suffix: String::with_capacity(10),
        }
    }

    fn get_branch_prefix(&self) -> Result<String> {
        if let Some(bp) = self.cmd {
            match bp {
                GfBranch::Feature => {
                    Ok(self.repo.get_config("gitflow.prefix.feature")?)
                }
                GfBranch::Bugfix => {
                    Ok(self.repo.get_config("gitflow.prefix.bugfix")?)
                }
                GfBranch::Hotfix => {
                    Ok(self.repo.get_config("gitflow.prefix.hotfix")?)
                }
                GfBranch::Release => {
                    Ok(self.repo.get_config("gitflow.prefix.release")?)
                }
                GfBranch::Support => {
                    Ok(self.repo.get_config("gitflow.prefix.support")?)
                }
            }
        } else {
            Err(Error::Generic("No cmd set to get branch prefix, set cmd firstly.".to_string()))
        }

    }

    pub fn set_branch_suffix(&mut self, bs: &str) {
        self.branch_suffix = bs.to_string();
    }

    pub fn set_cmd(&mut self, cmd: GfBranch) {
        self.cmd = Some(cmd);
    }

    pub fn set_subcmd(&mut self, subcmd: GfCmds) {
        self.subcmd = Some(subcmd);
    }

    fn subconfig(&self, prompt: &str, name: &str, default: &str) -> Result<()> {
        let mut s = get_user_input(prompt)?;
        if s.is_empty() {
            s.push_str(default);
        }
        self.repo.config(name, &s)?;

        Ok(())
    }

    fn config(&self) -> Result<()> {
        self.subconfig("Branch name for production releases: [master] ", "gitflow.branch.master", "master")?;
        self.subconfig("Branch name for \"next release\" development: [develop] ", "gitflow.branch.develop", "develop")?;

        println!("How to name your supporting branch prefixes?");
        self.subconfig("Feature branches? [feature/]", "gitflow.prefix.feature", "feature/")?;
        self.subconfig("Bugfix branches? [bugfix/]", "gitflow.prefix.bugfix", "bugfix/")?;
        self.subconfig("Release branches? [release/]", "gitflow.prefix.release", "release/")?;
        self.subconfig("Hotfix branches? [hotfix/]", "gitflow.prefix.hotfix", "hotfix/")?;
        self.subconfig("Support branches? [support/]", "gitflow.prefix.support", "support/")?;

        self.subconfig("Version tag prefix? []", "gitflow.prefix.versiontag", "")?;

        let repodir = self.repo.get_workdir()?;
        let hooksdir = format!("{}/.git/hooks", repodir.to_str().unwrap());
        self.subconfig(&format!("Hooks and filters directory? [{}]", hooksdir), "gitflow.path.hooks", &hooksdir)?;

        Ok(())
    }

    fn cat_gfbranch(&self) -> Result<String> {
        let branch_prefix = self.get_branch_prefix()?;
        Ok(format!("{}{}", &branch_prefix, &self.branch_suffix))
    }

    // The main api to do git-flow works
    pub fn work(&self) -> Result<()> {
        if self.subcmd.is_none() {
            return Err(Error::Generic(format!("No subcommand supplied to work")));
        }
        if self.cmd.is_none() && self.subcmd.unwrap() != GfCmds::Init {
            return Err(Error::Generic(format!("No branch_prefix supplied to work")));
        }
        match self.subcmd.unwrap() {
            GfCmds::Init => {
                self.repo.init()?;
                self.config()?;
                Ok(())
            }
            GfCmds::Start => {
                let branch = self.cat_gfbranch()?;
                // create a new branch
                self.repo.branch(&branch)?;
                // and checkout it
                self.repo.checkout(&branch)?;
                Ok(())
            }
            GfCmds::Finish => {
                let branch = self.cat_gfbranch()?;
                // checkout to related branch
                // and merge
                match self.cmd.unwrap() {
                    GfBranch::Release | GfBranch::Hotfix => {
                        self.repo.checkout(&self.repo.get_config("gitflow.branch.master")?)?;
                        let refname = format!("refs/heads/{}", &branch);
                        let branch_ref = self.repo.0.find_reference(&refname)?;
                        self.repo.merge(
                            &self.repo.get_config("gitflow.branch.master")?,
                            self.repo.0.reference_to_annotated_commit(&branch_ref)?,
                            // TODO (too simple to get the merge message)
                            &get_user_input("Input your merge message: ")?
                        )?;

                        // Give some tag message
                        self.repo.tag(
                            self.repo.0.refname_to_id(&format!("refs/heads/{}",
                                    &self.repo.get_config("gitflow.branch.master")?))?,
                            &get_user_input("Input a tag name")?,
                        )?;

                        self.repo.checkout(&self.repo.get_config("gitflow.branch.develop")?)?;
                        let refname = format!("refs/heads/{}",
                            &self.repo.get_config("gitflow.branch.master")?);
                        let branch_ref = self.repo.0.find_reference(&refname)?;
                        self.repo.merge(
                            &self.repo.get_config("gitflow.branch.develop")?,
                            self.repo.0.reference_to_annotated_commit(&branch_ref)?,
                            &get_user_input("Input your merge message: ")?
                        )?;
                    },
                    _ => {
                        self.repo.checkout(&self.repo.get_config("gitflow.branch.develop")?)?;
                        let refname = format!("refs/heads/{}", &branch);
                        let branch_ref = self.repo.0.find_reference(&refname)?;
                        self.repo.merge(
                            &self.repo.get_config("gitflow.branch.develop")?,
                            self.repo.0.reference_to_annotated_commit(&branch_ref)?,
                            &get_user_input("Input your merge message: ")?
                        )?;
                    }
                }
                // then delete the git-flow branch
                self.repo.delete_branch(&branch)?;

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
