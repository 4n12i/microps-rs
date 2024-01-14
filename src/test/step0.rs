use crate::debug_dump;
use crate::test::data::TEST_DATA;
use anyhow::Result;
use tracing::debug;

pub fn run() -> Result<()> {
    debug!("Hello, World!");
    debug_dump!(TEST_DATA);
    Ok(())
}
