use crate::app::AppState;
use crate::entity::movie;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use super::types::{Image, get_best_image_url};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub images: Option<Vec<Image>>,
    pub title_slug: Option<String>,
}

pub async fn handle(request: Request, state: &AppState) -> Result<(), StatusCode> {
    let poster_url = get_best_image_url(&request.images);
    
    let new_movie = movie::ActiveModel {
        external_id: Set(request.id),
        title: Set(request.title),
        title_slug: Set(request.title_slug),
        year: Set(request.year),
        overview: Set(request.overview),
        poster_url: Set(poster_url),
        downloaded: Set(Some(false)),
        added_at: Set(Some(chrono::Utc::now().naive_utc())),
        ..Default::default()
    };
    
    new_movie
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(())
}
