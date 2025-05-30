use actix_web::{
    HttpResponse,
    web::{self, ServiceConfig},
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "v1.0"))
        .finish()
}
