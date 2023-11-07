use dotenv::dotenv;
use const_format::concatcp;
use dotenv_codegen::dotenv;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use sqlx::PgPool;

use crate::api::{auth::AuthApi, MainApi};

pub mod api;
pub mod db;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    color_eyre::install()?;
    dotenv().ok();

    const DATABASE_URL: &str = dotenv!("DATABASE_URL", "DATABASE_URL must be passed in");
    let pool = PgPool::connect(&DATABASE_URL).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let main_service = OpenApiService::new(MainApi::new(pool.clone()), "Main", "1.0")
        .server(concatcp!("http://localhost:", PORT, "/api"));

    let auth_service = OpenApiService::new(AuthApi::new(pool.clone()), "Auth", "1.0")
        .server(concatcp!("http://localhost:", PORT, "/api/sessions/"));

    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("public/")
                .show_files_listing()
                .index_file("index.html"),
        )
        .nest("/api/docs", main_service.swagger_ui())
        .nest("/api", main_service)

        .nest("/api/sessions/docs", auth_service.swagger_ui())
        .nest("/api/sessions", auth_service)
        ;


    println!("Starting server on port http://localhost:{}", PORT);

    poem::Server::new(TcpListener::bind(concatcp!("127.0.0.1:", PORT)))
        .run(app)
        .await?;

    Ok(())
}
