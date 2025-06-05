use actix_web::{HttpResponse, web};
use std::error::Error;

pub async fn index() -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::Ok().body("SINGLE TONE DATA PATTERNS"))
}

pub fn configure_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}
