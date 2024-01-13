//
// Logging
//

pub fn tracing_init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init()
}
