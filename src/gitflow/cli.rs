extern crate clap;

use std::path::PathBuf;
use clap::{
    Arg,
    App,
    SubCommand,
};

use super::{
    utils::is_repo,
    error::{
        Error,
        Result,
    },
    gf::{
        GfBranch,
        GfCmds,
        GfWork,
    },
};

pub fn cli_run() -> Result<String> {
    let matches = App::new("git-flow")
        .version("0.1.0")
        .author("Jason Wang <wang_borong@163.com>")
        .about("Workflow in git")
        // Init subcommand
        .subcommand(SubCommand::with_name("init")
            .about("Setup a git repository for git flow usage.")
            .arg(Arg::with_name("init_path")
                .help("Path to be initialized")))
        // Config subcommand
        .subcommand(SubCommand::with_name("config")
            .about("Show the git-flow configurations"))
        // Feature subcommand
        .subcommand(SubCommand::with_name("feature")
            .about("Manage your feature branches.")
            .subcommand(SubCommand::with_name("start")
                .about("Start new feature branch.")
                .arg(Arg::with_name("feature_name")
                    .help("The new feature to be started")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("Finish feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be finished")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("Lists all the existing feature branches in the local repository"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish feature branch on origin.")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be published")))
            .subcommand(SubCommand::with_name("track")
                .about("Start tracking feature that is shared on origin")
                .arg(Arg::with_name("feature_name")
                    .help("The feature branch to be tracked")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("diff")
                .about("Show all changes in feature branch that are not in the base branch.")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be checked")))
            .subcommand(SubCommand::with_name("rebase")
                .about("Rebase feature on develop")
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
            .about("git flow release")
            .subcommand(SubCommand::with_name("start")
                .about("release start command")
                .arg(Arg::with_name("release_name")
                    .help("work on a release branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("release finish command")
                .arg(Arg::with_name("release_name")
                    .help("work off a release branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("release list command"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish release branch on origin.")
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
            .about("git flow hotfix")
            .subcommand(SubCommand::with_name("start")
                .about("hotfix start command")
                .arg(Arg::with_name("hotfix_name")
                    .help("work on a hotfix branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("hotfix finish command")
                .arg(Arg::with_name("hotfix_name")
                    .help("work off a hotfix branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("hotfix list command"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish feature branch on origin.")
                .arg(Arg::with_name("feature_name")
                    .help("The feature to be published")))
            .subcommand(SubCommand::with_name("delete")
                .about("Delete a given feature branch")
                .arg(Arg::with_name("feature_name")
                    .help("The feature branch to be deleted")
                    .required(true)
                    .index(1)))
        )
        .subcommand(SubCommand::with_name("bugfix")
            .about("git flow bugfix")
            .subcommand(SubCommand::with_name("start")
                .about("bugfix start command")
                .arg(Arg::with_name("bugfix_name")
                    .help("work on a bugfix branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("finish")
                .about("bugfix finish command")
                .arg(Arg::with_name("bugfix_name")
                    .help("work off a bugfix branch")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("list")
                .about("bugfix list command"))
            .subcommand(SubCommand::with_name("publish")
                .about("Publish bugfix branch on origin.")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix to be published")))
            .subcommand(SubCommand::with_name("track")
                .about("Start tracking bugfix that is shared on origin")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix branch to be tracked")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("diff")
                .about("Show all changes in bugfix branch that are not in the base branch.")
                .arg(Arg::with_name("bugfix_name")
                    .help("The bugfix to be checked")))
            .subcommand(SubCommand::with_name("rebase")
                .about("Rebase bugfix on develop")
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
            .about("git flow support")
            .subcommand(SubCommand::with_name("start")
                .about("support start command")
                .arg(Arg::with_name("support_name")
                    .help("work on a bugfix branch")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("base_branch")
                    .help("the based branch which a support starts from")
                    .required(true)
                    .index(2)))
            .subcommand(SubCommand::with_name("list")
                .about("bugfix list command"))
        )
        // ...
        .get_matches();

    // Init
    if let Some(matches) = matches.subcommand_matches("init") {
        let path = matches.value_of("init_path").unwrap_or(".");
        let mut gfwork = GfWork::new(&PathBuf::from(path));

        gfwork.set_subcmd(GfCmds::Init);
        gfwork.work()?;

        return Ok(format!("init {} success", path));
    }

    if !is_repo(".") {
        return Err(Error::Generic(
                "This is not a repo, use git-flow init ot initialize it".to_string()));
    }

    let mut gfwork = GfWork::new(&PathBuf::from("."));

    match matches.subcommand() {
        ("feature", feature_matches) => {
            // set command
            gfwork.set_cmd(GfBranch::Feature);
            // set subcommand
            match feature_matches.unwrap().subcommand() {
                ("start", feature_start_matches) => {
                    gfwork.set_subcmd(GfCmds::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &feature_start_matches.unwrap()
                        .value_of("feature_name").unwrap());
                }
                ("finish", feature_finish_matches) => {
                    gfwork.set_subcmd(GfCmds::Finish);
                    gfwork.set_branch_suffix(
                        &feature_finish_matches.unwrap()
                        .value_of("feature_name").unwrap());
                }
                // ...
                _ => {}
            }

        }
        ("release", release_matches) => {
            gfwork.set_cmd(GfBranch::Release);
            match release_matches.unwrap().subcommand() {
                ("start", release_start_matches) => {
                    gfwork.set_subcmd(GfCmds::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &release_start_matches.unwrap()
                        .value_of("release_name").unwrap());
                }
                ("finish", release_finish_matches) => {
                    gfwork.set_subcmd(GfCmds::Finish);
                    gfwork.set_branch_suffix(
                        &release_finish_matches.unwrap()
                        .value_of("release_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("bugfix", bugfix_matches) => {
            gfwork.set_cmd(GfBranch::Bugfix);
            match bugfix_matches.unwrap().subcommand() {
                ("start", bugfix_start_matches) => {
                    gfwork.set_subcmd(GfCmds::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &bugfix_start_matches.unwrap()
                        .value_of("bugfix_name").unwrap());
                }
                ("finish", bugfix_finish_matches) => {
                    gfwork.set_subcmd(GfCmds::Finish);
                    gfwork.set_branch_suffix(
                        &bugfix_finish_matches.unwrap()
                        .value_of("bugfix_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("hotfix", hotfix_matches) => {
            gfwork.set_cmd(GfBranch::Hotfix);
            match hotfix_matches.unwrap().subcommand() {
                ("start", hotfix_start_matches) => {
                    gfwork.set_subcmd(GfCmds::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &hotfix_start_matches.unwrap()
                        .value_of("hotfix_name").unwrap());
                }
                ("finish", hotfix_finish_matches) => {
                    gfwork.set_subcmd(GfCmds::Finish);
                    gfwork.set_branch_suffix(
                        &hotfix_finish_matches.unwrap()
                        .value_of("hotfix_name").unwrap());
                }
                // ...
                _ => {}
            }
        }
        ("support", support_matches) => {
            gfwork.set_cmd(GfBranch::Support);
            match support_matches.unwrap().subcommand() {
                ("start", support_start_matches) => {
                    gfwork.set_subcmd(GfCmds::Start);
                    // set branch suffix
                    gfwork.set_branch_suffix(
                        &support_start_matches.unwrap()
                        .value_of("support_name").unwrap());
                }
                ("finish", support_finish_matches) => {
                    gfwork.set_subcmd(GfCmds::Finish);
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
