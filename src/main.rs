use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("hello world from router") }));

    // REGION: ---> Start Server
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    println!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    //ENDREGION ---> Start server
}

// async fn hello_handler() -> () {}
