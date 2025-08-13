use lazy_static::lazy_static;
use prometheus::{Encoder, Histogram, HistogramOpts, Registry, TextEncoder};

lazy_static! {
    /// Global metrics registry.
    pub static ref REGISTRY: Registry = Registry::new();
    /// Histogram tracking chunk streaming durations.
    pub static ref CHUNK_STREAM_HISTOGRAM: Histogram = Histogram::with_opts(HistogramOpts::new(
        "chunk_stream_duration_seconds",
        "Time spent streaming chunks"
    ))
    .expect("failed to create chunk histogram");
    /// Histogram tracking packet processing durations.
    pub static ref PACKET_PROCESS_HISTOGRAM: Histogram = Histogram::with_opts(HistogramOpts::new(
        "packet_process_duration_seconds",
        "Time spent processing packets"
    ))
    .expect("failed to create packet histogram");
}

/// Registers all default metrics with the global registry.
pub fn init_metrics() {
    // Registration errors are ignored because metrics may already be registered
    let _ = REGISTRY.register(Box::new(CHUNK_STREAM_HISTOGRAM.clone()));
    let _ = REGISTRY.register(Box::new(PACKET_PROCESS_HISTOGRAM.clone()));
}

/// Exports collected metrics in the Prometheus text format.
pub fn export_metrics() -> Result<String, prometheus::Error> {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer).expect("metrics not valid UTF-8"))
}
