// config
mod config;
use crate::config::config_host::host;
// database
mod db;
use crate::db::db_repo::{DbPool, db_connection};
use actix_web::{App, HttpServer, web};
// use std::sync::Arc;

// router
mod router;
use crate::router::index::configure_router;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let host_mutex = host();
    let host_guard = host_mutex.lock().unwrap();
    let host_addr: &str = &host_guard.as_str();
    println!("Server connection Established: {}", *host_guard);
    //db
    let pool: DbPool = db_connection();
    println!("Database Connection Established!");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_router)
    })
    .bind(host_addr)?
    .run()
    .await
}
