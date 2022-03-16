use crate::routes::HttpError;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

//TODO Extract and share RequestQuery
#[derive(Deserialize)]
pub struct RequestQuery {
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverageResponseBody {
    unknown_characters: Vec<String>,
}

//NOTE Request body is not used for GET, only request URI will be considered
//REF https://www.rfc-editor.org/rfc/rfc2616#section-9.3
pub async fn get_coverage(
    query: web::Query<RequestQuery>,
    state: web::Data<AppData>,
) -> Result<HttpResponse, HttpError> {
    let unknown_characters: Vec<String> = Vec::new();

    Ok(HttpResponse::Ok().json(CoverageResponseBody { unknown_characters }))
}

//NOTE Use GET as it is the only cacheable response
//REF https://developer.mozilla.org/en-US/docs/Glossary/cacheable
pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/coverage").route("", web::get().to(get_coverage)));
}
