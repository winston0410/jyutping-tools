use crate::routes::HttpError;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::services::{chars_to_jyutping, chars_to_yale};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JyutpingSystem {
    Jyutping,
    Yale,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    to: Option<JyutpingSystem>,
}

#[derive(Debug)]
pub struct UnwrappedRequestQuery {
    to: JyutpingSystem,
}

impl From<RequestQuery> for UnwrappedRequestQuery {
    fn from(orig: RequestQuery) -> Self {
        let to = orig.to.unwrap_or(JyutpingSystem::Jyutping);
        Self { to }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertResponseBody {
    system_used: JyutpingSystem,
    converted_text: String 
}

pub async fn convert_chars_to_jyutping(
    body: Result<web::Json<RequestBody>>,
    query: web::Query<RequestQuery>,
) -> Result<HttpResponse, HttpError> {
    let unwrapped_body = body?;
    let unwrapped_query = UnwrappedRequestQuery::from(query.into_inner());

    let converted = match unwrapped_query.to {
        JyutpingSystem::Jyutping => chars_to_jyutping(unwrapped_body.text.to_owned()),
        JyutpingSystem::Yale => chars_to_yale(unwrapped_body.text.to_owned()),
    };

    Ok(HttpResponse::Ok().json(ConvertResponseBody {
        system_used: unwrapped_query.to,
        converted_text: converted
    }))
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/convert")
            .route("/chars", web::get().to(convert_chars_to_jyutping))
            // .route("", web::post().to(new_employee))
            // .service(
                // web::scope("/{id}")
                    // .route("", web::get().to(get_employee))
                    // .route("", web::put().to(update_employee))
                    // .route("", web::delete().to(delete_employee))
                    // .service(
                        // web::scope("/sessions")
                            // .route("", web::get().to(list_employee_sessions))
                            // .service(
                                // web::scope("/{sid}")
                                    // .route("", web::put().to(assign_to_session))
                                    // .route("", web::delete().to(remove_from_session)),
                            // ),
                    // ),
            // ),
    );
}
