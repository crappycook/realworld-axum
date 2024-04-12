use realworld::{router, server::AppState};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "realworld=info,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
    tracing::info!("listening on {}", addr);
    let app = router::init_router(state);
    axum::serve(listener, app).await.unwrap();
}
