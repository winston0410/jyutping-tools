use crate::routes::HttpError;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    input: String,
}

type CharsToJyutpingResult = Vec<(String, Vec<String>)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertResponseBody {
    results: CharsToJyutpingResult,
}

//NOTE Request body is not used for GET, only request URI will be considered
//REF https://www.rfc-editor.org/rfc/rfc2616#section-9.3
pub async fn convert_chars_to_jyutping(
    query: web::Query<RequestQuery>,
    state: web::Data<AppData>,
) -> Result<HttpResponse, HttpError> {
    let results = state.segmenter.characters_to_jyutping(&query.input);

    Ok(HttpResponse::Ok().json(ConvertResponseBody { results }))
}

//NOTE Use GET as it is the only cacheable response
//REF https://developer.mozilla.org/en-US/docs/Glossary/cacheable
pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/convert").route("", web::get().to(convert_chars_to_jyutping)));
}
