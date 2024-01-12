use crate::net::NetDevice as DummyDevice;
use crate::net::*;
use crate::util::*;
use anyhow::bail;
use anyhow::Result;
use std::collections::LinkedList;

const DUMMY_MTU: u16 = u16::MAX; // Maximum size of IP datagram

impl NetDeviceOps for DummyDevice {
    fn open(&self) -> Result<()> {
        bail!("Not supported")
    }

    fn close(&self) -> Result<()> {
        bail!("Not supported")
    }

    fn transmit(&self, type_: u16, data: &[u8]) -> Result<()> {
        debugf(&format!(
            "dev={}, type=0x{:04x}, len={}",
            self.name,
            type_,
            data.len()
        ));
        debugdump(data);
        // Drop data
        Ok(())
    }
}

pub fn dummy_init(devs: &mut LinkedList<DummyDevice>) -> Result<()> {
    let dev = DummyDevice::new(
        0,
        NET_DEVICE_TYPE_DUMMY,
        DUMMY_MTU,
        NET_DEVICE_FLAG_UNSPECIFIED,
        None,
    );

    debugf(&format!("initialized, dev={}", dev.name));
    devs.push_front(dev);

    Ok(())
}
