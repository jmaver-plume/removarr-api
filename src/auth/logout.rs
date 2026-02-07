use crate::app::AppState;
use crate::entity::refresh_token;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    refresh_token: String,
}

#[derive(Serialize)]
pub struct Response {
    success: bool,
}

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    // Find and revoke the refresh token
    let token = refresh_token::Entity::find()
        .filter(refresh_token::Column::Token.eq(&payload.refresh_token))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let mut token: refresh_token::ActiveModel = token.into();
    token.revoked = Set(true);
    token
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(Response { success: true }))
}
