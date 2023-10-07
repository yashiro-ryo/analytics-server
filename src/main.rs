use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/v1/register", post(register_event))
}

async fn root() -> &'static str {
    "Hello World!!\n"
}

async fn register_event(Json(payload): Json<Event>) -> impl IntoResponse {
    println!(
        "event_name: {}, event_detail: {}",
        payload.event_name, payload.event_detail
    );
    let event = Event {
        event_name: payload.event_name,
        event_detail: payload.event_detail,
    };
    (StatusCode::CREATED, Json(event));
}

#[derive(Deserialize)]
struct Event {
    event_name: String,
    event_detail: String,
}
