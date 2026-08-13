use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::{StatusCode, Request},
    middleware::{self, Next},
    Json,
};
use tower_http::services::{ServeDir, ServeFile};
use std::env;
use std::sync::Arc;
use tracing::{info, error};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    admin_token: String,
}

pub async fn run() {
    tracing_subscriber::fmt::init();
    
    let admin_token = env::var("ADMIN_TOKEN").unwrap_or_else(|_| {
        info!("ADMIN_TOKEN not found in environment, falling back to empty string for testing");
        "".to_string()
    });
    
    if admin_token.is_empty() {
        error!("ADMIN_TOKEN is empty!");
        // We panic if it's strictly required by requirements "Read an ADMIN_TOKEN environment variable on startup"
        panic!("ADMIN_TOKEN environment variable not set");
    }

    let state = Arc::new(AppState { admin_token });

    let api_routes = Router::new()
        .route("/read", get(api_read))
        .route("/sync", post(api_sync).route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware)));

    let serve_dir = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(serve_dir)
        .with_state(state);

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:4405".to_string());
    
    let listener = match TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to {}: {}", bind_addr, e);
            return;
        }
    };

    info!("Backend listening on {}", bind_addr);
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
    }
}

async fn api_read() -> impl IntoResponse {
    (StatusCode::OK, "Read OK")
}

async fn api_sync() -> impl IntoResponse {
    (StatusCode::OK, "Sync OK")
}

async fn auth_middleware(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> axum::response::Response {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if token == state.admin_token {
                    return next.run(req).await;
                }
            }
        }
    }
    
    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
