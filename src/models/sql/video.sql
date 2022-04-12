CREATE TABLE video_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    position INT NOT NULL DEFAULT 0
);


CREATE TABLE video_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    types           "char" NOT NULL,
    description     VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    count           INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,
    position        SMALLINT DEFAULT 0,

    can_see_el      "char" NOT NULL,
    can_see_comment "char" NOT NULL,
    create_el       "char" NOT NULL,
    create_comment  "char" NOT NULL,
    copy_el         "char" NOT NULL,

    CONSTRAINT fk_video_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_video_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE videos (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    list_id         INT NOT NULL,
    types           "char" NOT NULL,
    preview         VARCHAR(500),
    image           VARCHAR(500),
    file            VARCHAR(500) NOT NULL,
    description     VARCHAR(500),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    votes_on        BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT DEFAULT 0,
    view            INT DEFAULT 0,
    liked           INT DEFAULT 0,
    disliked        INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,
    position        SMALLINT DEFAULT 0,

    CONSTRAINT fk_videos_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_videos_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_videos_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_comments (
    id          SERIAL PRIMARY KEY,
    item_id     INT NOT NULL,
    creator_id  INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    types       "char" NOT NULL,
    attach      VARCHAR(200),
    created     TIMESTAMP NOT NULL,

    liked       INT DEFAULT 0,
    disliked    INT DEFAULT 0,
    repost      INT DEFAULT 0,

    CONSTRAINT fk_video_comment
        FOREIGN KEY(item_id)
            REFERENCES videos(id),

    CONSTRAINT fk_user_video_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_sticker_video_comment
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id),

    CONSTRAINT fk_video_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES video_comments(id)
);
CREATE INDEX video_comments_item_id_idx ON video_comments (item_id);
CREATE INDEX video_comments_creator_id_idx ON video_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_video_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    list_id INT NOT NULL,

   CONSTRAINT fk_user_video_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_video_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_video_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    list_id       INT NOT NULL,

   CONSTRAINT fk_community_video_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_video_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_list_perm (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    list_id         INT NOT NULL,
    can_see_item    "char",
    can_see_comment "char",
    create_item     "char",
    create_comment  "char",
    can_copy        "char",

   CONSTRAINT fk_video_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_video_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);
