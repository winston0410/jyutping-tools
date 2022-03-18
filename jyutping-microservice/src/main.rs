use actix_cors::Cors;
use actix_web::http::header::LAST_MODIFIED;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
mod middlewares;
mod routes;
mod services;
use rscantonese::data::mock;
use rscantonese::RsCantonese;

// Use debug assertion for checking PYTHONPATH
//REF https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
const BUILD_TIME: &str = include!("/tmp/timestamp.txt");

pub struct AppData {
    rscantonese: RsCantonese,
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!(
        "{}",
        format!("Starting microservice..., built at {}", BUILD_TIME)
    );

    let mut rscantonese = RsCantonese::default();

    rscantonese.train(&mock());

    let state = web::Data::new(AppData {
        rscantonese,
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("You should set env variable DATABASE_URL."))
            .await
            .expect("Failed to initialize connection with database"),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(if env::var("PRODUCTION").is_ok() {
                println!("Using restrictive CORS for production");
                Cors::default().allowed_origin(
                    &env::var("ALLOWED_ORIGIN")
                        .expect("You should set env variable ALLOWED_ORIGIN in production."),
                )
            } else {
                println!("Using permissive CORS for development");
                Cors::permissive()
            })
            .service(
                web::scope("/v1")
                    .wrap(middlewares::CacheHeader::default())
                    .wrap(middleware::DefaultHeaders::new().add((LAST_MODIFIED, BUILD_TIME)))
                    .configure(routes::parse::setup)
                    .configure(routes::coverage::setup),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
