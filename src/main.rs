use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(hello2_handler));

    // REGION: ---> Start Server
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    println!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    // ENDREGION: ---> Start Server
}

// region: ---> handler hello

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g, `hello/name=?John`
async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("Hello {name}."))
}

//e.g, `hello/Jane`
async fn hello2_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {name:?}", "HANDLER");

    Html(format!("Hello2 {name}."))
}

// endregion: ---> handler hello
