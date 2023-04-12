use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
        .route("/other", get(other));

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let port = server.local_addr();
    println!("the port is: {port}");

    server.await.unwrap();
}

async fn root() {}
async fn other() {}