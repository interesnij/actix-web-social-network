CREATE TABLE phone_codes (
    id    SERIAL PRIMARY KEY,
    phone VARCHAR(14) NOT NULL,
    code  INT NOT NULL
);

CREATE TABLE custom_links (
    id   SERIAL PRIMARY KEY,
    link VARCHAR(100) NOT NULL
);
