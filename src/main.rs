use anyhow::Result;

fn main() -> Result<()> {
    // Initial logging setting
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    microps_rs::steps::step1::run()
}
