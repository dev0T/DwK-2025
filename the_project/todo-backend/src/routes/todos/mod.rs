mod create;
mod get;

use actix_web::web::{self, ServiceConfig};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/todos")
            .route("", web::get().to(get::all))
            .route("", web::post().to(create::create)),
    );
}
