use crate::app::AppState;
use crate::entity::vote_movie;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    voter_id: i32,
    movie_id: i32,
}

#[derive(Serialize)]
pub struct Response {
    id: i32,
    voter_id: i32,
    movie_id: i32,
    created_at: String,
}

impl From<vote_movie::Model> for Response {
    fn from(value: vote_movie::Model) -> Self {
        Self {
            id: value.id,
            voter_id: value.voter_id,
            movie_id: value.movie_id,
            created_at: value.created_at.to_string(),
        }
    }
}

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let vote = vote_movie::ActiveModel {
        voter_id: Set(payload.voter_id),
        movie_id: Set(payload.movie_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    
    let vote = vote
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::CONFLICT)?; // Unique constraint violation
    
    let response: Response = vote.into();
    Ok(Json(response))
}
