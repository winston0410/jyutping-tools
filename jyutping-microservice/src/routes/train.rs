use crate::routes::HttpError;
use crate::services;
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use rscantonese::RsCantonese;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestBody {
    tokens: Vec<rscantonese::token::InputToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertResponseBody {
    results: rscantonese::Result,
}

pub async fn train_model(
    body: Result<web::Json<RequestBody>>,
    state: web::Data<AppData>,
) -> Result<HttpResponse, HttpError> {
    let unwrapped = body?;

    let mut rscantonese = state.rscantonese.lock().unwrap();
    rscantonese.train(&unwrapped.tokens);

    for token in unwrapped.tokens.iter() {
        //Insert the new token into database

        //Find and remove identical tokens from the database
        services::unknown::remove_unknown_sentence(&state.pool, &token.word).await;
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/train").route("", web::put().to(train_model)));
}
