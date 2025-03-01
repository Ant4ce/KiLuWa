use crate::schema::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

//////////////////////////////////////////MOVIES//////////////////////////

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movie {
    pub id: i32, 
    pub name: String,
    pub description: Option<String>,
    pub poster: Option<String>,
    pub rating: Option<i32>,
    pub release_year: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = movies)]
pub struct NewMovie {
    pub name: String,
    pub description: Option<String>,
    pub poster: Option<String>,
    pub rating: Option<i32>,
    pub release_year: i32,
}

//////////////////////////////////GENRE//////////////////////////////////

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::genres)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Genre {
    pub id: i32, 
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32, 
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::keywords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Keyword {
    pub id: i32, 
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::key_relation_strength)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KeyRelationStrength {
    pub id: i32, 
    pub keyword_id: i32,
    pub genre_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::movies_categorized)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MovieCategorized {
    pub id: i32, 
    pub movie_id: i32,
    pub genre_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::user_ratings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRating {
    pub id: i32, 
    pub rating: i32,
    pub user_id: i32,
    pub movie_id: i32,
}

