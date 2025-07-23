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
use std::time::Duration;

#[macro_use]
extern crate rocket;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    rocket::tokio::spawn({
        let pool = pool.clone();
        async move {
            loop {
                if let Err(e) = clone::worker::clone_worker_run(&pool).await {
                    eprintln!("clone_items error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    rocket::build()
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
