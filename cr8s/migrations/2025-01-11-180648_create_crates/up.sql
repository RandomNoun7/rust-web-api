CREATE TABLE crates (
    id SERIAL PRIMARY KEY,
    rustacean_id integer NOT NULL REFERENCES rustaceans(id),
    code VARCHAR(64),
    name VARCHAR(128),
    version VARCHAR(64),
    description text,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)