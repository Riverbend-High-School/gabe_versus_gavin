#[macro_export]
macro_rules! make_json_response {
    ($status:expr, $message:expr, $data:expr) => {
        Json(json!({
            "status": $status as i32,
            "message": $message,
            "data": $data,
        }).to_string())
    };
    ($status:expr, $message:expr) => {
        Json(json!({
            "status": $status as i32,
            "message": $message,
        }).to_string())
    }
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($r:expr, $s:expr) => {
        match $r {
            Ok(r) => r,
            Err(e) => {
                warn!("Unwrapped on error {} (error {})", e, $s);
                return None;
            }
        }
    };
    ($o:expr, $s:expr) => {
        match $o {
            Some(r) => r,
            None => {
                warn!("Unwrapped on None (error {})", $s);
                return None;
            }
        }
    };
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
