use crate::routes::HttpError;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::services::{segment_chars, SegmentResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SegmentResponseBody {
    results: SegmentResult
}

pub async fn segment(
    body: Result<web::Json<RequestBody>>,
) -> Result<HttpResponse, HttpError> {
    let unwrapped_body = body?;

    let segmented = segment_chars(unwrapped_body.text.to_owned());
    
    Ok(HttpResponse::Ok().json(SegmentResponseBody {
        results: segmented
    }))
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/segment")
            .route("", web::get().to(segment))
    );
}
