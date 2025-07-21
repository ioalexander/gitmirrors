mod routes;
use dotenv::dotenv;

use crate::utils::catchers::{internal_error, not_found, unauthorized};
mod db;
mod middlewares;
mod models;
mod schema;
mod utils;

#[macro_use]
extern crate rocket;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

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
