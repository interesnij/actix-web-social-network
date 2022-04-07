CREATE TABLE sound_genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR,

    count INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0
);

CREATE TABLE artists (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    description TEXT,
    image TEXT,
    created TIMESTAMP NOT NULL,

    count INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,

    can_see_el INT NOT NULL DEFAULT 1,
    order INT NOT NULL DEFAULT 0
);

CREATE TABLE music_albums (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    artist_id INT,
    creator_id INT NOT NULL,
    description TEXT,
    image TEXT,
    created TIMESTAMP NOT NULL,

    count INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,

    can_see_el INT NOT NULL DEFAULT 1,
    can_see_comment INT NOT NULL DEFAULT 1,
    create_el INT NOT NULL DEFAULT 7,
    create_comment INT NOT NULL DEFAULT 1,
    copy_el INT NOT NULL DEFAULT 1,
    order INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_music_albums_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_albums_artist
        FOREIGN KEY(artist_id)
            REFERENCES artists(id)
);

CREATE TABLE music_lists (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    description TEXT,
    image TEXT,
    created TIMESTAMP NOT NULL,

    count INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,

    can_see_el INT NOT NULL DEFAULT 1,
    can_see_comment INT NOT NULL DEFAULT 1,
    create_el INT NOT NULL DEFAULT 7,
    create_comment INT NOT NULL DEFAULT 1,
    copy_el INT NOT NULL DEFAULT 1,
    order INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_music_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE music (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    list_id INT NOT NULL,
    genre_id INT,
    album_id INT,
    _type VARCHAR NOT NULL,
    file TEXT NOT NULL,
    image TEXT,
    created TIMESTAMP NOT NULL,

    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,
    view INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_music_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_music_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_music_list_collections (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,

   CONSTRAINT fk_user_music_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_music_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_music_list_collections (
    id SERIAL PRIMARY KEY,
    community_id INT,
    list_id INT,

   CONSTRAINT fk_community_music_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_music_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);

CREATE TABLE music_list_perm (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,
    can_see_item INT DEFAULT 0,
    create_item INT DEFAULT 0,
    can_copy INT DEFAULT 0,

   CONSTRAINT fk_music_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_music_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);
