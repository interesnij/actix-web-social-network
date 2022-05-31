-- Your SQL goes here

ALTER TABLE custom_links ADD COLUMN owner
SMALLINT NOT NULL DEFAULT 0;
