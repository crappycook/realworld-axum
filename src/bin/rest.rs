use realworld::{config::AppConfig, router, server::AppState};
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

    let path = std::env::var("CONFIG_FILE").expect("CONFIG_FILE not found");
    let config = AppConfig::load_from_file(path);
    let state = AppState::new(config.clone()).await;

    // run it with hyper on localhost:8080
    let addr = config.server.get_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    let app = router::init_router(state);
    axum::serve(listener, app).await.unwrap();
}
