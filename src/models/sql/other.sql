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
    position    SMALLINT NOT NULL DEFAULT 0,
    user_id     INT,
    description VARCHAR(200)
);

CREATE TABLE stickers (
    id                     SERIAL PRIMARY KEY,
    name                   VARCHAR(100),
    position               SMALLINT NOT NULL DEFAULT 0,
    sticker_categorie_id   INT,
    image                  VARCHAR(500),

    CONSTRAINT fk_stickers
        FOREIGN KEY(sticker_categorie_id)
            REFERENCES sticker_categories(id)
);

CREATE TABLE smile_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100),
    position    SMALLINT NOT NULL DEFAULT 0,
    description VARCHAR(200)
);

CREATE TABLE smiles (
    id                   SERIAL PRIMARY KEY,
    name                 VARCHAR(100),
    position             SMALLINT NOT NULL DEFAULT 0,
    smile_categorie_id   INT,
    image                VARCHAR(500),

    CONSTRAINT fk_smiles
        FOREIGN KEY(smile_categorie_id)
            REFERENCES smile_categories(id)
);
