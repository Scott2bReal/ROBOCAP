CREATE TABLE IF NOT EXISTS users (
  id int PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS bottlecaps (
  id int PRIMARY KEY,
  user_id int REFERENCES users (id)
    NOT NULL
    ON DELETE CASCADE,
  reason text NOT NULL,
  available boolean DEFAULT true
  awarded timestamp DEFAULT now()
);
