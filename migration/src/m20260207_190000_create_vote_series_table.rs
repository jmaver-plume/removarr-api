use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VoteSeries::Table)
                    .if_not_exists()
                    .col(pk_auto(VoteSeries::Id))
                    .col(integer(VoteSeries::VoterId).not_null())
                    .col(integer(VoteSeries::SeriesId).not_null())
                    .col(timestamp(VoteSeries::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_series_voter")
                            .from(VoteSeries::Table, VoteSeries::VoterId)
                            .to(Voter::Table, Voter::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_series_series")
                            .from(VoteSeries::Table, VoteSeries::SeriesId)
                            .to(Series::Table, Series::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint to ensure one vote per voter per series
        manager
            .create_index(
                Index::create()
                    .name("idx_vote_series_unique")
                    .table(VoteSeries::Table)
                    .col(VoteSeries::VoterId)
                    .col(VoteSeries::SeriesId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VoteSeries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VoteSeries {
    Table,
    Id,
    VoterId,
    SeriesId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Voter {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Series {
    Table,
    Id,
}
