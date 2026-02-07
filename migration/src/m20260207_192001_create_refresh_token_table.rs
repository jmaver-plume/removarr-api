use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(pk_auto(RefreshToken::Id))
                    .col(string_uniq(RefreshToken::Token).not_null())
                    .col(integer(RefreshToken::AdminId).not_null())
                    .col(timestamp(RefreshToken::ExpiresAt).not_null())
                    .col(timestamp(RefreshToken::CreatedAt).not_null())
                    .col(boolean(RefreshToken::Revoked).not_null().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refresh_token_admin")
                            .from(RefreshToken::Table, RefreshToken::AdminId)
                            .to(Admin::Table, Admin::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RefreshToken {
    Table,
    Id,
    Token,
    AdminId,
    ExpiresAt,
    CreatedAt,
    Revoked,
}

#[derive(DeriveIden)]
enum Admin {
    Table,
    Id,
}
