use opentelemetry::sdk::export::trace::stdout;
use tracing::{error, span};
//use tracing_subscriber::subscriber::SubscriberExt;
use tracing_subscriber::Registry;

fn main() {
    // Install a new OpenTelemetry trace pipeline
    let (tracer, _uninstall) = stdout::new_pipeline().install();

    // Create a tracing subscriber with the configured tracer
    let telemetry = tracing_opentelemetry::subscriber().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let collector = Registry::default().with(telemetry);

    // Trace executed code
    tracing::collect::with_default(collector, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });
}