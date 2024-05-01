use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

use super::verify_file;

// 阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密 / 解密
// 要求：

// rcli text encrypt -key"xxx"> 加密并输出 base64
// rcli text decrypt -key"XXX" >base64 > binary> 解密文本

/// Subcommand of Text Command
#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    
    Encrypt(TextEncryptOpts),
    Decrypt(TextDecryptopts),
}

/// Text Encrypt Options
#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file)]
    input: String,
    #[arg(short, long)]
    key: String,
}

/// Text Decrypt Options
#[derive(Debug, Parser)]
pub struct TextDecryptopts {
    #[arg(short, long, value_parser = verify_file)]
    input: String,
    #[arg(short, long)]
    key: String,
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Execute encrypt.");
        Ok(())
    }
}

impl CmdExecutor for TextDecryptopts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Execute decrypt.");
        Ok(())
    }
}
