use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VoteMovie::Table)
                    .if_not_exists()
                    .col(pk_auto(VoteMovie::Id))
                    .col(integer(VoteMovie::VoterId).not_null())
                    .col(integer(VoteMovie::MovieId).not_null())
                    .col(timestamp(VoteMovie::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_movie_voter")
                            .from(VoteMovie::Table, VoteMovie::VoterId)
                            .to(Voter::Table, Voter::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_movie_movie")
                            .from(VoteMovie::Table, VoteMovie::MovieId)
                            .to(Movie::Table, Movie::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint to ensure one vote per voter per movie
        manager
            .create_index(
                Index::create()
                    .name("idx_vote_movie_unique")
                    .table(VoteMovie::Table)
                    .col(VoteMovie::VoterId)
                    .col(VoteMovie::MovieId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VoteMovie::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VoteMovie {
    Table,
    Id,
    VoterId,
    MovieId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Voter {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Movie {
    Table,
    Id,
}
