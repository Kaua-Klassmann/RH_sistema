use tokio::net::TcpListener;

mod app;
mod configs;
mod connections;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let port = configs::get_app_config().port;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    let app = app::create_app().await;

    println!("Server running on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
