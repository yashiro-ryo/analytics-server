use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
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
        .route("/api/v1/events", get(get_events))
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
    (StatusCode::CREATED, Json(event))
}

async fn get_events() -> impl IntoResponse {
    let events: [Event; 3] = [
        Event {
            event_name: "onclick".to_string(),
            event_detail: "run-button".to_string(),
        },
        Event {
            event_name: "onclick".to_string(),
            event_detail: "hint-button".to_string(),
        },
        Event {
            event_name: "onclick".to_string(),
            event_detail: "testcase-button".to_string(),
        },
    ];
    (StatusCode::OK, Json(events))
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Event {
    event_name: String,
    event_detail: String,
}

// ここからテスト
#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    // POST /api/v1/register のテスト
    #[tokio::test]
    async fn should_return_event() {
        let req = Request::builder()
            .uri("/api/v1/register")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"{ "event_name": "button-click", "event_detail": "run-button" }"#,
            ))
            .unwrap();

        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", body);
        let event: Event = serde_json::from_str(&body).expect("cannot convert Event instance.");

        assert_eq!(
            event,
            Event {
                event_name: "button-click".to_string(),
                event_detail: "run-button".to_string(),
            }
        );
    }
}
