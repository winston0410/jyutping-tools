use actix_cors::Cors;
use actix_web::http::header::{LastModified, LAST_MODIFIED};
use actix_web::{middleware, web, App, HttpServer};
use std::env;
mod middlewares;
mod routes;
mod services;
use pyo3::prelude::*;
use std::time::{Duration, SystemTime};

// Use debug assertion for checking PYTHONPATH
//REF https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust

const BUILD_TIME: &str = include!("/tmp/timestamp.txt");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
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
        App::new()
            .wrap(middleware::Logger::default())
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
                    .configure(routes::convert::setup)
                    .configure(routes::segment::setup),
            )
            .service(web::scope("").configure(routes::health_check::setup))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
