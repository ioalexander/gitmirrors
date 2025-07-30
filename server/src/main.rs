mod routes;
use dotenv::dotenv;
use rocket::tokio;

use crate::utils::catchers::{internal_error, not_found, unauthorized};
mod clone;
mod db;
mod middlewares;
mod models;
mod schema;
mod utils;
use futures::FutureExt;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::{panic::AssertUnwindSafe, time::Duration};

#[macro_use]
extern crate rocket;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let cors_urls = dotenv::var("CORS_URL").unwrap_or_default();
    let cors_origins: Vec<&str> = cors_urls
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let cors_allowed_origins = AllowedOrigins::some_exact(&cors_origins);
    let cors = CorsOptions {
        allowed_origins: cors_allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS fairing");

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    rocket::tokio::spawn({
        let pool = pool.clone();
        async move {
            loop {
                let result = AssertUnwindSafe(clone::worker::clone_worker_run(&pool))
                    .catch_unwind()
                    .await;

                match result {
                    Ok(Ok(())) => {
                        // all good
                    }

                    // clone_worker_run returned Err(e)
                    Ok(Err(e)) => {
                        eprintln!("clone_worker_run error: {:?}", e);
                    }

                    // it panicked
                    Err(panic_payload) => {
                        eprintln!("clone_worker_run panicked: {:?}", panic_payload);
                    }
                }

                // always sleep and loop again
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    rocket::build()
        .attach(cors)
        .manage(pool)
        .configure(
            rocket::Config::figment().merge((
                "port",
                dotenv::var("SERVER_PORT")
                    .expect("SERVER_PORT must be set")
                    .parse::<u16>()
                    .expect("SERVER_PORT must be a valid u16"),
            )),
        )
        .mount("/api/", routes![health])
        .mount("/api/", routes::routes())
        .register("/", catchers![not_found, internal_error, unauthorized])
}
