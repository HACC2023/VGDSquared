use poem_openapi::{OpenApi, payload::PlainText, param::Query};
use sqlx::{Postgres, Pool, query};

pub mod auth;
pub mod extractor;
pub mod role;
pub mod thread;

pub struct MainApi {
    pool: Pool<Postgres>,
}

impl MainApi {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

/// This API will get removed soon, but until it does, it is like a misc API
#[poem_grants::open_api]
#[OpenApi]
impl MainApi {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[has_permissions("something")]
    #[oai(path = "/math", method = "get")]
    async fn math(&self) -> PlainText<String> {
        let res = query!("SELECT 1 + 1 AS math_result")
            .fetch_optional(&self.pool)
            .await
            .expect("The database should have the capacity to add two numbers");
        PlainText(format!("1 + 1 equals {:?}", res))
    }
}