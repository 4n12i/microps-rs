//
// Logging
//

#[macro_export]
macro_rules! debug_dump {
    ($data:expr) => {
        debug!("{:?}", pretty_hex::PrettyHex::hex_dump($data));
    };
}
