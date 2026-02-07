use sea_orm::{Database, DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use migration::{Migrator, MigratorTrait};

async fn setup_test_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to test database");
    Migrator::up(&db, None).await.expect("Failed to migrate test database");
    db
}

#[tokio::test]
async fn test_vote_series_cascade_on_voter_delete() {
    use removarr_rust::entity::{voter, series, vote_series};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Alice".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a series
    let series = series::ActiveModel {
        external_id: Set(1),
        ..Default::default()
    };
    let series = series.insert(&db).await.unwrap();
    
    // Create a vote
    let vote = vote_series::ActiveModel {
        voter_id: Set(voter.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    let vote = vote.insert(&db).await.unwrap();
    
    // Verify vote exists
    let votes = vote_series::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 1);
    assert_eq!(votes[0].id, vote.id);
    
    // Delete the voter
    use sea_orm::ModelTrait;
    voter.delete(&db).await.unwrap();
    
    // Verify vote was cascade deleted
    let votes = vote_series::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 0);
}

#[tokio::test]
async fn test_vote_series_cascade_on_series_delete() {
    use removarr_rust::entity::{voter, series, vote_series};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Bob".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a series
    let series = series::ActiveModel {
        external_id: Set(2),
        ..Default::default()
    };
    let series = series.insert(&db).await.unwrap();
    
    // Create a vote
    let vote = vote_series::ActiveModel {
        voter_id: Set(voter.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote.insert(&db).await.unwrap();
    
    // Verify vote exists
    let votes = vote_series::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 1);
    
    // Delete the series
    use sea_orm::ModelTrait;
    series.delete(&db).await.unwrap();
    
    // Verify vote was cascade deleted
    let votes = vote_series::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 0);
}

#[tokio::test]
async fn test_vote_movie_cascade_on_voter_delete() {
    use removarr_rust::entity::{voter, movie, vote_movie};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Charlie".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a movie
    let movie = movie::ActiveModel {
        external_id: Set(1),
        ..Default::default()
    };
    let movie = movie.insert(&db).await.unwrap();
    
    // Create a vote
    let vote = vote_movie::ActiveModel {
        voter_id: Set(voter.id),
        movie_id: Set(movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote.insert(&db).await.unwrap();
    
    // Verify vote exists
    let votes = vote_movie::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 1);
    
    // Delete the voter
    use sea_orm::ModelTrait;
    voter.delete(&db).await.unwrap();
    
    // Verify vote was cascade deleted
    let votes = vote_movie::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 0);
}

#[tokio::test]
async fn test_vote_movie_cascade_on_movie_delete() {
    use removarr_rust::entity::{voter, movie, vote_movie};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Diana".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a movie
    let movie = movie::ActiveModel {
        external_id: Set(2),
        ..Default::default()
    };
    let movie = movie.insert(&db).await.unwrap();
    
    // Create a vote
    let vote = vote_movie::ActiveModel {
        voter_id: Set(voter.id),
        movie_id: Set(movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote.insert(&db).await.unwrap();
    
    // Verify vote exists
    let votes = vote_movie::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 1);
    
    // Delete the movie
    use sea_orm::ModelTrait;
    movie.delete(&db).await.unwrap();
    
    // Verify vote was cascade deleted
    let votes = vote_movie::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 0);
}

#[tokio::test]
async fn test_vote_series_unique_constraint() {
    use removarr_rust::entity::{voter, series, vote_series};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Eve".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a series
    let series = series::ActiveModel {
        external_id: Set(3),
        ..Default::default()
    };
    let series = series.insert(&db).await.unwrap();
    
    // Create first vote
    let vote1 = vote_series::ActiveModel {
        voter_id: Set(voter.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote1.insert(&db).await.unwrap();
    
    // Try to create duplicate vote (should fail)
    let vote2 = vote_series::ActiveModel {
        voter_id: Set(voter.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    let result = vote2.insert(&db).await;
    
    assert!(result.is_err(), "Duplicate vote should fail due to unique constraint");
}

#[tokio::test]
async fn test_vote_movie_unique_constraint() {
    use removarr_rust::entity::{voter, movie, vote_movie};
    
    let db = setup_test_db().await;
    
    // Create a voter
    let voter = voter::ActiveModel {
        name: Set("Frank".to_string()),
        ..Default::default()
    };
    let voter = voter.insert(&db).await.unwrap();
    
    // Create a movie
    let movie = movie::ActiveModel {
        external_id: Set(3),
        ..Default::default()
    };
    let movie = movie.insert(&db).await.unwrap();
    
    // Create first vote
    let vote1 = vote_movie::ActiveModel {
        voter_id: Set(voter.id),
        movie_id: Set(movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote1.insert(&db).await.unwrap();
    
    // Try to create duplicate vote (should fail)
    let vote2 = vote_movie::ActiveModel {
        voter_id: Set(voter.id),
        movie_id: Set(movie.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    let result = vote2.insert(&db).await;
    
    assert!(result.is_err(), "Duplicate vote should fail due to unique constraint");
}

#[tokio::test]
async fn test_multiple_voters_can_vote_same_series() {
    use removarr_rust::entity::{voter, series, vote_series};
    
    let db = setup_test_db().await;
    
    // Create multiple voters
    let voter1 = voter::ActiveModel {
        name: Set("George".to_string()),
        ..Default::default()
    };
    let voter1 = voter1.insert(&db).await.unwrap();
    
    let voter2 = voter::ActiveModel {
        name: Set("Helen".to_string()),
        ..Default::default()
    };
    let voter2 = voter2.insert(&db).await.unwrap();
    
    let voter3 = voter::ActiveModel {
        name: Set("Ivan".to_string()),
        ..Default::default()
    };
    let voter3 = voter3.insert(&db).await.unwrap();
    
    // Create a series
    let series = series::ActiveModel {
        external_id: Set(4),
        ..Default::default()
    };
    let series = series.insert(&db).await.unwrap();
    
    // Each voter votes for the same series
    let vote1 = vote_series::ActiveModel {
        voter_id: Set(voter1.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote1.insert(&db).await.unwrap();
    
    let vote2 = vote_series::ActiveModel {
        voter_id: Set(voter2.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote2.insert(&db).await.unwrap();
    
    let vote3 = vote_series::ActiveModel {
        voter_id: Set(voter3.id),
        series_id: Set(series.id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    vote3.insert(&db).await.unwrap();
    
    // Verify all 3 votes exist
    let votes = vote_series::Entity::find().all(&db).await.unwrap();
    assert_eq!(votes.len(), 3);
}
