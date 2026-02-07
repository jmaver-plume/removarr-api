use crate::app::AppState;
use crate::auth::jwt::{self, Claims};
use crate::entity::admin;
use axum::extract::{State, Extension};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    current_password: String,
    new_password: String,
}

#[derive(Serialize)]
pub struct Response {
    success: bool,
}

pub async fn handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    // Find admin
    let admin_user = admin::Entity::find_by_id(claims.admin_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Verify current password
    jwt::verify_password(&payload.current_password, &admin_user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .then_some(())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Hash new password
    let new_hash = jwt::hash_password(&payload.new_password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Update password
    let mut admin_active: admin::ActiveModel = admin_user.into();
    admin_active.password_hash = Set(new_hash);
    admin_active.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    admin_active
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(Response { success: true }))
}
