use core::fmt;
use pretty_hex::*;

//
// Logging
//

#[derive(Debug)]
enum Level {
    Error,
    Warn,
    Info,
    Debug,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "E",
            Self::Warn => "W",
            Self::Info => "I",
            Self::Debug => "D",
        };
        write!(f, "{s}")
    }
}

pub fn errorf(message: &str) {
    lprintf(Level::Error, message)
}

pub fn warnf(message: &str) {
    lprintf(Level::Warn, message)
}

pub fn infof(message: &str) {
    lprintf(Level::Info, message)
}

pub fn debugf(message: &str) {
    lprintf(Level::Debug, message)
}

fn lprintf(level: Level, message: &str) {
    // TODO: timestamp, function name
    println!("[{}] {} ({}:{})", level, message, file!(), line!());
}

pub fn debugdump(data: &[u8]) {
    println!("{:?}", data.hex_dump())
}
