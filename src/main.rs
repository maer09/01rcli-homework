use clap::Parser;
use rcli::{Cli, CmdExecutor};

// windows下启动
// $env:RUST_LOG='info'; cargo run http serve

// 作业1 chacha20加解密示例：
// cargo run encrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_encrypt.txt
// cargo run text decrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_decrypt.txt

// 作业2 jwt签名验签示例：
// cargo run jwt sign -s Goo -a www.google.com -e 1714593571436
// cargo run jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJHb28iLCJhdWQiOiJ3d3cuZ29vZ2xlLmNvbSIsImV4cCI6MTcxNDU5MzU3MTQzNn0.CRIoZuLnQ6JUJ9nwHfATCd6PQxVxp4ZWoy32fdoct3VNx9CpIUpKHQIVb8EGfYn0FREy6axobVbmFzC7uBMvRA -a www.google.com

// 作业3 http访问文件夹
// $env:RUST_LOG='info'; cargo run http serve
// 访问：http://localhost:8080/fixtures，然后继续点击链接


/// Main Function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Cli::parse();
    opts.cmd.execute().await?;

    Ok(())
}
