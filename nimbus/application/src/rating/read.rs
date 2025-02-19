use domain::models::UserRating;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_user_rating(user_rating_id: i32) -> Result<UserRating, NotFound<String>> {
    use domain::schema::user_ratings;

    match user_ratings::table.find(user_rating_id).first::<UserRating>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", user_rating_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_user_ratings() -> Vec<UserRating> {
    use domain::schema::user_ratings;

    match user_ratings::table.select(user_ratings::all_columns).load::<UserRating>(&mut establish_postgres_connection()) {
        Ok(mut user_ratings) => {
            user_ratings.sort();
            user_ratings
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
