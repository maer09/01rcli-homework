use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use std::{fs::read_dir, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

use crate::{verify_path, CmdExecutor};

/// Subcommand of Text Command
#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, &self.port).await?;
        Ok(())
    }
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: &u16) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    info!("Serving {:?} on {}", &path, addr);
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Html<String>) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file{:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            Html(format!("File {} not found", p.display())),
        );
    } else {
        // 处理文件夹时的情况
        if p.is_dir() {
            return match serve_dir(p) {
                Ok(content) => (StatusCode::OK, Html(content)),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
            };
        }

        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(err) => {
                warn!("Error reading file: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string()))
            }
        }
    }
}

fn serve_dir(path: PathBuf) -> anyhow::Result<String> {
    let link_list = read_dir(path.clone())?
        .map(|entry| {
            let filename = entry.unwrap().file_name().into_string().unwrap();
            let link = path.file_name().unwrap().to_str().unwrap();
            let link = format!("{}/{}", link, filename);
            format!("<li><a href=\"{}\"/>{}</li>", link, filename)
        })
        .reduce(|a, b| a + &b);

    Ok(format!(
        "<!DOCTYPE html> <html> <head> <meta charset=\"utf-8\"></head><body><ul>{}<ul></body></html>",
        link_list.unwrap_or("".into())
    ))
}