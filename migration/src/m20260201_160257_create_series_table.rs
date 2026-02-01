use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("series")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer_uniq("external_id").not_null())
                    .col(string("text"))
                    .col(string("title"))
                    .col(string("slug"))
                    .col(integer("year"))
                    .col(string("overview"))
                    .col(string("poster_url"))
                    .col(boolean("downloaded").not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("series").to_owned())
            .await
    }
}
