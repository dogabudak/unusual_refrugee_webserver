#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;

use rocket::request::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::json::Json;
use log::{warn, info};

#[macro_use] extern crate rocket;

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[derive(Serialize, Deserialize)]
struct Action {
    coordinate: (String, String),
    // TODO this should be an enum
    action_type: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("authorize");
        match token {
            Some(token) => {
                Outcome::Success(Token(token.to_string()))
            }
            None => {
                warn!("No token in request ");
                Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
            },
        }
    }
}
/*
  Returns the coordinates which are the only ones the user can see
*/
#[get("/coordinates")]
fn coordinates(authorize: Token)-> String{
    authorize.0
}

#[post("/", format = "json", data = "<input>")]
fn new(input: Json<Action>) -> &'static str {
    info!("Razor located: {}", input.action_type);
    "200 Okey Dokey"
}

fn main(){
    rocket::ignite().mount("/", routes![coordinates]).launch();
}