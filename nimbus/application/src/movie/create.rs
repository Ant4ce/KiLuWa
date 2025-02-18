//use domain::models::{Movie, NewMovie};
//use shared::response_models::{Response, ResponseBody};
//use infrastructure::establish_postgres_connection;
//use diesel::prelude::*;
//use rocket::response::status::Created;
//use rocket::serde::json::Json;
//
//pub fn create_movie(movie: Json<NewMovie>) -> Created<String> {
//    use domain::schema::movies;
//
//    let movie = movie.into_inner();
//
//    match diesel::insert_into(movies::table).values(&movie).get_result::<Movie>(&mut establish_postgres_connection()) {
//        Ok(movie) => {
//            let response = Response { body: ResponseBody::Movie(movie) };
//            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
//        },
//        Err(err) => match err {
//            _ => {
//                panic!("Database error - {}", err);
//            }
//        }
//    }
//}
