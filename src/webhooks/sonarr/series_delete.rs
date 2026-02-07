use crate::app::AppState;
use crate::entity::series;
use crate::entity::series::Entity as Series;
use http::StatusCode;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ModelTrait};
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
}

pub async fn handle(request: Request, state: &AppState) -> Result<(), StatusCode> {
    let existing = Series::find()
        .filter(series::Column::ExternalId.eq(request.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(existing) = existing {
        existing
            .delete(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(())
}
