use clap::Parser;
use sqlx::PgPool;
use tokio::signal;

use sandbox::app::create_app;
use sandbox::settings::Settings;
use sandbox::telemetry::init_telemetry;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8000;

#[derive(Parser)]
#[clap(version, disable_help_flag = true)]
struct Args {
    #[clap(short, long, default_value = DEFAULT_HOST)]
    host: String,

    #[clap(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to register Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received Ctrl+C, shutting down"),
        _ = terminate => tracing::info!("Received SIGTERM, shutting down"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);

    let _guard = init_telemetry()?;

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let settings = Settings::new()?;
    let db_pool = PgPool::connect(&settings.database_url).await?;
    let app = create_app(db_pool);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
