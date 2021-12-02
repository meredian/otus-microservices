use lazy_static::lazy_static;
use prometheus::{HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new("response_code", "Response Codes"),
        &["env", "method", "path", "statuscode", "type"]
    )
    .expect("metric can be created");
    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(
        HistogramOpts::new("response_time", "Response Times"),
        &["env", "method", "path"]
    )
    .expect("metric can be created");
}

pub fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(RESPONSE_CODE_COLLECTOR.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(RESPONSE_TIME_COLLECTOR.clone()))
        .expect("collector can be registered");
}

pub fn track_request_time(response_time: f64, method: &str, path: &str, env: &str) {
    RESPONSE_TIME_COLLECTOR
        .with_label_values(&[env, method, path])
        .observe(response_time);
}

pub fn track_status_code(status_code: usize, method: &str, path: &str, env: &str) {
    let status_code_group = if status_code > 100 && status_code < 600 {
        (status_code / 100).to_string()
    } else {
        String::from("Unknown")
    };
    RESPONSE_CODE_COLLECTOR
        .with_label_values(&[
            env,
            method,
            path,
            &status_code.to_string(),
            &status_code_group,
        ])
        .inc();
}
