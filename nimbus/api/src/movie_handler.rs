use shared::response_models::{Response, ResponseBody};
use application::movie::read;
use application::user::read as read_user;
use application::genre::read as read_genre;
use domain::models::{Movie, User, Genre};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_movies_handler() -> String {
    let movies: Vec<Movie> = read::list_movies();
    let response = Response { body: ResponseBody::Movies(movies) };

    serde_json::to_string(&response).unwrap()
}

#[get("/movie/<movie_id>")]
pub fn list_movie_handler(movie_id: i32) -> Result<String, NotFound<String>> {
    let movie = read::list_movie(movie_id)?;
    let response = Response { body: ResponseBody::Movie(movie) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/users")]
pub fn list_users_handler() -> String {
    let users: Vec<User> = read_user::list_users();
    let response = Response { body: ResponseBody::Users(users) };

    serde_json::to_string(&response).unwrap()
}

#[get("/genres")]
pub fn list_genres_handler() -> String {
    let genres: Vec<Genre> = read_genre::list_genres();
    let response = Response { body: ResponseBody::Genres(genres) };

    serde_json::to_string(&response).unwrap()
}

//#[post("/new_movie", format = "application/json", data = "<movie>")]
//pub fn create_movie_handler(movie: Json<NewMovie>) -> Created<String> {
//    create::create_movie(movie)
//}
