use std::path::PathBuf;

use crossterm::event::EventStream;
use synd_term::{
    application::Application,
    auth,
    cli::{self, Args},
    client::Client,
    terminal::Terminal,
    ui::theme::Theme,
};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;

fn init_tracing(log_path: PathBuf) -> anyhow::Result<WorkerGuard> {
    use synd_o11y::opentelemetry::init_propagation;
    use tracing_subscriber::{
        filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt as _, Registry,
    };

    // Open log file
    let (log, guard) = {
        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let log = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;
        tracing_appender::non_blocking(log)
    };

    Registry::default()
        .with(
            fmt::Layer::new()
                .with_ansi(true)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_file(false)
                .with_line_number(false)
                .with_target(true)
                .with_writer(log),
        )
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .try_init()?;

    // Set text map progator globally
    init_propagation();

    Ok(guard)
}

#[tokio::main]
async fn main() {
    let Args {
        endpoint,
        log,
        command,
        palette,
    } = cli::parse();

    let _guard = init_tracing(log).unwrap();

    #[allow(clippy::single_match)]
    match command {
        Some(cli::Command::Clear(clear)) => clear.run(),
        None => {}
    }

    let mut app = {
        let terminal = Terminal::new().expect("Failed to construct terminal");
        let client = Client::new(endpoint).expect("Failed to construct client");
        Application::new(terminal, client).with_theme(Theme::with_palette(&palette.into()))
    };

    if let Some(auth) = auth::credential_from_cache() {
        info!("Use authentication cache");
        app.set_credential(auth);
    }

    info!("Running...");

    if let Err(err) = app.run(&mut EventStream::new()).await {
        error!("{err}");
        std::process::exit(1);
    }
}
