use actix_web::{middleware, web, App, HttpServer};
use std::env;
mod routes;
mod services;
use pyo3::prelude::*;

// Use debug assertion for checking PYTHONPATH
//REF https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // println!("Check equality {:?}", env::var("DEBUG_PYTHON").unwrap() == "1" );
    Python::with_gil(|py| -> PyResult<()> {
        py.run(
            r#"
import sys
print(sys.executable, sys.path, sys.version)
"#,
            None,
            None,
        )
        .unwrap();
        Ok(())
    });

    println!("Starting microservice...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/v1")
                .configure(routes::convert::setup)
                .configure(routes::segment::setup)
            )
            .service(web::scope("").configure(routes::health_check::setup))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
