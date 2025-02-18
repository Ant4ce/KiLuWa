#[macro_use] extern crate rocket;
use api::movie_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            movie_handler::list_movie_handler, 
            movie_handler::list_movies_handler,
            movie_handler::list_users_handler,
        ])
}
