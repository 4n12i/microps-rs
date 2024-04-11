pub mod intr;

// https://man7.org/linux/man-pages/man7/signal.7.html
// https://www.chromium.org/chromium-os/developer-library/reference/linux-constants/signals/
const SIGRTMIN: u8 = 34; /* First number of real-time signals */
pub const INTR_IRQ_BASE: u8 = SIGRTMIN + 1;
pub const INTR_IRQ_SHARED: u16 = 0x0001;
