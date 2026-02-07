use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Movie::Table)
                    .if_not_exists()
                    .col(pk_auto(Movie::Id))
                    .col(integer(Movie::ExternalId).not_null())
                    .col(string_null(Movie::Title))
                    .col(string_null(Movie::TitleSlug))
                    .col(integer_null(Movie::Year))
                    .col(string_null(Movie::Overview))
                    .col(string_null(Movie::PosterUrl))
                    .col(boolean_null(Movie::Downloaded))
                    .col(timestamp_null(Movie::AddedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movie::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Movie {
    Table,
    Id,
    ExternalId,
    Title,
    TitleSlug,
    Year,
    Overview,
    PosterUrl,
    Downloaded,
    AddedAt,
}
