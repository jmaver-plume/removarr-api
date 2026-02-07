mod types;
mod series_add;
mod series_download;
mod series_delete;

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
    series: Option<serde_json::Value>,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    match payload.event_type.as_str() {
        "SeriesAdd" => {
            if let Some(series) = payload.series {
                let request: series_add::Request = serde_json::from_value(series)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                series_add::handle(request, &state).await?;
            }
        }
        "Download" => {
            if let Some(series) = payload.series {
                let request: series_download::Request = serde_json::from_value(series)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                series_download::handle(request, &state).await?;
            }
        }
        "SeriesDelete" => {
            if let Some(series) = payload.series {
                let request: series_delete::Request = serde_json::from_value(series)
                    .map_err(|_| StatusCode::BAD_REQUEST)?;
                series_delete::handle(request, &state).await?;
            }
        }
        _ => {}
    }
    
    Ok(Json(Response { success: true }))
}

