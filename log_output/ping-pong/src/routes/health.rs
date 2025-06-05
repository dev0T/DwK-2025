use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Version", "0.0.1"))
        .finish()
}
