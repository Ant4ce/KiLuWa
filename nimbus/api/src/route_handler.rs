use shared::response_models::{Response, ResponseBody};
use application::movie::read as read_movie;
use application::user::read as read_user;
use application::genre::read as read_genre;
use application::keyword::read as read_keyword;
use application::relation::read as read_relation;
use application::categorized::read as read_category;
use application::rating::read as read_rating;
use domain::models::{Movie, User, Genre, Keyword, KeyRelationStrength, MovieCategorized, UserRating};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/movies")]
pub fn list_movies_handler() -> String {
    let movies: Vec<Movie> = read_movie::list_movies();
    let response = Response { body: ResponseBody::Movies(movies) };

    serde_json::to_string(&response).unwrap()
}

#[get("/movie/<movie_id>")]
pub fn list_movie_handler(movie_id: i32) -> Result<String, NotFound<String>> {
    let movie = read_movie::list_movie(movie_id)?;
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

#[get("/keywords")]
pub fn list_keywords_handler() -> String {
    let keywords: Vec<Keyword> = read_keyword::list_keywords();
    let response = Response { body: ResponseBody::Keywords(keywords) };

    serde_json::to_string(&response).unwrap()
}

#[get("/relations")]
pub fn list_relations_handler() -> String {
    let relations: Vec<KeyRelationStrength> = read_relation::list_key_relation_strengths();
    let response = Response { body: ResponseBody::Relations(relations) };

    serde_json::to_string(&response).unwrap()
}

#[get("/categorized")]
pub fn list_movies_categorized_handler() -> String {
    let categorized: Vec<MovieCategorized> = read_category::list_movies_categorized();
    let response = Response { body: ResponseBody::Categories(categorized) };

    serde_json::to_string(&response).unwrap()
}
#[get("/ratings")]
pub fn list_user_ratings_handler() -> String {
    let ratings: Vec<UserRating> = read_rating::list_user_ratings();
    let response = Response { body: ResponseBody::Ratings(ratings) };

    serde_json::to_string(&response).unwrap()
}
//#[post("/new_movie", format = "application/json", data = "<movie>")]
//pub fn create_movie_handler(movie: Json<NewMovie>) -> Created<String> {
//    create::create_movie(movie)
//}
