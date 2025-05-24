use axum::extract::{Json, Path, Query};
use axum::{Router, extract::ConnectInfo, response::Html, routing::get};
use serde::Deserialize;
use serde_json::{Value, json};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/users/:id", get(get_user_data));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

#[axum::debug_handler]
async fn get_user_data(
    Path(user_id): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Json<Value> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(30);
    Json(json!({
        "user_id": user_id,
        "page": page,
        "per_page": per_page,
        "items": []
    }))
}

async fn index(ConnectInfo(_addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<!DOCTYPE html>
        <html>
            <head>
                <title>Simple Rust App</title>
            </head>
            <body style=\"font-family: system-ui; font-size: 3rem;\">
                <h1>Hello, There!</h1>
                <p>Use <code>/users/:id?page=1&per_page=30</code> to get user data.</p>
            </body>
        </html>",
    );

    Html(html)
}
