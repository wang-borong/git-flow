extern crate clap;

pub mod gitflow;

use std::path::PathBuf;

use clap::{
    Arg,
    App,
    SubCommand,
    crate_authors,
    crate_version,
};

use gitflow::{
    utils::is_repo,
    error::{
        Error,
        Result,
    },
    gf::{
        GFBranch,
        GFSubCmd,
        GFWork,
    },
};

pub fn cli_run() -> Result<String> {
    let matches = App::new("git-flow")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Workflow in git")
        // Init subcommand
        .subcommand(SubCommand::with_name("init")
            .about("Initialize a new git repo with support for the branching model")
            .arg(Arg::with_name("init_path")
                .help("The path to be initialized")))
        // Config subcommand
        .subcommand(SubCommand::with_name("config")
            .about("Manage your git-flow configuration"))
        // Feature subcommand
        .subcommand(SubCommand::with_name("feature")
            .about("Manage your feature branches")
            .subcommand(SubCommand::with_name("start")
                .about("Start new feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The new feature name to be started")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("Finish feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature name to be finished")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("Lists all the existing feature branches in the local repository"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish feature branch on origin")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be published")))
            .subcommand(SubCommand::with_name("track")
                .about("Start tracking feature that is shared on origin")
                .arg(Arg::with_name("feature_name")
                    .help("The feature branch to be tracked")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("diff")
                .about("Show all changes in feature branch that are not in the base branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be checked")))
            .subcommand(SubCommand::with_name("rebase")
                .about("Rebase feature branch on base branch")
                .arg(Arg::with_name("interactive")
                    .short("i")
                    .help("Do an interactive rebase"))
                .arg(Arg::with_name("rebase-merges")
                    .short("r")
                    .help("Preserve merges"))
                .arg(Arg::with_name("feature_name")
                    .help("The feature branch to be rebased")
                    .index(1)))
            .subcommand(SubCommand::with_name("checkout")
                .about("Switch to feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature name to be checked out")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("delete")
                .about("Delete a given feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature branch to be deleted")
                    .required(true)
                    .index(1)))
        )
        // Release subcommand
        .subcommand(SubCommand::with_name("release")
            .about("Manage your release branches")
            .subcommand(SubCommand::with_name("start")
                .about("Start new release branch")
                .arg(Arg::with_name("release_name")
                    .help("The new release branch to be started")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("Finish release branch")
                .arg(Arg::with_name("release_name")
                    .help("The release branch to be finished")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("List all the existing release branches in local repository"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish release branch on origin")
                .arg(Arg::with_name("release_name")
                    .help("The release to be published")))
            .subcommand(SubCommand::with_name("track")
                .about("Start tracking release that is shared on origin")
                .arg(Arg::with_name("release_name")
                    .help("The release branch to be tracked")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("delete")
                .about("Delete a given release branch")
                .arg(Arg::with_name("release_name")
                    .help("The release branch to be deleted")
                    .required(true)
                    .index(1)))
        )
        // Hotfix subcommand
        .subcommand(SubCommand::with_name("hotfix")
            .about("Manage your hotfix branches")
            .subcommand(SubCommand::with_name("start")
                .about("Start new hotfix branch")
                .arg(Arg::with_name("hotfix_name")
                    .help("The new hotfix name to be started")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("Finish hotfix branch")
                .arg(Arg::with_name("hotfix_name")
                    .help("The hotfix name to be finished")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("List all the existing hotfix branches in local repository"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish hotfix branch on origin")
                .arg(Arg::with_name("hotfix_name")
                    .help("The feature to be published")))
            .subcommand(SubCommand::with_name("delete")
                .about("Delete a given hotfix branch")
                .arg(Arg::with_name("hotfix_name")
                    .help("The hotfix branch to be deleted")
                    .required(true)
                    .index(1)))
        )
        .subcommand(SubCommand::with_name("bugfix")
            .about("Manage your bugfix branches")
            .subcommand(SubCommand::with_name("start")
                .about("Start new bugfix branch")
                .arg(Arg::with_name("bugfix_name")
                    .help("The new bugfix name to be started")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("Finish bugfix branch")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix branch to be finished")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("List all the existing bugfix branches in local repository"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish bugfix branch on origin")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix to be published")))
            .subcommand(SubCommand::with_name("track")
                .about("Start tracking bugfix that is shared on origin")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix branch to be tracked")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("diff")
                .about("Show all changes in bugfix branch that are not in the base branch")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix to be checked")))
            .subcommand(SubCommand::with_name("rebase")
                .about("Rebase bugfix on base branch")
                .arg(Arg::with_name("interactive")
                    .short("i")
                    .help("Do an interactive rebase"))
                .arg(Arg::with_name("rebase-merges")
                    .short("r")
                    .help("Preserve merges"))
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix branch to be rebased")
                    .index(1)))
            .subcommand(SubCommand::with_name("checkout")
                .about("Switch to bugfix branch")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix name to be checked out")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("delete")
                .about("Delete a given bugfix branch")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix branch to be deleted")
                    .required(true)
                    .index(1)))
        )
        .subcommand(SubCommand::with_name("support")
            .about("Manage your support branches")
            .subcommand(SubCommand::with_name("start")
                .about("Start new support branch")
                .arg(Arg::with_name("support_name")
                    .help("The new support name to be started")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("base_branch")
                    .help("The based branch which a support starts from")
                    .required(true)
                    .index(2)))
            .subcommand(SubCommand::with_name("list")
                .about("List all the existing support branches in local repository"))
        )
        // ...
        .get_matches();

    // Init
    if let Some(matches) = matches.subcommand_matches("init") {
        let path = matches.value_of("init_path").unwrap_or(".");
        let mut gfwork = GFWork::new(&PathBuf::from(path));

        gfwork.set_subcmd(GFSubCmd::Init);
        gfwork.work()?;

        return Ok(format!("init {} success", path));
    }

    if !is_repo(".") {
        return Err(Error::Generic(
                "This is not a repo, use git-flow init ot initialize it".to_string()));
    }

    let mut gfwork = GFWork::new(&PathBuf::from("."));

    match matches.subcommand() {
        ("feature", feature_matches) => {
            // set command
            gfwork.set_cmd(GFBranch::Feature);
            // set subcommand
            match feature_matches.unwrap().subcommand() {
                ("start", feature_start_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &feature_start_matches.unwrap()
                        .value_of("feature_name").unwrap());
                }
                ("finish", feature_finish_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Finish);
                    gfwork.set_branch_suffix(
                        &feature_finish_matches.unwrap()
                        .value_of("feature_name").unwrap());
                }
                // ...
                _ => {}
            }

        }
        ("release", release_matches) => {
            gfwork.set_cmd(GFBranch::Release);
            match release_matches.unwrap().subcommand() {
                ("start", release_start_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &release_start_matches.unwrap()
                        .value_of("release_name").unwrap());
                }
                ("finish", release_finish_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Finish);
                    gfwork.set_branch_suffix(
                        &release_finish_matches.unwrap()
                        .value_of("release_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("bugfix", bugfix_matches) => {
            gfwork.set_cmd(GFBranch::Bugfix);
            match bugfix_matches.unwrap().subcommand() {
                ("start", bugfix_start_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &bugfix_start_matches.unwrap()
                        .value_of("bugfix_name").unwrap());
                }
                ("finish", bugfix_finish_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Finish);
                    gfwork.set_branch_suffix(
                        &bugfix_finish_matches.unwrap()
                        .value_of("bugfix_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("hotfix", hotfix_matches) => {
            gfwork.set_cmd(GFBranch::Hotfix);
            match hotfix_matches.unwrap().subcommand() {
                ("start", hotfix_start_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &hotfix_start_matches.unwrap()
                        .value_of("hotfix_name").unwrap());
                }
                ("finish", hotfix_finish_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Finish);
                    gfwork.set_branch_suffix(
                        &hotfix_finish_matches.unwrap()
                        .value_of("hotfix_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("support", support_matches) => {
            gfwork.set_cmd(GFBranch::Support);
            match support_matches.unwrap().subcommand() {
                ("start", support_start_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &support_start_matches.unwrap()
                        .value_of("support_name").unwrap());
                }
                ("finish", support_finish_matches) => {
                    gfwork.set_subcmd(GFSubCmd::Finish);
                    gfwork.set_branch_suffix(
                        &support_finish_matches.unwrap()
                        .value_of("support_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        // ...
        _ => {
        }
    }

    gfwork.work()?;

    Ok(format!("run ... success"))

}
