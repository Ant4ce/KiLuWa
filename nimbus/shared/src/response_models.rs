use domain::models::{Movie, User, Genre, Keyword, KeyRelationStrength, MovieCategorized, UserRating};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Movie(Movie),
    Movies(Vec<Movie>),
    User(User),
    Users(Vec<User>),
    Genre(Genre),
    Genres(Vec<Genre>),
    Keyword(Keyword),
    Keywords(Vec<Keyword>),
    Relation(KeyRelationStrength),
    Relations(Vec<KeyRelationStrength>),
    Category(MovieCategorized),
    Categories(Vec<MovieCategorized>),
    Rating(UserRating),
    Ratings(Vec<UserRating>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
