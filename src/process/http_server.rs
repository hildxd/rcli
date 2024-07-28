use core::fmt;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;

enum HtmlOrStringResponse {
    Html(Html<String>),
    String(String),
}

impl IntoResponse for HtmlOrStringResponse {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            HtmlOrStringResponse::Html(html) => html.into_response(),
            HtmlOrStringResponse::String(s) => s.into_response(),
        }
    }
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
    addr: SocketAddr,
}

pub async fn process_http_server(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("http server start at: {}", addr);

    let state = HttpServeState {
        path: path.clone(),
        addr,
    };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, HtmlOrStringResponse) {
    let p = std::path::Path::new(&state.path).join(&path);
    match p.exists() {
        false => (
            StatusCode::NOT_FOUND,
            HtmlOrStringResponse::String(format!("{} not found", p.display())),
        ),
        true => {
            if p.is_dir() {
                let mut dirs = tokio::fs::read_dir(&p).await.expect("读取目录失败");
                let mut paths = vec![];
                while let Some(dir) = dirs.next_entry().await.expect("读取目录失败") {
                    info!("{}", dir.path().display());
                    paths.push(FilePath {
                        path: format!(
                            "http://{}/{}",
                            state.addr,
                            dir.path().display().to_string().trim_start_matches("./")
                        ),
                        name: dir.file_name().to_string_lossy().to_string(),
                    });
                }
                (
                    StatusCode::OK,
                    HtmlOrStringResponse::Html(Html(render_html(paths))),
                )
            } else {
                match tokio::fs::read_to_string(p).await {
                    Ok(content) => (StatusCode::OK, HtmlOrStringResponse::String(content)),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        HtmlOrStringResponse::String(e.to_string()),
                    ),
                }
            }
        }
    }
}

struct FilePath {
    path: String,
    name: String,
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"<li><a href="{}" target="_blank">{}</a></li>"#,
            self.path, self.name
        )
    }
}

fn render_html(paths: Vec<FilePath>) -> String {
    format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>File Server</title>
    </head>
    <body>
        <ul>
            {}
        </ul>
    </body>
    </html>
    "#,
        paths
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
