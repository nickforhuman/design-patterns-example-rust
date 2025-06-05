//db url
use crate::config::config_db::database_url;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn db_connection() -> DbPool {
    let db_url_mutex = database_url();
    let db_url_guard = db_url_mutex.lock().unwrap();
    let db_url: &str = &db_url_guard.as_str();
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Database connection failed")
}
