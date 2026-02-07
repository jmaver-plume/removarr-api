use removarr_rust::entity::{movie, series, vote_movie, vote_series, voter};
use sea_orm::{
    ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set,
};
use migration::{Migrator, MigratorTrait};

async fn setup_test_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    db
}

#[tokio::test]
async fn test_series_delete_requires_all_votes() {
    let db = setup_test_db().await;

    // Create 3 voters
    let voter1 = voter::ActiveModel {
        name: Set("Voter 1".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter2 = voter::ActiveModel {
        name: Set("Voter 2".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter3 = voter::ActiveModel {
        name: Set("Voter 3".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Create a series
    let test_series = series::ActiveModel {
        external_id: Set(123),
        title: Set(Some("Test Series".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Only 2 out of 3 voters vote
    vote_series::ActiveModel {
        voter_id: Set(voter1.id),
        series_id: Set(test_series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    vote_series::ActiveModel {
        voter_id: Set(voter2.id),
        series_id: Set(test_series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Verify the series exists
    let found = series::Entity::find_by_id(test_series.id)
        .one(&db)
        .await
        .unwrap();
    assert!(found.is_some());

    // Count voters and votes
    use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
    let total_voters = voter::Entity::find().count(&db).await.unwrap();
    let votes = vote_series::Entity::find()
        .filter(vote_series::Column::SeriesId.eq(test_series.id))
        .count(&db)
        .await
        .unwrap();

    assert_eq!(total_voters, 3);
    assert_eq!(votes, 2);

    // This simulates the check in delete handler - should fail
    assert!(votes < total_voters, "Should not have enough votes to delete");
}

#[tokio::test]
async fn test_series_delete_succeeds_with_all_votes() {
    let db = setup_test_db().await;

    // Create 2 voters
    let voter1 = voter::ActiveModel {
        name: Set("Voter 1".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter2 = voter::ActiveModel {
        name: Set("Voter 2".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Create a series
    let test_series = series::ActiveModel {
        external_id: Set(456),
        title: Set(Some("Test Series 2".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Both voters vote
    vote_series::ActiveModel {
        voter_id: Set(voter1.id),
        series_id: Set(test_series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    vote_series::ActiveModel {
        voter_id: Set(voter2.id),
        series_id: Set(test_series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Count voters and votes
    use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
    let total_voters = voter::Entity::find().count(&db).await.unwrap();
    let votes = vote_series::Entity::find()
        .filter(vote_series::Column::SeriesId.eq(test_series.id))
        .count(&db)
        .await
        .unwrap();

    assert_eq!(total_voters, 2);
    assert_eq!(votes, 2);

    // This simulates the check in delete handler - should pass
    assert_eq!(votes, total_voters, "Should have all votes needed to delete");

    // Now actually delete
    use sea_orm::ModelTrait;
    let series_id = test_series.id;
    test_series.delete(&db).await.unwrap();

    // Verify deleted
    let found = series::Entity::find_by_id(series_id)
        .one(&db)
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_movie_delete_requires_all_votes() {
    let db = setup_test_db().await;

    // Create 3 voters
    let voter1 = voter::ActiveModel {
        name: Set("Voter 1".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter2 = voter::ActiveModel {
        name: Set("Voter 2".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter3 = voter::ActiveModel {
        name: Set("Voter 3".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Create a movie
    let test_movie = movie::ActiveModel {
        external_id: Set(789),
        title: Set(Some("Test Movie".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Only 1 out of 3 voters votes
    vote_movie::ActiveModel {
        voter_id: Set(voter1.id),
        movie_id: Set(test_movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Count voters and votes
    use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
    let total_voters = voter::Entity::find().count(&db).await.unwrap();
    let votes = vote_movie::Entity::find()
        .filter(vote_movie::Column::MovieId.eq(test_movie.id))
        .count(&db)
        .await
        .unwrap();

    assert_eq!(total_voters, 3);
    assert_eq!(votes, 1);

    // This simulates the check in delete handler - should fail
    assert!(votes < total_voters, "Should not have enough votes to delete");
}

#[tokio::test]
async fn test_movie_delete_succeeds_with_all_votes() {
    let db = setup_test_db().await;

    // Create 2 voters
    let voter1 = voter::ActiveModel {
        name: Set("Voter 1".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let voter2 = voter::ActiveModel {
        name: Set("Voter 2".to_string()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Create a movie
    let test_movie = movie::ActiveModel {
        external_id: Set(999),
        title: Set(Some("Test Movie 2".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Both voters vote
    vote_movie::ActiveModel {
        voter_id: Set(voter1.id),
        movie_id: Set(test_movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    vote_movie::ActiveModel {
        voter_id: Set(voter2.id),
        movie_id: Set(test_movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Count voters and votes
    use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
    let total_voters = voter::Entity::find().count(&db).await.unwrap();
    let votes = vote_movie::Entity::find()
        .filter(vote_movie::Column::MovieId.eq(test_movie.id))
        .count(&db)
        .await
        .unwrap();

    assert_eq!(total_voters, 2);
    assert_eq!(votes, 2);

    // This simulates the check in delete handler - should pass
    assert_eq!(votes, total_voters, "Should have all votes needed to delete");

    // Now actually delete
    use sea_orm::ModelTrait;
    let movie_id = test_movie.id;
    test_movie.delete(&db).await.unwrap();

    // Verify deleted
    let found = movie::Entity::find_by_id(movie_id)
        .one(&db)
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_zero_voters_allows_deletion() {
    let db = setup_test_db().await;

    // Create a series with no voters in the system
    let test_series = series::ActiveModel {
        external_id: Set(111),
        title: Set(Some("Orphan Series".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Count voters and votes
    use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
    let total_voters = voter::Entity::find().count(&db).await.unwrap();
    let votes = vote_series::Entity::find()
        .filter(vote_series::Column::SeriesId.eq(test_series.id))
        .count(&db)
        .await
        .unwrap();

    assert_eq!(total_voters, 0);
    assert_eq!(votes, 0);

    // With 0 voters and 0 votes, deletion should be allowed
    assert_eq!(votes, total_voters, "Zero voters means deletion allowed");

    // Actually delete
    use sea_orm::ModelTrait;
    let series_id = test_series.id;
    test_series.delete(&db).await.unwrap();

    // Verify deleted
    let found = series::Entity::find_by_id(series_id)
        .one(&db)
        .await
        .unwrap();
    assert!(found.is_none());
}
