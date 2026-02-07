use crate::app::AppState;
use crate::entity::vote_movie;
use crate::entity::vote_movie::Entity as VoteMovie;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub voter_id: i32,
    pub movie_id: i32,
}

pub async fn handler(
    State(state): State<AppState>,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let vote = VoteMovie::find()
        .filter(vote_movie::Column::VoterId.eq(req.voter_id))
        .filter(vote_movie::Column::MovieId.eq(req.movie_id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    vote.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
