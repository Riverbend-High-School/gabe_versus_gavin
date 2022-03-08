use crate::{make_json_response, CURRENT_POLL};
use rocket::response::content::Json;
use rocket::serde::{self, Deserialize, Serialize};
use rocket::tokio::time::timeout;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct New_Poll {
    pub title: String,
    pub options: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<i32>,
    pub active: bool,
}
impl Poll {
    pub fn new(title: String, options: Vec<String>, active: bool) -> Self {
        let len = options.len();
        Self {
            title,
            options,
            votes: vec![0; len],
            active,
        }
    }

    pub fn replace(&mut self, new_poll: &New_Poll) {
        let len = new_poll.options.len();
        self.title = new_poll.title.clone();
        self.options = new_poll.options.clone();
        self.votes = vec![0; len];
        self.active = new_poll.active;
    }
}
impl Default for Poll {
    fn default() -> Self {
        Self {
            title: String::new(),
            options: Vec::new(),
            votes: Vec::new(),
            active: false,
        }
    }
}

#[get("/results")]
pub async fn results() -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };
    make_json_response!(200, "Success", current_poll.clone())
}

#[post("/vote/<option>")]
pub async fn vote(option: i32) -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let mut current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };

    if !current_poll.active {
        return make_json_response!(400, "Poll is not active");
    }

    if option < 0 || option >= current_poll.votes.len() as i32 {
        return make_json_response!(400, "Invalid option");
    }
    current_poll.votes[option as usize] += 1;

    make_json_response!(200, "OK")
}

#[post(
    "/verysecureadminpath",
    format = "application/json",
    data = "<new_poll>"
)]
pub async fn admin(new_poll: serde::json::Json<New_Poll>) -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let mut current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };
    current_poll.replace(&new_poll.into_inner());

    make_json_response!(200, "OK")
}

#[post("/verysecureadminpath/activate")]
pub async fn activate() -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let mut current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };
    current_poll.active = true;

    make_json_response!(200, "OK")
}

#[post("/verysecureadminpath/deactivate")]
pub async fn deactivate() -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let mut current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };
    current_poll.active = false;

    make_json_response!(200, "OK")
}
