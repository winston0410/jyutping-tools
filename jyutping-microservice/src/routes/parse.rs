use crate::routes::HttpError;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestQuery {
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertResponseBody {
    results: rscantonese::Result,
}

//NOTE Request body is not used for GET, only request URI will be considered
//REF https://www.rfc-editor.org/rfc/rfc2616#section-9.3
pub async fn parse_cantonese_characters(
    query: web::Query<RequestQuery>,
    state: web::Data<AppData>,
) -> Result<HttpResponse, HttpError> {
    let results = state.rscantonese.parse(&query.input);

    Ok(HttpResponse::Ok().json(ConvertResponseBody { results }))
}

//NOTE Use GET as it is the only cacheable response
//REF https://developer.mozilla.org/en-US/docs/Glossary/cacheable
pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/parse").route("", web::get().to(parse_cantonese_characters)));
}
