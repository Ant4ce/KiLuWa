use domain::models::Movie;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_movie(movie_id: i32) -> Result<Movie, NotFound<String>> {
    use domain::schema::movies;

    match movies::table.find(movie_id).first::<Movie>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", movie_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_movies() -> Vec<Movie> {
    use domain::schema::movies;

    match movies::table.select(movies::all_columns).load::<Movie>(&mut establish_postgres_connection()) {
        Ok(mut movies) => {
            movies.sort();
            movies
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
