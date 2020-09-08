extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}`")]
    Generic(String),

    #[error("git: no head found")]
    NoHead,

    #[error("io error:{0}")]
    Io(#[from] std::io::Error),

    #[error("git error:{0}")]
    Git(#[from] git2::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
