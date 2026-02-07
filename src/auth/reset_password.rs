use crate::app::AppState;
use crate::auth::jwt::{self, Claims};
use crate::entity::admin;
use axum::extract::{State, Extension};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(as = ResetPasswordRequest)]
pub struct Request {
    current_password: String,
    new_password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(as = ResetPasswordResponse)]
pub struct Response {
    success: bool,
}

#[utoipa::path(
    post,
    path = "/api/auth/reset-password",
    tag = "Authentication",
    operation_id = "auth_reset_password",
    request_body = Request,
    security(("jwt" = [])),
    responses(
        (status = 200, description = "Password reset successful", body = Response),
        (status = 401, description = "Unauthorized - invalid or missing JWT token, or current password is incorrect"),
        (status = 404, description = "Admin user not found"),
        (status = 500, description = "Internal server error - database or password hashing error")
    )
)]
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
