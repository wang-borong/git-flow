use git2::{
    Repository,
    RepositoryOpenFlags,
    Oid,
};
use std::{
    io::{stdin, stdout, Write},
    string::String,
    path::Path,
};
use crate::{
    error:: {
        Error,
        Result,
    }
};

pub fn is_repo(repo_path: &str) -> bool {
    Repository::open_ext(
        repo_path,
        RepositoryOpenFlags::empty(),
        Vec::<&Path>::new(),
    )
    .is_ok()
}

pub fn get_branch_name(repo: &Repository) -> Result<String> {
    let branch_iter = repo.branches(None)?;

    for b in branch_iter {
        let b = b?;

        if b.0.is_head() {
            let name = b.0.name()?.unwrap_or("");
            return Ok(name.into());
        }
    }

    Err(Error::NoHead)
}

pub fn get_head(repo: &Repository) -> Result<Oid> {
    let head = repo.head()?.target();
    if let Some(head_id) = head {
        Ok(head_id)
    } else {
        Err(Error::NoHead)
    }
}

pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    stdout().flush()?;
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {},
    }

    Ok(input.trim().to_string())
}
