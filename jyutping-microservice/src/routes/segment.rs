use crate::routes::HttpError;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SegmentResponseBody {
    results: Vec<String>,
}

pub async fn segment_cantonese(
    query: web::Query<RequestQuery>,
    state: web::Data<AppData>,
) -> Result<HttpResponse, HttpError> {
    let results = state.segmenter.segment(&query.input);

    Ok(HttpResponse::Ok().json(SegmentResponseBody { results }))
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/segment").route("", web::get().to(segment_cantonese)));
}
