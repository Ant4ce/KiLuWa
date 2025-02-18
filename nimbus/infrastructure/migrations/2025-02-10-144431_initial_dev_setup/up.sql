CREATE TABLE IF NOT EXISTS users (
  id INT GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS movies (
  id INT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(255) UNIQUE NOT NULL,
  description VARCHAR(255),
  poster Varchar(255),
  rating INT CHECK (rating >= 0) CHECK (rating <= 5),
  release_year INT,
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS genres (
  id INT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(255),
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS keywords (
  id INT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(255) UNIQUE NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS user_ratings (
  id INT GENERATED ALWAYS AS IDENTITY,
  rating INT NOT NULL,
  user_id INT NOT NULL,
  movie_id INT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_of_user_for_ratings FOREIGN KEY(user_id) REFERENCES users(id),
  CONSTRAINT fk_of_movie_for_ratings FOREIGN KEY(movie_id) REFERENCES movies(id)
);

CREATE TABLE IF NOT EXISTS movies_categorized (
  id INT GENERATED ALWAYS AS IDENTITY,
  movie_id INT NOT NULL,
  genre_id INT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_of_movie_for_categories FOREIGN KEY(movie_id) REFERENCES movies(id),
  CONSTRAINT fk_of_genre_for_categories FOREIGN KEY(genre_id) REFERENCES genres(id)
);

CREATE TABLE IF NOT EXISTS key_relation_strength (
  id INT GENERATED ALWAYS AS IDENTITY,
  keyword_id INT NOT NULL,
  genre_id INT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_of_keyword_for_relation_strength FOREIGN KEY(keyword_id) REFERENCES keywords(id),
  CONSTRAINT fk_of_genre_for_relation_strength FOREIGN KEY(genre_id) REFERENCES genres(id)
);
-- Your SQL goes here
