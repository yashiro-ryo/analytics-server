use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::{Event, EventRepository};

pub async fn create_event<T: EventRepository>(
    Json(payload): Json<Event>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let event = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(event)))
}

pub async fn all_events<T: EventRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let events = repository.all().await.unwrap();

    Ok((StatusCode::OK, Json(events)))
}
