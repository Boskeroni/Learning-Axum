use axum::{routing::get, Router, Server, Json, response::IntoResponse};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root));

    let server = Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(router.into_make_service());
    let port = server.local_addr();
    println!("the port is: {port}");

    server.await.unwrap();
}

#[axum::debug_handler]
async fn root() -> impl IntoResponse {
    let mut all_files = Vec::new();
    let mut path = tokio::fs::read_dir("./").await.unwrap();

    while let Ok(Some(e)) = path.next_entry().await {
        let name = e.file_name().to_str().unwrap().to_string();
        all_files.push(name);
    }

    Json(all_files)
}