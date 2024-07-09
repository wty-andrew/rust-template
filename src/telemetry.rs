use opentelemetry::{global, logs::LogError, trace::TraceError};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::LoggerProvider, metrics::SdkMeterProvider, propagation::TraceContextPropagator, runtime,
    trace as sdktrace,
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{
    fmt::MakeWriter, layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt,
    EnvFilter, Layer,
};

pub struct OtelGuard {
    meter_provider: SdkMeterProvider,
    logger_provider: LoggerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.logger_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
        global::shutdown_tracer_provider();
    }
}

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(sdktrace::config())
        .install_batch(runtime::Tokio)
}

fn init_meter_provider() -> opentelemetry::metrics::Result<SdkMeterProvider> {
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .build()
}

fn init_logger_provider() -> Result<LoggerProvider, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
}

pub fn fmt_layer<S, W>(make_writer: W) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber,
    for<'a> S: LookupSpan<'a>,
    W: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
{
    tracing_subscriber::fmt::layer()
        .with_writer(make_writer)
        .boxed()
}

pub fn init_telemetry() -> Result<OtelGuard, anyhow::Error> {
    let tracer = init_tracer()?;
    let meter_provider = init_meter_provider()?;
    let logger_provider = init_logger_provider()?;

    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_meter_provider(meter_provider.clone());

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt_layer(std::io::stdout))
        .with(OpenTelemetryLayer::new(tracer))
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryTracingBridge::new(&logger_provider))
        .init();

    Ok(OtelGuard {
        meter_provider,
        logger_provider,
    })
}
