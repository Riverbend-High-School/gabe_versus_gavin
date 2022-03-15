use crate::{make_json_response, ALREADY_VOTED, CURRENT_POLL};
use rocket::http::{Cookie, CookieJar};
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
    #[allow(dead_code)]
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

#[get("/results", format = "application/json")]
pub async fn results(cookies: &CookieJar<'_>) -> Json<String> {
    let five_seconds = std::time::Duration::from_secs(5);
    let current_poll = match timeout(five_seconds, CURRENT_POLL.lock()).await {
        Ok(poll) => poll,
        Err(_) => {
            return make_json_response!(500, "Could not get current poll. Please try again later.");
        }
    };
    if cookies.get("id").is_none() {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";
        const SESSION_LEN: usize = 60;

        let session_id: String = (0..SESSION_LEN)
            .map(|_| {
                let idx = rand::thread_rng().gen_range(0..CHARSET.len()); // must generate multiple thread_rng's for async functions
                CHARSET[idx] as char
            })
            .collect();
        cookies.add(Cookie::new("id", session_id.clone()));
    }
    make_json_response!(200, "Success", current_poll.clone())
}

#[post("/vote/<option>")]
pub async fn vote(option: i32, cookies: &CookieJar<'_>) -> Json<String> {
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

    if let Some(session_id) = cookies.get("id") {
        let mut already_voted = ALREADY_VOTED.lock().await;
        let session_id_str = session_id.to_string();
        if let Some(_) = already_voted
            .iter()
            .find(|session_id| **session_id == session_id_str)
        {
            return make_json_response!(400, "You have already voted");
        }

        if option < 0 || option >= current_poll.votes.len() as i32 {
            return make_json_response!(400, "Invalid option");
        }
        current_poll.votes[option as usize] += 1;
        already_voted.push(session_id_str);

        make_json_response!(200, "OK")
    } else {
        make_json_response!(400, "You don't have a voter ID!")
    }
}

#[post["/vote/check_vote_status"]]
pub async fn check_vote_status(cookies: &CookieJar<'_>) -> Json<String> {
    if let Some(session_id) = cookies.get("id") {
        let already_voted = ALREADY_VOTED.lock().await;
        let session_id_str = session_id.to_string();
        if let Some(_) = already_voted
            .iter()
            .enumerate()
            .find(|(_, session_id)| **session_id == session_id_str)
        {
            return make_json_response!(400, "You have already voted");
        }

        make_json_response!(200, "ID not found")
    } else {
        make_json_response!(200, "No ID set")
    }
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

    if new_poll.options.len() > 4 {
        return make_json_response!(400, "Too many options");
    } else if new_poll.options.len() == 0 {
        return make_json_response!(400, "Too few options");
    }
    let mut already_voted = ALREADY_VOTED.lock().await;
    *already_voted = Vec::new();

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
