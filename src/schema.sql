CREATE TABLE IF NOT EXISTS users (
  id text PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS bottlecaps (
  id serial PRIMARY KEY,
  user_id text REFERENCES users (id)
    ON DELETE CASCADE
    NOT NULL,
  reason text NOT NULL,
  available boolean DEFAULT true,
  awarded timestamp DEFAULT now()
);
