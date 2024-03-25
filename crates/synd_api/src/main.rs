use std::time::Duration;

use fdlimit::Outcome;
use synd_o11y::{
    metric,
    opentelemetry::OpenTelemetryGuard,
    tracing_subscriber::otel_metrics::{self, metrics_event_filter},
};
use tokio_metrics::RuntimeMonitor;
use tracing::{error, info};

use synd_api::{
    args::{self, Args, ObservabilityOptions},
    config,
    dependency::Dependency,
    monitor::Monitors,
    repository::kvsd::ConnectKvsdFailed,
    serve::listen_and_serve,
    shutdown::Shutdown,
};

fn init_tracing(options: &ObservabilityOptions) -> Option<OpenTelemetryGuard> {
    use synd_o11y::{
        opentelemetry::init_propagation,
        tracing_subscriber::{audit, otel_log, otel_trace},
    };
    use tracing_subscriber::{
        filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt as _, Layer as _,
        Registry,
    };

    let color = {
        use supports_color::Stream;
        supports_color::on(Stream::Stdout).is_some()
    };
    let show_src = options.show_code_location;
    let show_target = options.show_target;

    let (opentelemetry_layers, guard) = {
        match options.otlp_endpoint.as_deref() {
            None | Some("") => (None, None),
            Some(endpoint) => {
                let resource = synd_o11y::opentelemetry::resource(config::NAME, config::VERSION);

                let trace_layer =
                    otel_trace::layer(endpoint, resource.clone(), options.trace_sampler_ratio);
                let log_layer = otel_log::layer(endpoint, resource.clone());
                let metrics_layer = otel_metrics::layer(endpoint, resource);

                (
                    Some(trace_layer.and_then(log_layer).and_then(metrics_layer)),
                    Some(synd_o11y::opentelemetry::OpenTelemetryGuard),
                )
            }
        }
    };

    Registry::default()
        .with(
            fmt::Layer::new()
                .with_ansi(color)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_file(show_src)
                .with_line_number(show_src)
                .with_target(show_target)
                .with_filter(metrics_event_filter())
                .and_then(opentelemetry_layers)
                .with_filter(
                    EnvFilter::try_from_env("SYND_LOG")
                        .or_else(|_| EnvFilter::try_new("info"))
                        .unwrap()
                        .add_directive(audit::Audit::directive()),
                ),
        )
        .with(audit::layer())
        .init();

    // Set text map propagator globally
    init_propagation();

    guard
}

async fn run(
    Args {
        kvsd,
        bind,
        serve,
        tls,
        o11y,
    }: Args,
    shutdown: Shutdown,
    monitors: Monitors,
) -> anyhow::Result<()> {
    let dep = Dependency::new(kvsd, tls, serve, monitors).await?;

    info!(
        version = config::VERSION,
        otlp_endpoint=?o11y.otlp_endpoint,
        request_timeout=?dep.serve_options.timeout,
        request_body_limit_bytes=dep.serve_options.body_limit_bytes,
        concurrency_limit=?dep.serve_options.concurrency_limit,
        "Runinng...",
    );

    listen_and_serve(dep, bind.into(), shutdown).await
}

fn init_file_descriptor_limit() {
    fdlimit::raise_fd_limit()
        .inspect(|outcome| {
            match outcome {
                Outcome::LimitRaised { from, to } => {
                    tracing::info!("Raise fd limit {from} to {to}");
                }

                Outcome::Unsupported => tracing::info!("Raise fd limit unsupported"),
            };
        })
        .ok();
}

fn init_runtime_monitor() -> Monitors {
    let handle = tokio::runtime::Handle::current();
    let runtime_monitor = RuntimeMonitor::new(&handle);
    let task_monitors = Monitors::new();
    let intervals = runtime_monitor
        .intervals()
        .zip(task_monitors.gql.intervals());
    tokio::spawn(async move {
        for (runtime_metrics, gql_metrics) in intervals {
            // total_xxx metrics seems to not reset over interval(=monotonic)
            metric!(monotonic_counter.runtime.poll = runtime_metrics.total_polls_count);
            metric!(
                monotonic_counter.runtime.busy_duration =
                    runtime_metrics.total_busy_duration.as_secs_f64()
            );
            metric!(
                monotonic_counter.task.graphql.idle_duration =
                    gql_metrics.total_idle_duration.as_secs_f64()
            );

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    task_monitors
}

#[tokio::main]
async fn main() {
    let args = args::parse();
    let _guard = init_tracing(&args.o11y);
    let shutdown = Shutdown::watch_signal();
    let monitors = init_runtime_monitor();

    init_file_descriptor_limit();

    if let Err(err) = run(args, shutdown, monitors).await {
        if let Some(err) = err.downcast_ref::<ConnectKvsdFailed>() {
            error!("{err}: make sure kvsd is running");
        } else {
            error!("{err:?}");
        }
        std::process::exit(1);
    }
}
