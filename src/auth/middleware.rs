use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::app::AppState;
use crate::auth::jwt;

pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let claims = jwt::verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Store claims in request extensions for handlers to use
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}
