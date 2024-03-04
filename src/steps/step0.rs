use anyhow::Result;
use tracing::debug;

use crate::debug_dump;
use crate::steps::data::TEST_DATA;

pub fn run() -> Result<()> {
    debug!("Hello, World!");
    debug_dump!(TEST_DATA);
    Ok(())
}
