CREATE TABLE phone_codes (
    id    SERIAL PRIMARY KEY,
    phone VARCHAR(14) NOT NULL,
    code  INT NOT NULL
);

CREATE TABLE custom_links (
    id   SERIAL PRIMARY KEY,
    link VARCHAR(100) NOT NULL
);

CREATE TABLE sticker_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100),
    position    INT NOT NULL DEFAULT 0,
    creator_id  INT,
    description VARCHAR(200)
);

CREATE TABLE stickers (
    id                     SERIAL PRIMARY KEY,
    name                   VARCHAR(100),
    position               INT NOT NULL DEFAULT 0,
    sticker_categories_id  INT,
    image                  VARCHAR(500),

    CONSTRAINT fk_stickers
        FOREIGN KEY(sticker_categories_id)
            REFERENCES sticker_categories(id)
);

CREATE TABLE smile_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100),
    position    INT NOT NULL DEFAULT 0,
    description VARCHAR(200)
);

CREATE TABLE smiles (
    id                   SERIAL PRIMARY KEY,
    name                 VARCHAR(100),
    position             INT NOT NULL DEFAULT 0,
    smile_categories_id  INT,
    image                VARCHAR(500),

    CONSTRAINT fk_smiles
        FOREIGN KEY(smile_categories_id)
            REFERENCES smile_categories(id)
);
