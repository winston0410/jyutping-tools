use crate::services;
use crate::routes::HttpError;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use rscantonese::RsCantonese;

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

    if RsCantonese::has_unknown(&results) {
        services::unknown::insert_unknown_sentence(&state.pool, &query.input);
        //Check if the unknown has been logged previously
        //
        //If not, save the unknown sentence to the database
    }

    Ok(HttpResponse::Ok().json(ConvertResponseBody { results }))
}

//NOTE Use GET as it is the only cacheable response
//REF https://developer.mozilla.org/en-US/docs/Glossary/cacheable
pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/parse").route("", web::get().to(parse_cantonese_characters)));
}
