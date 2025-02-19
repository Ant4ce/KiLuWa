use domain::models::KeyRelationStrength;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_key_relation_strength(key_relation_strength_id: i32) -> Result<KeyRelationStrength, NotFound<String>> {
    use domain::schema::key_relation_strength;

    match key_relation_strength::table.find(key_relation_strength_id).first::<KeyRelationStrength>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", key_relation_strength_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_key_relation_strengths() -> Vec<KeyRelationStrength> {
    use domain::schema::key_relation_strength;

    match key_relation_strength::table.select(key_relation_strength::all_columns).load::<KeyRelationStrength>(&mut establish_postgres_connection()) {
        Ok(mut key_relation_strength) => {
            key_relation_strength.sort();
            key_relation_strength
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
