pub mod jwt;
pub mod middleware;
pub mod login;
pub mod logout;
pub mod reset_password;

use crate::entity::admin;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, PaginatorTrait};

pub async fn initialize_admin(db: &DatabaseConnection) {
    // Check if admin already exists
    let admin_count = admin::Entity::find()
        .count(db)
        .await
        .expect("Failed to count admins");
    
    if admin_count == 0 {
        // Generate random password
        let password = jwt::generate_random_password(16);
        let password_hash = jwt::hash_password(&password)
            .expect("Failed to hash password");
        
        // Create admin
        let admin_model = admin::ActiveModel {
            username: Set("removarr".to_string()),
            password_hash: Set(password_hash),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        
        admin_model
            .insert(db)
            .await
            .expect("Failed to create admin");
        
        // Log credentials to stdout
        println!("╔════════════════════════════════════════════════════╗");
        println!("║          DEFAULT ADMIN CREATED                     ║");
        println!("╠════════════════════════════════════════════════════╣");
        println!("║  Username: removarr                                ║");
        println!("║  Password: {:<39} ║", password);
        println!("╠════════════════════════════════════════════════════╣");
        println!("║  ⚠️  IMPORTANT: Save this password now!            ║");
        println!("║  It will not be shown again.                       ║");
        println!("╚════════════════════════════════════════════════════╝");
    }
}
