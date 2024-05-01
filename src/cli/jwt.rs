// json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。

// CLI：

// rcli jwt sign --sub acme --aud device1 --exp 14d
// rcli jwt verify -t

// cargo run jwt sign -s Goo -a www.google.com -e 1714593571436
// cargo run jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJHb28iLCJhdWQiOiJ3d3cuZ29vZ2xlLmNvbSIsImV4cCI6MTcxNDU5MzU3MTQzNn0.CRIoZuLnQ6JUJ9nwHfATCd6PQxVxp4ZWoy32fdoct3VNx9CpIUpKHQIVb8EGfYn0FREy6axobVbmFzC7uBMvRA -a www.google.com

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::CmdExecutor;

/// Subcommand of Text Command
#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    Sign(TextJwtSignOpts),
    Verify(TextJwtVerifyopts),
}

const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Parser, Serialize)]
pub struct TextJwtSignOpts {
    #[arg(short, long)]
    sub: String,
    #[arg(short, long)]
    aud: String,
    #[arg(short, long)]
    exp: u64,
}

#[derive(Debug, Parser)]
pub struct TextJwtVerifyopts {
    #[arg(short, long)]
    token: String,
    #[arg(short, long)]
    aud: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

impl CmdExecutor for TextJwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 构造JWT参数
        let header = Header::new(Algorithm::HS512);
        let claims = Claims {
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
        };
        // JWT加密并签名
        let jwt = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))?;
        println!("{}", jwt);
        Ok(())
    }
}

impl CmdExecutor for TextJwtVerifyopts {
    async fn execute(self) -> anyhow::Result<()> {
        // 设置JWT校验参数
        let mut validation = Validation::new(Algorithm::HS512);
        validation.set_audience(&[self.aud]);
        // JWT串解密并验证
        let decoded = decode::<Claims>(
            &self.token,
            &DecodingKey::from_secret(JWT_SECRET),
            &validation,
        )?;
        println!("{:?}", decoded.claims);
        Ok(())
    }
}
