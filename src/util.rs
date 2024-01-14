//
// Logging
//

pub fn tracing_init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init()
}

#[macro_export]
macro_rules! debug_dump {
    ($data:expr) => {
        debug!("{:?}", pretty_hex::PrettyHex::hex_dump($data));
    };
}
