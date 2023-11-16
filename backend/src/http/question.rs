use crate::http::{Error, Result};
use axum::extract::Path;
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new().route("/v1/question/:question_level", get(get_questions))
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    level: i32,
    question: String,
}

async fn get_questions(
    Extension(db): Extension<PgPool>,
    Path(question_level): Path<i32>,
) -> Result<Json<Vec<Question>>> {
    // Make some check here preventing question level queries above a certain number

    let questions = sqlx::query_as!(
        Question,
        "SELECT * FROM question WHERE level = $1",
        question_level
    )
    .fetch_all(&db)
    .await?;

    Ok(Json(questions))
}
