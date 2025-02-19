use domain::models::MovieCategorized;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_movie_categorized(movie_categorized_id: i32) -> Result<MovieCategorized, NotFound<String>> {
    use domain::schema::movies_categorized;

    match movies_categorized::table.find(movie_categorized_id).first::<MovieCategorized>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", movie_categorized_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_movies_categorized() -> Vec<MovieCategorized> {
    use domain::schema::movies_categorized;

    match movies_categorized::table.select(movies_categorized::all_columns).load::<MovieCategorized>(&mut establish_postgres_connection()) {
        Ok(mut movies_categorized) => {
            movies_categorized.sort();
            movies_categorized
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

