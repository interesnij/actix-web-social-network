CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    type VARCHAR NOT NULL,
    gender VARCHAR NOT NULL,
    device VARCHAR NOT NULL,
    language VARCHAR NOT NULL,
    perm VARCHAR NOT NULL,
    level INT default 100,
    password VARCHAR NOT NULL,
    have_link VARCHAR,
    status VARCHAR,
    b_avatar TEXT,
    s_avatar TEXT,
    email VARCHAR NOT NULL,

    UNIQUE(phone),
    UNIQUE(email)
);
