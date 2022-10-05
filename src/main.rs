#[macro_use]
extern crate rocket;

mod core;
mod http;
mod models;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init() {
    // init logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // init colored result logging
    color_eyre::install().unwrap();
}

#[launch]
fn rocket() -> _ {
    init();
    core::process_loop::start();
    rocket::build()
        .attach(http::middlware::Logger)
        .mount("/api/v1/", http::routes())
}
