use std::sync::{Mutex, OnceLock};

// const HOST: &str = "127.0.0.1:8000";

pub static HOSTCONFIG: OnceLock<Mutex<String>> = OnceLock::new();

pub fn host() -> &'static Mutex<String> {
    HOSTCONFIG.get_or_init(|| Mutex::new("127.0.0.1:8000".to_string()))
}
