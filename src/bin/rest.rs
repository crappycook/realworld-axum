use std::time::Duration;

use realworld::{router, server::AppState};
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut opts = ConnectOptions::new(db_url);
    opts.max_connections(100);
    opts.connect_timeout(Duration::from_secs(8));
    opts.sqlx_logging(true);
    let conn = Database::connect(opts)
        .await
        .expect("Database connection failed");

    let state = AppState { db: conn };

    // run it with hyper on localhost:8080
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router::init_router(state))
        .await
        .unwrap();
}
