use crate::routes::HttpError;
use crate::services::{segment_chars, SegmentResult};
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SegmentResponseBody {
    results: SegmentResult,
}

#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    input: String,
}

#[derive(Debug)]
pub struct UnwrappedQuery {
    input: String,
}

impl From<RequestQuery> for UnwrappedQuery {
    fn from(orig: RequestQuery) -> Self {
        Self { input: orig.input }
    }
}

pub async fn segment(query: web::Query<RequestQuery>) -> Result<HttpResponse, HttpError> {
    let unwrapped_query = UnwrappedQuery::from(query.into_inner());

    let segmented = segment_chars(unwrapped_query.input.to_owned());

    Ok(HttpResponse::Ok().json(SegmentResponseBody { results: segmented }))
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/segment").route("", web::get().to(segment)));
}
