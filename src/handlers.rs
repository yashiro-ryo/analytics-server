use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::{Event, EventRepository};

pub async fn create_event<T: EventRepository>(
    Json(payload): Json<Event>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let event = repository.create(payload);

    (StatusCode::CREATED, Json(event))
}

pub async fn all_events<T: EventRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let events = repository.all();

    (StatusCode::OK, Json(events))
}
