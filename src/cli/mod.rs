mod text;
mod jwt;
mod http;

use std::path::{Path, PathBuf};

use clap::{ Parser, Subcommand};
use enum_dispatch::enum_dispatch;

pub use self::{text::*, jwt::*, http::*};

/// Command Set
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum Commands {
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

/// Verify if the filename input is exists.
pub fn verify_file (filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_path(filename: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(filename);
    if p.exists() && p.is_dir() {
        Ok(filename.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

