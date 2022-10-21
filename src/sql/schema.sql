-- DROP TABLE IF EXISTS bottlecaps;

CREATE TABLE IF NOT EXISTS bottlecaps (
  id serial PRIMARY KEY,
  user_id text NOT NULL,
  reason text NOT NULL,
  available boolean DEFAULT true,
  awarded text
);
