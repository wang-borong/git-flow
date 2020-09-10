use std::path::PathBuf;
use super::{
    gitc::GitcRepo,
    utils::get_user_input,
    error::{
        Error,
        Result,
    },
};

use clap::ArgMatches;

// also use it as the first command
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GFBranch {
    Feature,
    Bugfix,
    Hotfix,
    Release,
    Support,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GFSubCmd {
    Init,
    Start,
    Finish,
}

pub struct GFWork {
    pub repo: GitcRepo,
    pub branch_suffix: String, // passed by user
}

impl GFWork {
    pub fn new(p: &PathBuf) -> Self {
        Self {
            repo: GitcRepo::new(p),
            branch_suffix: String::with_capacity(10),
        }
    }

    fn get_branch_prefix(&self, cmd: GFBranch) -> Result<String> {
        match cmd {
            GFBranch::Feature => {
                Ok(self.repo.get_config("gitflow.prefix.feature")?)
            }
            GFBranch::Bugfix => {
                Ok(self.repo.get_config("gitflow.prefix.bugfix")?)
            }
            GFBranch::Hotfix => {
                Ok(self.repo.get_config("gitflow.prefix.hotfix")?)
            }
            GFBranch::Release => {
                Ok(self.repo.get_config("gitflow.prefix.release")?)
            }
            GFBranch::Support => {
                Ok(self.repo.get_config("gitflow.prefix.support")?)
            }
        }
    }

    pub fn set_branch_suffix(&mut self, bs: &str) {
        self.branch_suffix = bs.to_string();
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

        println!("\nHow to name your supporting branch prefixes?");
        self.subconfig("Feature branches? [feature/]", "gitflow.prefix.feature", "feature/")?;
        self.subconfig("Bugfix branches? [bugfix/]", "gitflow.prefix.bugfix", "bugfix/")?;
        self.subconfig("Release branches? [release/]", "gitflow.prefix.release", "release/")?;
        self.subconfig("Hotfix branches? [hotfix/]", "gitflow.prefix.hotfix", "hotfix/")?;
        self.subconfig("Support branches? [support/]", "gitflow.prefix.support", "support/")?;

        self.subconfig("Version tag prefix? []", "gitflow.prefix.versiontag", "")?;

        let repodir = self.repo.get_workdir()?;
        let hooksdir = format!("{}.git/hooks", repodir.to_str().unwrap());
        self.subconfig(&format!("Hooks and filters directory? [{}]", hooksdir), "gitflow.path.hooks", &hooksdir)?;

        Ok(())
    }

    fn cat_gfbranch(&self, cmd: GFBranch) -> Result<String> {
        let branch_prefix = self.get_branch_prefix(cmd)?;
        Ok(format!("{}{}", &branch_prefix, &self.branch_suffix))
    }

    pub fn init(&self) -> Result<()> {
        self.repo.init()?;
        self.config()?;
        // get develop branch name and create it
        let branch = self.repo.get_config("gitflow.branch.develop")?;
        self.repo.branch(&branch)?;
        // then checkout it
        self.repo.checkout(&branch)?;

        Ok(())
    }

    pub fn start(&self, cmd:GFBranch, fetch: bool) -> Result<()> {
        let branch = self.cat_gfbranch(cmd)?;
        // create a new branch
        self.repo.branch(&branch)?;
        // and checkout it
        self.repo.checkout(&branch)?;

        Ok(())
    }

    // Perhaps parse optional arguments here is ugly
    pub fn finish(&self, cmd: GFBranch, opts_args: &ArgMatches) -> Result<()> {
        let branch = self.cat_gfbranch(cmd)?;
        // checkout to related branch
        // and merge
        match cmd {
            GFBranch::Release | GFBranch::Hotfix => {
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

#[cfg(test)]
mod tests {
}
