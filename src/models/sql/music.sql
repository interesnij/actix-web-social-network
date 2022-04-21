CREATE TABLE sound_genres (
    id    SERIAL PRIMARY KEY,
    name  VARCHAR(100) NOT NULL,

    count INT NOT NULL,
    copy  INT NOT NULL
);

CREATE TABLE artists (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    image       VARCHAR(500),
    created     TIMESTAMP NOT NULL,

    count       INT NOT NULL,
    repost      INT NOT NULL,
    copy        INT NOT NULL,
    position    SMALLINT NOT NULL,

    can_see_el  "char" NOT NULL
);

CREATE TABLE music_albums (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    artist_id       INT,
    user_id         INT NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,

    count           INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,

    can_see_el      "char" NOT NULL,
    create_el       "char" NOT NULL,
    copy_el         "char" NOT NULL,

    CONSTRAINT fk_music_albums_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_albums_artist
        FOREIGN KEY(artist_id)
            REFERENCES artists(id)
);

CREATE TABLE music_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    user_id         INT NOT NULL,
    types           SMALLINT NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,

    count           INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,

    can_see_el      "char" NOT NULL,
    create_el       "char" NOT NULL,
    copy_el         "char" NOT NULL,

    CONSTRAINT fk_music_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE musics (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    community_id  INT,
    user_id       INT NOT NULL,
    music_list_id INT NOT NULL,
    genre_id      INT,
    album_id      INT,
    types         "char" NOT NULL,
    file          VARCHAR(500) NOT NULL,
    image         VARCHAR(500),
    created       TIMESTAMP NOT NULL,

    view          INT NOT NULL,
    repost        INT NOT NULL,
    copy          INT NOT NULL,
    position      SMALLINT NOT NULL,

    CONSTRAINT fk_music_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_music_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_music_list
        FOREIGN KEY(music_list_id)
            REFERENCES music_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_music_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    music_list_id INT NOT NULL,

   CONSTRAINT fk_user_music_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_music_list_collections_list
        FOREIGN KEY(music_list_id)
            REFERENCES music_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_music_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    music_list_id      INT NOT NULL,

   CONSTRAINT fk_community_music_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

   CONSTRAINT fk_community_music_list_collections_list
        FOREIGN KEY(music_list_id)
            REFERENCES music_lists(id)
);

CREATE TABLE music_list_perms (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    music_list_id INT NOT NULL,
    can_see_item  "char",
    create_item   "char",
    can_copy      "char",

   CONSTRAINT fk_music_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_music_list_perm_list
        FOREIGN KEY(music_list_id)
            REFERENCES music_lists(id)
);
