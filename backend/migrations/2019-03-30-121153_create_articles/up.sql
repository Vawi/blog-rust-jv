-- Your SQL goes here
CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  discribe VARCHAR NOT NULL,
  body TEXT NOT NULL
)