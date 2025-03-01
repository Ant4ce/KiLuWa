#[macro_use] extern crate rocket;
use api::route_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            route_handler::list_movie_handler, 
            route_handler::create_movie_entry_handler,
            route_handler::list_movies_handler,
            route_handler::list_users_handler,
            route_handler::list_genres_handler,
            route_handler::list_keywords_handler,
            route_handler::list_relations_handler,
            route_handler::list_movies_categorized_handler,
            route_handler::list_user_ratings_handler,
        ])
}
