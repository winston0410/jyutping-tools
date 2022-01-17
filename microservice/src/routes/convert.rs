use crate::routes::HttpError;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
// use crate::services::{chars_to_jyutping, chars_to_yale};
use crate::services::{chars_to_jyutping, chars_to_yale, CharsToJyutpingResult};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JyutpingSystem {
    Jyutping,
    Yale,
}

#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    to: Option<JyutpingSystem>,
    input: String,
}

#[derive(Debug)]
pub struct UnwrappedQuery {
    to: JyutpingSystem,
    input: String,
}

impl From<RequestQuery> for UnwrappedQuery {
    fn from(orig: RequestQuery) -> Self {
        let to = orig.to.unwrap_or(JyutpingSystem::Jyutping);
        Self {
            to,
            input: orig.input,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertResponseBody {
    system_used: JyutpingSystem,
    results: CharsToJyutpingResult,
}

//NOTE Request body is not used for GET, only request URI will be considered
//REF https://www.rfc-editor.org/rfc/rfc2616#section-9.3
pub async fn convert_chars_to_jyutping(
    query: web::Query<RequestQuery>,
) -> Result<HttpResponse, HttpError> {
    let unwrapped_query = UnwrappedQuery::from(query.into_inner());

    let converted = match unwrapped_query.to {
        JyutpingSystem::Jyutping => chars_to_jyutping(unwrapped_query.input.to_owned()),
        JyutpingSystem::Yale => chars_to_yale(unwrapped_query.input.to_owned()),
    };

    Ok(HttpResponse::Ok().json(ConvertResponseBody {
        system_used: unwrapped_query.to,
        results: converted,
    }))
}

//NOTE Use GET as it is the only cacheable response
//REF https://developer.mozilla.org/en-US/docs/Glossary/cacheable
pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/convert").route("/chars", web::get().to(convert_chars_to_jyutping)));
}
