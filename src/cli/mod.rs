mod text;
mod jwt;

use std::path::Path;

use clap::{ Parser, Subcommand};
use enum_dispatch::enum_dispatch;

pub use self::{text::*, jwt::*};

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
    Jwt(JwtSubCommand)
}

/// Verify if the filename input is exists.
pub fn verify_file (filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

