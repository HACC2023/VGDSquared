use poem_openapi::{
    param::Path,
    payload::{self, Json, PlainText},
    types::multipart::JsonField,
    ApiExtractor, ApiResponse, OpenApi,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use sqlx::{prelude::FromRow, query, query_as, Pool, Postgres};
use time::PrimitiveDateTime;

pub struct ThreadApi {
    pool: Pool<Postgres>,
}

impl ThreadApi {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[OpenApi]
impl ThreadApi {
    #[oai(path = "/:id", method = "post")]
    async fn post_thread(&self, json: Json<String>) -> PostThreadResponse {
        let json: PostThreadRequest = match from_str(&json.0) {
            Ok(it) => it,
            Err(err) => return PostThreadResponse::BadJson(PlainText(err.to_string())),
        };

        let res = query!(
            "INSERT INTO thread (category_id, author_id) VALUES ($1, $2) RETURNING id",
            json.category,
            1
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let res = SuccessfulPostThreadResponse { post_id: res.id };

        PostThreadResponse::Ok(Json(to_string(&res).expect("Serialization failed")))
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_thread(&self, id: Path<i32>) -> GetThreadResponse {
        let thread = query_as!(ThreadTable, "SELECT * FROM thread WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
            .expect("Database should be successful");

        let thread = if let Some(thread) = thread {
            if thread.is_deleted {
                return GetThreadResponse::Deleted;
            } else {
                thread
            }
        } else {
            return GetThreadResponse::NotFound;
        };

        let thread = ThreadPreview {
            author_id: thread.author_id,
            created_at: thread.created_at,
            title: todo!(),
            body: todo!(),
            updated_at: todo!(),
        };

        // GetThreadResponse::Ok(Json(to_string(&thread).expect("Serde decided to fail")))
        GetThreadResponse::Ok(todo!())
    }
}

#[derive(FromRow)]
struct ThreadTable {
    pub id: i32,
    pub author_id: i32,
    pub category_id: i32,
    pub created_at: PrimitiveDateTime,
    pub is_deleted: bool,
    pub thread_content_id: Option<i32>,
}

struct ThreadPreview {
    pub author_id: i32,
    pub created_at: PrimitiveDateTime,
    pub title: String,
    pub body: String,
    pub updated_at: Option<String>,
}

#[derive(FromRow)]
struct ThreadContentTable {
    pub id: i32,
    pub thread_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: PrimitiveDateTime,
}

#[derive(ApiResponse)]
enum GetThreadResponse {
    #[oai(status = "200")]
    Ok(Json<String>),
    #[oai(status = "410")]
    Deleted,
    #[oai(status = "404")]
    NotFound,
}

#[derive(Deserialize)]
struct PostThreadRequest {
    category: i32,
    title: String,
    contents: String,
}

#[derive(ApiResponse)]
enum PostThreadResponse {
    /// Post created successfully
    #[oai(status = "200")]
    Ok(Json<String>),
    /// Unauthorized to create a post
    #[oai(status = "401")]
    Unauthorized,
    /// Invalid JSON
    #[oai(status = "400")]
    BadJson(PlainText<String>),
}

#[derive(Serialize)]
struct SuccessfulPostThreadResponse {
    post_id: i32,
}

/*
CREATE TABLE thread (
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL REFERENCES account (id),
    -- For moderation purposes, when the thread is edited or deleted it still remains in the database
    is_deleted BOOLEAN,
    thread_content_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE thread_content (
    id SERIAL PRIMARY KEY,
    thread_id INT NOT NULL REFERENCES thread (id),
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
 */
