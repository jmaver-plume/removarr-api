use crate::app::AppState;
use crate::auth::jwt;
use crate::entity::{admin, refresh_token};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    access_token: String,
    refresh_token: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    // Find admin by username
    let admin_user = admin::Entity::find()
        .filter(admin::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify password
    jwt::verify_password(&payload.password, &admin_user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .then_some(())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Create access token
    let access_token = jwt::create_access_token(admin_user.id, &admin_user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create refresh token
    let refresh_token_str = jwt::create_refresh_token();
    let refresh_token_model = refresh_token::ActiveModel {
        token: Set(refresh_token_str.clone()),
        admin_id: Set(admin_user.id),
        expires_at: Set(jwt::get_refresh_token_expiry()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        revoked: Set(false),
        ..Default::default()
    };
    
    refresh_token_model
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(Response {
        access_token,
        refresh_token: refresh_token_str,
    }))
}
