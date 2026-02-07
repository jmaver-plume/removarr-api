mod types;
mod movie_added;
mod movie_download;
mod movie_delete;

use crate::app::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct Response {
    success: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WebhookPayload {
    event_type: String,
    movie: Option<serde_json::Value>,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    match payload.event_type.as_str() {
        "MovieAdded" => {
            if let Some(movie) = payload.movie {
                let request: movie_added::Request = serde_json::from_value(movie)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                movie_added::handle(request, &state).await?;
            }
        }
        "Download" => {
            if let Some(movie) = payload.movie {
                let request: movie_download::Request = serde_json::from_value(movie)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                movie_download::handle(request, &state).await?;
            }
        }
        "MovieDelete" => {
            if let Some(movie) = payload.movie {
                let request: movie_delete::Request = serde_json::from_value(movie)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                movie_delete::handle(request, &state).await?;
            }
        }
        _ => {}
    }
    
    Ok(Json(Response { success: true }))
}
