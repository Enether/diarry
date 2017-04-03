CREATE TABLE diary_entries (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  creation_date DATE NOT NULL DEFAULT CURRENT_DATE)