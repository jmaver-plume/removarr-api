use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub external_id: i32,
    pub title: Option<String>,
    pub title_slug: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub downloaded: Option<bool>,
    pub added_at: Option<DateTime>,
}

impl ActiveModelBehavior for ActiveModel {}
