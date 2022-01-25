#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

#[macro_use] extern crate rocket;

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}
impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("authorize");
        match token {
            Some(token) => {
                Outcome::Success(Token(token.to_string()))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
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
fn main(){
    rocket::ignite().mount("/", routes![coordinates]).launch();
}