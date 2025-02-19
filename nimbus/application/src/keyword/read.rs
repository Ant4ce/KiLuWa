use domain::models::Keyword;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_postgres_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_keyword(keyword_id: i32) -> Result<Keyword, NotFound<String>> {
    use domain::schema::keywords;

    match keywords::table.find(keyword_id).first::<Keyword>(&mut establish_postgres_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", keyword_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_keywords() -> Vec<Keyword> {
    use domain::schema::keywords;

    match keywords::table.select(keywords::all_columns).load::<Keyword>(&mut establish_postgres_connection()) {
        Ok(mut keywords) => {
            keywords.sort();
            keywords
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
