use crate::app::AppState;
use crate::entity::movie;
use crate::entity::movie::Entity as Movie;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set};
use serde::Deserialize;
use super::types::Image;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub images: Option<Vec<Image>>,
    pub title_slug: Option<String>,
    pub has_file: Option<bool>,
}

pub async fn handle(request: Request, state: &AppState) -> Result<(), StatusCode> {
    let downloaded = request.has_file.unwrap_or(false);
    
    let existing = Movie::find()
        .filter(movie::Column::ExternalId.eq(request.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(existing) = existing {
        let mut active: movie::ActiveModel = existing.into();
        active.downloaded = Set(Some(downloaded));
        active
            .update(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(())
}
