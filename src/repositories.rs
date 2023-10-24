use anyhow::Ok;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[async_trait]
pub trait EventRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: Event) -> anyhow::Result<Event>;
    async fn all(&self) -> anyhow::Result<Vec<Event>>;
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Event {
    uid: String,
    event_name: String,
    event_detail: String,
}

#[derive(Debug, Clone)]
pub struct EventRepositoryForDb {
    pool: PgPool,
}

impl EventRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        EventRepositoryForDb { pool }
    }
}

#[async_trait]
impl EventRepository for EventRepositoryForDb {
    async fn create(&self, payload: Event) -> anyhow::Result<Event> {
        let event = sqlx::query_as::<_, Event>(
            r#"
            insert into events_table (uid, event_name, event_detail, timestamp) values ($1, $2, $3, current_timestamp)
            returning *
            "#,
        ).bind(payload.uid.clone()).bind(payload.event_name.clone()).bind(payload.event_detail.clone()).fetch_one(&self.pool).await?;
        Ok(event)
    }

    async fn all(&self) -> anyhow::Result<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            r#"
            select * from events_table;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }
}
