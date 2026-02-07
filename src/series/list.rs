use crate::app::AppState;
use crate::entity::series;
use crate::entity::series::Entity as Series;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    id: i32,
    external_id: i32,
    title: Option<String>,
    title_slug: Option<String>,
    year: Option<i32>,
    overview: Option<String>,
    poster_url: Option<String>,
    downloaded: Option<bool>,
    added_at: Option<String>,
}

impl From<series::Model> for Response {
    fn from(value: series::Model) -> Self {
        Self {
            id: value.id,
            external_id: value.external_id,
            title: value.title,
            title_slug: value.title_slug,
            year: value.year,
            overview: value.overview,
            poster_url: value.poster_url,
            downloaded: value.downloaded,
            added_at: value.added_at.map(|dt| dt.to_string()),
        }
    }
}

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let series_list: Vec<series::Model> = Series::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response: Vec<Response> = series_list.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
