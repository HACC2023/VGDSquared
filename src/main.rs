use const_format::concatcp;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use sqlx::{query, PgPool, Pool, Postgres};

struct Api {
    pool: Pool<Postgres>,
}

impl Api {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
    #[oai(path = "/math", method = "get")]
    async fn math(&self) -> PlainText<String> {
        let res = query!("SELECT 1 + 1 AS math_result")
            .fetch_optional(&self.pool)
            .await
            .expect("The database should have the capacities to add two numbers");
        PlainText(format!("1 + 1 equals {:?}", res))
    }
}

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    color_eyre::install()?;

    let conn_str = get_url();
    let pool = PgPool::connect(&conn_str).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let api_service =
        OpenApiService::new(Api::new(pool), "HACC", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/api/docs", ui)
        .nest(
            "/",
            StaticFilesEndpoint::new("public/")
                .show_files_listing()
                .index_file("index.html"),
        );

    println!("Starting server on port http://localhost:{}", PORT);

    poem::Server::new(TcpListener::bind(concatcp!("127.0.0.1:", PORT)))
        .run(app)
        .await?;

    Ok(())
}

fn get_url() -> String {
    dotenv::dotenv().ok();
    dotenv::var("DATABASE_URL").expect("Field DATABASE_URL must be included in the .env file")
}
