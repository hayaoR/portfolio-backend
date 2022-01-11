use axum::{extract::Extension, Json};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Skill {
    id: i32,
    title: String,
    content: String,
}

#[tracing::instrument(name = "reading skills data")]
pub async fn skills(Extension(pool): Extension<PgPool>) -> Json<Vec<Skill>> {
    let result = sqlx::query!("select * from skill where userid = $1", 1)
        .fetch_all(&pool)
        .await;
    let mut v = vec![];
    match result {
        Ok(rows) => {
            for row in rows {
                v.push(Skill {
                    id: row.id,
                    title: row.title,
                    content: row.content,
                });
            }
            return Json(v);
        }
        Err(err) => {
            tracing::error!("Failed to read about data {:?}", err);
            return Json(v);
        }
    }
}
