use domain::models::{Movie, User, Genre};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Movie(Movie),
    Movies(Vec<Movie>),
    User(User),
    Users(Vec<User>),
    Genre(Genre),
    Genres(Vec<Genre>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
