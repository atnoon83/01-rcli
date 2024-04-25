use crate::ServeState;
use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Serving on addr {} from path {:?}", addr, dir);
    let state = ServeState { dir: dir.clone() };
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*dir", get(file_handler))
        .with_state(Arc::new(state));

    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<ServeState>>,
    Path(dir): Path<PathBuf>,
) -> (StatusCode, String) {
    let path = std::path::Path::new(&state.dir).join(dir);
    info!("reading file {:?}", path);
    if !path.exists() {
        warn!("file not found: {:?}", path);
        (StatusCode::NOT_FOUND, "File not found".to_string())
    } else {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                )
            }
        }
    }
}
