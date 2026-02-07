use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Removarr API",
        version = "0.1.0",
        description = "API for managing media removal voting system integrated with Sonarr and Radarr"
    ),
    paths(
        // Auth
        crate::auth::login::handler,
        crate::auth::logout::handler,
        crate::auth::reset_password::handler,
        
        // Voters
        crate::voters::post::handler,
        crate::voters::list::handler,
        crate::voters::get::handler,
        crate::voters::patch::handler,
        crate::voters::delete::handler,
        
        // Settings
        crate::settings::get::handler,
        crate::settings::put::handler,
        
        // Series
        crate::series::list::handler,
        crate::series::delete::handler,
        
        // Movies
        crate::movies::list::handler,
        crate::movies::delete::handler,
        
        // Votes
        crate::votes_series::post::handler,
        crate::votes_series::list::handler,
        crate::votes_series::delete::handler,
        crate::votes_movie::post::handler,
        crate::votes_movie::list::handler,
        crate::votes_movie::delete::handler,
        
        // Webhooks
        crate::webhooks::sonarr::handler,
        crate::webhooks::radarr::handler,
    ),
    components(schemas(
        // Auth schemas
        crate::auth::login::Request,
        crate::auth::login::Response,
        crate::auth::logout::Request,
        crate::auth::logout::Response,
        crate::auth::reset_password::Request,
        crate::auth::reset_password::Response,
        
        // Voter schemas
        crate::voters::post::Request,
        crate::voters::post::Response,
        crate::voters::list::Response,
        crate::voters::get::Response,
        crate::voters::patch::Request,
        
        // Settings schemas
        crate::settings::get::Response,
        crate::settings::get::Config,
        crate::settings::get::Credentials,
        crate::settings::put::Request,
        crate::settings::put::Config,
        crate::settings::put::Credentials,
        
        // Series schemas
        crate::series::list::Response,
        
        // Movie schemas
        crate::movies::list::Response,
        
        // Vote schemas
        crate::votes_series::post::Request,
        crate::votes_series::post::Response,
        crate::votes_series::list::Response,
        crate::votes_series::delete::Request,
        crate::votes_movie::post::Request,
        crate::votes_movie::post::Response,
        crate::votes_movie::list::Response,
        crate::votes_movie::delete::Request,
        
        // Webhook schemas
        crate::webhooks::sonarr::Response,
        crate::webhooks::sonarr::WebhookPayload,
        crate::webhooks::radarr::Response,
        crate::webhooks::radarr::WebhookPayload,
        
        // Error schema
        crate::error::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "Authentication endpoints for login, logout, and password management"),
        (name = "Voters", description = "Manage voters who can vote on media deletion"),
        (name = "Settings", description = "Application settings for Sonarr and Radarr integration"),
        (name = "Series", description = "TV series tracked from Sonarr"),
        (name = "Movies", description = "Movies tracked from Radarr"),
        (name = "Votes", description = "Voting endpoints for series and movie deletion"),
        (name = "Webhooks", description = "Webhook endpoints for Sonarr and Radarr integration (no authentication required)")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token obtained from /api/auth/login endpoint"))
                        .build()
                ),
            )
        }
    }
}
