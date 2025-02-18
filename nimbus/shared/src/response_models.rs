use domain::models::{Movie, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Movie(Movie),
    Movies(Vec<Movie>),
    User(User),
    Users(Vec<User>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
