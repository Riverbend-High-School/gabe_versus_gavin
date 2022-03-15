#![allow(non_camel_case_types)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use rocket::{fs::NamedFile, tokio::sync::Mutex};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

mod poll;
mod util;

use crate::poll::Poll;

lazy_static! {
    static ref CURRENT_POLL: Arc<Mutex<Poll>> = Arc::new(Mutex::new(Poll::default()));
    static ref ALREADY_VOTED: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[rocket::main]
async fn main() {
    match rocket::build()
        .mount("/", routes![index, files])
        .mount(
            "/api",
            routes![
                poll::results,
                poll::vote,
                poll::admin,
                poll::activate,
                poll::deactivate,
                poll::check_vote_status,
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
