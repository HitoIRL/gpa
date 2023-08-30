mod cors;
mod kitchen;
mod errors;

use dotenvy::dotenv;
use rocket::{launch, routes};
use rocket::fs::FileServer;
use crate::cors::Cors;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Cors)
        .mount("/", routes![
            kitchen::routes::get_files,
            kitchen::routes::upload_file,
        ])
        .mount("/storage", FileServer::from("storage"))
}
