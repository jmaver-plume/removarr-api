use crate::app::AppState;
use crate::entity::series;
use crate::entity::series::Entity as Series;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set};
use serde::Deserialize;
use super::types::Image;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub percent_of_episodes: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub images: Option<Vec<Image>>,
    pub title_slug: Option<String>,
    pub statistics: Option<Statistics>,
}

pub async fn handle(request: Request, state: &AppState) -> Result<(), StatusCode> {
    let downloaded = request.statistics
        .as_ref()
        .and_then(|s| s.percent_of_episodes)
        .map(|p| p >= 100.0)
        .unwrap_or(false);
    
    let existing = Series::find()
        .filter(series::Column::ExternalId.eq(request.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(existing) = existing {
        let mut active: series::ActiveModel = existing.into();
        active.downloaded = Set(Some(downloaded));
        active
            .update(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(())
}
