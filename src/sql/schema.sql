-- DROP TABLE IF EXISTS bottlecaps;

CREATE TABLE IF NOT EXISTS bottlecaps (
    id serial PRIMARY KEY,
    user_id text NOT NULL,
    reason text NOT NULL,
    available boolean DEFAULT true,
    awarded text,
);

CREATE TABLE IF NOT EXISTS next_game (
    id serial PRIMARY KEY,
    date text
)
