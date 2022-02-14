use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::{LastModified, LAST_MODIFIED};
use actix_web::{middleware, web, App, Error, HttpServer};
use std::env;
mod middlewares;
mod routes;
mod services;
use pyo3::prelude::*;
use std::time::{Duration, SystemTime};

use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

// Use debug assertion for checking PYTHONPATH
//REF https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
const BUILD_TIME: &str = include!("/tmp/timestamp.txt");

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    if credentials.user_id() != &env::var("API_USER").unwrap() {
        return Err(routes::HttpError::Unauthorized {}.into());
    }

    if credentials.password().unwrap() != &env::var("API_PWD").unwrap() {
        return Err(routes::HttpError::Unauthorized {}.into());
    }
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env::var("API_USER").expect("You need to set env variable API_USER for Basic Auth's user");
    env::var("API_PWD").expect("You need to set env variable API_PWD for Basic Auth's password");
    env_logger::init();

    // Python::with_gil(|py| -> PyResult<()> {
    // py.run(
    // r#"
    // import sys
    // print(sys.executable, sys.path, sys.version)
    // "#,
    // None,
    // None,
    // )
    // .unwrap();
    // Ok(())
    // });
    //

    println!(
        "{}",
        format!("Starting microservice..., built at {}", BUILD_TIME)
    );

    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .wrap(if env::var("PRODUCTION").is_ok() {
                println!("Using restrictive CORS for production");
                Cors::default()
            } else {
                println!("Using permissive CORS for development");
                Cors::permissive()
            })
            .service(
                web::scope("/v1")
                    .wrap(middlewares::CacheHeader::default())
                    .wrap(middleware::DefaultHeaders::new().add((LAST_MODIFIED, BUILD_TIME)))
                    .configure(routes::convert::setup),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
