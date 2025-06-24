#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

pub mod logging {
    pub use super::log;
}
