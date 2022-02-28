use actix_web::{web, HttpResponse, Responder};

async fn get() -> impl Responder {
    return HttpResponse::Ok();
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health-check").route("", web::get().to(get)));
}
