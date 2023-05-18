use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cors = tower_http::cors::CorsLayer::permissive();
    let app  = Router::new()
    .route("/simple_call",get(simple_call_handler))
    .layer(cors);
    let config = aws_config::load_from_env().await;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    println!("Hello, world!");
    Ok(())
}

async fn simple_call_handler() -> axum::response::Html<&'static str>{
    println!("Hello World! call");
    axum::response::Html("<h1>Hello World!</h1>")
}
