#![allow(non_camel_case_types)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use rocket::{fs::NamedFile, tokio::sync::Mutex};
use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

mod poll;
mod util;

use crate::poll::Poll;

lazy_static! {
    static ref CURRENT_POLL: Arc<Mutex<Poll>> = Arc::new(Mutex::new(Poll::default()));
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    match rocket::build()
        .mount("/", routes![index,])
        .mount(
            "/api",
            routes![
                poll::results,
                poll::vote,
                poll::admin,
                poll::activate,
                poll::deactivate
            ],
        )
        .attach(crate::util::CORS)
        .launch()
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
