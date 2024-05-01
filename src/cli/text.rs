use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{get_reader, CmdExecutor};

use super::verify_file;

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    ChaCha20Poly1305,
};

// 阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密 / 解密
// 要求：
// rcli text encrypt -key"xxx"> 加密并输出 base64
// rcli text decrypt -key"XXX" >base64 > binary> 解密文本
// 
// cargo run encrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_encrypt.txt
// cargo run text decrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_decrypt.txt

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
    #[arg(short, long, value_parser = verify_length)]
    key: String,
}

/// 秘钥必须32字节长
pub fn verify_length(key : &str) -> Result<String, &'static str> {
    if key.len() == 32 {
        Ok(key.into())
    } else {
        Err("Key must be 32 characters length.")
    }
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
        // 32字节的秘钥
        let key = self.key.as_bytes();
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(&key));
        // 12字节的随机数
        let nonce = &key[..12];
        let nonce = GenericArray::from_slice(&nonce);

        let mut reader = get_reader(&self.input)?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let ciphertext = cipher.encrypt(&nonce, buf.as_ref())?;

        let encrypted_base64 = URL_SAFE_NO_PAD.encode(ciphertext);
        println!("{}", encrypted_base64);

        Ok(())
    }
}

impl CmdExecutor for TextDecryptopts {
    async fn execute(self) -> anyhow::Result<()> {
        // 32字节的秘钥
        let key = self.key.as_bytes();
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(&key));

        // read from input
        let mut reader = get_reader(&self.input)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        // decode base64
        let buf = URL_SAFE_NO_PAD.decode(buf)?;

        // 12字节的随机数
        let nonce = &key[..12];
        let nonce = GenericArray::from_slice(&nonce);

        let plaintext = cipher.decrypt(nonce, buf.as_ref())?;
        println!("{}", String::from_utf8(plaintext)?);
        Ok(())
    }
}
