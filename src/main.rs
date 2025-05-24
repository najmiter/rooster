use axum::extract::{Json, Path, Query};
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::{
    Router,
    body::Body,
    extract::ConnectInfo,
    middleware::{self, Next},
    response::Html,
    response::Response,
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}
async fn auth(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let url = req.uri_mut().path();
    let username = url.split('/').last().unwrap_or("");

    if username.is_empty() {
        let body = Json(json!({ "error": "User not found" }));
        let response = (StatusCode::NOT_FOUND, body);
        return Ok(response.into_response());
    }

    if username == "najmiter" {
        return Ok(next.run(req).await);
    }

    let body = Json(json!({ "error": "Unauthorized access" }));
    let response = (StatusCode::UNAUTHORIZED, body);
    return Ok(response.into_response());

    // let auth_header = req
    //     .headers()
    //     .get(axum::http::header::AUTHORIZATION)
    //     .and_then(|header| header.to_str().ok());

    // match auth_header {
    //     Some(auth_val) if auth_val.starts_with("Bearer ") => {
    //         let token = auth_val.trim_start_matches("Bearer ");
    //         if DUMMY_USERS.iter().any(|user| user.token == token) {
    //             Ok(next.run(req).await)
    //         } else {
    //             Err(StatusCode::UNAUTHORIZED)
    //         }
    //     }
    //     _ => Err(StatusCode::UNAUTHORIZED),
    // }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(index)).route(
        "/users/:id",
        get(get_user_data).route_layer(middleware::from_fn(auth)),
    );

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
