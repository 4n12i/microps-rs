use crate::debug_dump;
use crate::test::data::TEST_DATA;
use crate::util::tracing_init;
use anyhow::Result;
use tracing::debug;

pub fn run() -> Result<()> {
    tracing_init();
    debug!("Hello, World!");
    debug_dump!(TEST_DATA);
    Ok(())
}
