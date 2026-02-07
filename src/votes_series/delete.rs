use crate::app::AppState;
use crate::entity::vote_series;
use crate::entity::vote_series::Entity as VoteSeries;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub voter_id: i32,
    pub series_id: i32,
}

pub async fn handler(
    State(state): State<AppState>,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let vote = VoteSeries::find()
        .filter(vote_series::Column::VoterId.eq(req.voter_id))
        .filter(vote_series::Column::SeriesId.eq(req.series_id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    vote.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
