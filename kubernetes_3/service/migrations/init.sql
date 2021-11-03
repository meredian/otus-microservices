CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY NOT NULL,
    username text,
    firstname text,
    lastname text,
    email text,
    phone text,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);
