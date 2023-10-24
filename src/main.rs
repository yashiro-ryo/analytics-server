mod handlers;
mod repositories;

use crate::repositories::{EventRepository, EventRepositoryForDb};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use handlers::{all_events, create_event};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start connect database...");
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));
    let repository = EventRepositoryForDb::new(pool.clone());

    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3031));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: EventRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route(
            "/api/v1/events",
            post(create_event::<T>).get(all_events::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello World!!\n"
}
