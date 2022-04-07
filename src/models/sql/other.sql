CREATE TABLE phone_codes (
    id SERIAL PRIMARY KEY,
    phone VARCHAR NOT NULL,
    code INT NOT NULL
);

CREATE TABLE custom_link (
    id SERIAL PRIMARY KEY,
    link VARCHAR NOT NULL
);
