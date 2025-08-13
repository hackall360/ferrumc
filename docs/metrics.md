# Metrics and Profiling

FerrumC exposes runtime metrics using the [Prometheus](https://crates.io/crates/prometheus) crate. Metrics are gathered in a global registry and can be scraped in the Prometheus text format.

## Available Metrics

- `chunk_stream_duration_seconds` – histogram recording the time spent streaming chunks to clients.
- `packet_process_duration_seconds` – histogram recording the time spent processing incoming packet data.

These values allow operators to identify slow paths and observe latency distributions.

## Enabling Profiling

Run the server with the `--profiling` flag to record detailed tracing spans of hot paths. When enabled, profiling data is printed as JSON upon shutdown.

## Exporting Metrics

Call `ferrumc_utils::metrics::export_metrics()` to retrieve the current metrics in text format suitable for Prometheus scraping. The function returns a `Result<String, prometheus::Error>` containing the encoded metrics.

## Interpreting Histograms

Histogram metrics represent buckets of observed durations. High values in the tail buckets of `chunk_stream_duration_seconds` or `packet_process_duration_seconds` indicate performance bottlenecks in chunk streaming or packet handling respectively.

Regularly monitor these metrics to ensure that server throughput remains healthy and to identify regressions when deploying new code.
