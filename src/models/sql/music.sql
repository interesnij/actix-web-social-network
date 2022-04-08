CREATE TABLE sound_genres (
    id    SERIAL PRIMARY KEY,
    name  VARCHAR(100),

    count INT DEFAULT 0,
    copy  INT DEFAULT 0
);

CREATE TABLE artists (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    image       VARCHAR(500),
    created     TIMESTAMP NOT NULL,

    count       INT DEFAULT 0,
    repost      INT DEFAULT 0,
    copy        INT DEFAULT 0,

    can_see_el  INT DEFAULT 1,
    position    INT DEFAULT 0
);

CREATE TABLE music_albums (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    artist_id       INT,
    creator_id      INT NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,

    count           INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,

    can_see_el      INT DEFAULT 1,
    can_see_comment INT DEFAULT 1,
    create_el       INT DEFAULT 7,
    create_comment  INT DEFAULT 1,
    copy_el         INT DEFAULT 1,
    position        INT DEFAULT 0,

    CONSTRAINT fk_music_albums_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_albums_artist
        FOREIGN KEY(artist_id)
            REFERENCES artists(id)
);

CREATE TABLE music_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    types           VARCHAR(6) NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,

    count           INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,

    can_see_el      INT DEFAULT 1,
    can_see_comment INT DEFAULT 1,
    create_el       INT DEFAULT 7,
    create_comment  INT DEFAULT 1,
    copy_el         INT DEFAULT 1,
    position        INT DEFAULT 0,

    CONSTRAINT fk_music_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE music (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    community_id  INT,
    creator_id    INT NOT NULL,
    list_id       INT NOT NULL,
    genre_id      INT,
    album_id      INT,
    types         VARCHAR(6) NOT NULL,
    file          VARCHAR(500) NOT NULL,
    image         VARCHAR(500),
    created       TIMESTAMP NOT NULL,

    repost        INT DEFAULT 0,
    copy          INT DEFAULT 0,
    position      INT DEFAULT 0,
    view          INT DEFAULT 0,

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
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    list_id INT NOT NULL,

   CONSTRAINT fk_user_music_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_music_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_music_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    list_id      INT NOT NULL,

   CONSTRAINT fk_community_music_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_music_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);

CREATE TABLE music_list_perm (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    list_id       INT NOT NULL,
    can_see_item  INT DEFAULT 0,
    create_item   INT DEFAULT 0,
    can_copy      INT DEFAULT 0,

   CONSTRAINT fk_music_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_music_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES music_lists(id)
);
