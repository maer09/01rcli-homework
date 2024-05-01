use clap::Parser;
use rcli::{Cli, CmdExecutor};

// 作业1 chacha20加解密示例：
// cargo run encrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_encrypt.txt
// cargo run text decrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_decrypt.txt

// 作业2 jwt签名验签示例：
// cargo run jwt sign -s Goo -a www.google.com -e 1714593571436
// cargo run jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJHb28iLCJhdWQiOiJ3d3cuZ29vZ2xlLmNvbSIsImV4cCI6MTcxNDU5MzU3MTQzNn0.CRIoZuLnQ6JUJ9nwHfATCd6PQxVxp4ZWoy32fdoct3VNx9CpIUpKHQIVb8EGfYn0FREy6axobVbmFzC7uBMvRA -a www.google.com


/// Main Function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Cli::parse();
    opts.cmd.execute().await?;

    Ok(())
}
