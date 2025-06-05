use std::sync::{Mutex, OnceLock};

pub static DBURL: OnceLock<Mutex<String>> = OnceLock::new();

pub fn database_url() -> &'static Mutex<String> {
    DBURL.get_or_init(|| {
        Mutex::new("postgres://nickforpsql:3y3vaye1@localhost/nickfordb".to_string())
    })
}
