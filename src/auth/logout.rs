use crate::app::AppState;
use crate::entity::refresh_token;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(as = LogoutRequest)]
pub struct Request {
    refresh_token: String,
}

#[derive(Serialize, ToSchema)]
#[schema(as = LogoutResponse)]
pub struct Response {
    success: bool,
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "Authentication",
    operation_id = "auth_logout",
    request_body = Request,
    security(("jwt" = [])),
    responses(
        (status = 200, description = "Logout successful", body = Response),
        (status = 400, description = "Bad request - refresh token not found or invalid"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
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
