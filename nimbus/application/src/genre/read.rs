use domain::models::Genre;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_genre(genre_id: i32) -> Result<Genre, NotFound<String>> {
    use domain::schema::genres;

    match genres::table.find(genre_id).first::<Genre>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", genre_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_genres() -> Vec<Genre> {
    use domain::schema::genres;

    match genres::table.select(genres::all_columns).load::<Genre>(&mut establish_postgres_connection()) {
        Ok(mut genres) => {
            genres.sort();
            genres
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
