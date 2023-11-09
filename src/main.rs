use const_format::concatcp;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, EndpointExt, Route};
use poem_grants::GrantsMiddleware;
use poem_openapi::OpenApiService;
use sqlx::PgPool;
use tracing::info;

use crate::api::{auth::AuthApi, extractor::auth_extractor, MainApi};

pub mod api;
pub mod db;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    color_eyre::install().unwrap();
    // Logging setup
    let tracing_sub = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub).unwrap();
    dotenv().ok();

    const DATABASE_URL: &str = dotenv!("DATABASE_URL", "DATABASE_URL must be passed in");
    let pool = PgPool::connect(DATABASE_URL).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let main_service = OpenApiService::new(MainApi::new(pool.clone()), "Main", "1.0")
        .server(concatcp!("http://localhost:", PORT, "/api"));

    let auth_service = OpenApiService::new(AuthApi::new(pool.clone()), "Auth", "1.0")
        .server(concatcp!("http://localhost:", PORT, "/api/oauth"));

    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("public/")
                .show_files_listing()
                .index_file("index.html"),
        )
        .nest("/api/docs", main_service.swagger_ui())
        .nest("/api", main_service)
        .nest("/api/oauth/docs", auth_service.swagger_ui())
        .nest("/api/oauth", auth_service)
        .with(GrantsMiddleware::with_extractor(auth_extractor));

    println!("Starting server on port http://localhost:{}", PORT);

    poem::Server::new(TcpListener::bind(concatcp!("127.0.0.1:", PORT)))
        .run(app)
        .await?;

    Ok(())
}
