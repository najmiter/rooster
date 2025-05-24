use axum::{Router, extract::ConnectInfo, response::Html, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

// change return type to `Html<String>` to let browser know we are sending html
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<!DOCTYPE html>
        <html>
            <head>
                <title>Hello, Axum!</title>
            </head>
            <body>
                <h1>Hello, Axum!</h1>
                <p>Your IP address is: {}</p>
                <p>Welcome to the Axum web server!</p>
            </body>
        </html>",
        addr.ip()
    );

    // create `Html` type like this
    Html(html)
}
