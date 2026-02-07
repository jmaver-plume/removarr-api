pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_voter_table;
mod m20260201_160249_create_settings_table;
mod m20260201_160257_create_series_table;
mod m20260201_160258_create_movies_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_voter_table::Migration),
            Box::new(m20260201_160249_create_settings_table::Migration),
            Box::new(m20260201_160257_create_series_table::Migration),
            Box::new(m20260201_160258_create_movies_table::Migration),
        ]
    }
}
