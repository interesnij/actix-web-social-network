-- Your SQL goes here

CREATE TABLE good_images (
    id      SERIAL PRIMARY KEY,
    good_id INT NOT NULL,
    src     TEXT NOT NULL,

    CONSTRAINT fk_good_images
        FOREIGN KEY(good_id)
            REFERENCES goods(id)
);
CREATE INDEX good_images_id_idx ON good_images (good_id);
