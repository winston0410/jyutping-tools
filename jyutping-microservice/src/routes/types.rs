use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Error, Display, Serialize)]
#[display(fmt = "")]
struct HttpErrorBody {
    message: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "")]
pub enum HttpError {
    InvalidRequest { message: String },
    NotFound { message: String },
    ServerError {},
    Unauthorized {},
}

//REF https://stackoverflow.com/questions/40538554/is-it-possible-to-pattern-match-in-rust-with-multiple-types
impl From<actix_web::Error> for HttpError {
    fn from(e: actix_web::Error) -> Self {
        if let Some(err) = e.as_error::<actix_web::error::QueryPayloadError>() {
            return match err {
                _ => Self::ServerError {},
            };
        }

        if let Some(err) = e.as_error::<actix_web::error::JsonPayloadError>() {
            return match err {
                actix_web::error::JsonPayloadError::Deserialize(err) => {
                    Self::InvalidRequest { message: err.to_string() }
                },
                _ => Self::ServerError {}
            };
        }

        Self::ServerError {}
    }
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::InvalidRequest { message } => {
                return HttpResponseBuilder::new(self.status_code()).json(HttpErrorBody {
                    message: message.to_owned(),
                });
            }

            HttpError::NotFound { message } => {
                return HttpResponseBuilder::new(self.status_code()).json(HttpErrorBody {
                    message: message.to_owned(),
                })
            }

            HttpError::ServerError { .. } => {
                return HttpResponseBuilder::new(self.status_code()).finish()
            }

            HttpError::Unauthorized { .. } => {
                return HttpResponseBuilder::new(self.status_code()).finish()
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::InvalidRequest { .. } => StatusCode::BAD_REQUEST,

            HttpError::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,

            HttpError::NotFound { .. } => StatusCode::NOT_FOUND,

            HttpError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}
