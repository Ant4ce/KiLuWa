// @generated automatically by Diesel CLI.

diesel::table! {
    genres (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    key_relation_strength (id) {
        id -> Int4,
        keyword_id -> Int4,
        genre_id -> Int4,
    }
}

diesel::table! {
    keywords (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    movies (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        poster -> Nullable<Varchar>,
        rating -> Nullable<Int4>,
        release_year -> Nullable<Int4>,
    }
}

diesel::table! {
    movies_categorized (id) {
        id -> Int4,
        movie_id -> Int4,
        genre_id -> Int4,
    }
}

diesel::table! {
    user_ratings (id) {
        id -> Int4,
        rating -> Int4,
        user_id -> Int4,
        movie_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::joinable!(key_relation_strength -> genres (genre_id));
diesel::joinable!(key_relation_strength -> keywords (keyword_id));
diesel::joinable!(movies_categorized -> genres (genre_id));
diesel::joinable!(movies_categorized -> movies (movie_id));
diesel::joinable!(user_ratings -> movies (movie_id));
diesel::joinable!(user_ratings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    genres,
    key_relation_strength,
    keywords,
    movies,
    movies_categorized,
    user_ratings,
    users,
);
