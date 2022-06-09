CREATE TABLE video_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    position INT NOT NULL DEFAULT 0
);


CREATE TABLE video_lists (
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
    can_see_comment "char" NOT NULL,
    create_el       "char" NOT NULL,
    create_comment  "char" NOT NULL,
    copy_el         "char" NOT NULL,
    reactions       VARCHAR(100),

    CONSTRAINT fk_video_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_video_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE videos (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    community_id    INT,
    user_id         INT NOT NULL,
    video_list_id   INT NOT NULL,
    types           "char" NOT NULL,
    preview         VARCHAR(500),
    image           VARCHAR(500),
    file            VARCHAR(500) NOT NULL,
    description     VARCHAR(500),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT NOT NULL,
    view            INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,
    category_id     INT,
    reactions       INT NOT NULL,

    CONSTRAINT fk_videos_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_videos_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_videos_list
        FOREIGN KEY(video_list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_comments (
    id          SERIAL PRIMARY KEY,
    video_id    INT NOT NULL,
    user_id     INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    types       "char" NOT NULL,
    attach      VARCHAR(200),
    created     TIMESTAMP NOT NULL,
    repost      INT NOT NULL,
    reactions   INT NOT NULL,

    CONSTRAINT fk_video_comment
        FOREIGN KEY(video_id)
            REFERENCES videos(id),

    CONSTRAINT fk_user_video_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_sticker_video_comment
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id),

    CONSTRAINT fk_video_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES video_comments(id)
);
CREATE INDEX video_comments_video_id_idx ON video_comments (video_id);
CREATE INDEX video_comments_user_id_idx ON video_comments (user_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_video_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    video_list_id INT NOT NULL,

   CONSTRAINT fk_user_video_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_video_list_collections_list
        FOREIGN KEY(video_list_id)
            REFERENCES video_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_video_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    video_list_id       INT NOT NULL,

   CONSTRAINT fk_community_video_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

   CONSTRAINT fk_community_video_list_collections_list
        FOREIGN KEY(video_list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_list_perms (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    video_list_id         INT NOT NULL,
    can_see_item    "char",
    can_see_comment "char",
    create_item     "char",
    create_comment  "char",
    can_copy        "char",

   CONSTRAINT fk_video_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_video_list_perm_list
        FOREIGN KEY(video_list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_reactions (
    id         SERIAL PRIMARY KEY,
    video_id   INT NOT NULL,
    field_1    INT NOT NULL,
    field_2    INT NOT NULL,
    field_3    INT NOT NULL,
    field_4    INT NOT NULL,
    field_5    INT NOT NULL,
    field_6    INT NOT NULL,
    field_7    INT NOT NULL,
    field_8    INT NOT NULL,
    field_9    INT NOT NULL,
    field_10   INT NOT NULL,
    field_11   INT NOT NULL,
    field_12   INT NOT NULL,
    field_13   INT NOT NULL,
    field_14   INT NOT NULL,
    field_15   INT NOT NULL,
    field_16   INT NOT NULL,

    CONSTRAINT fk_video_reactions
        FOREIGN KEY(video_id)
            REFERENCES videos(id)
);

CREATE TABLE video_comment_reactions (
    id         SERIAL PRIMARY KEY,
    video_comment_id INT NOT NULL,
    field_1    INT NOT NULL,
    field_2    INT NOT NULL,
    field_3    INT NOT NULL,
    field_4    INT NOT NULL,
    field_5    INT NOT NULL,
    field_6    INT NOT NULL,
    field_7    INT NOT NULL,
    field_8    INT NOT NULL,
    field_9    INT NOT NULL,
    field_10   INT NOT NULL,
    field_11   INT NOT NULL,
    field_12   INT NOT NULL,
    field_13   INT NOT NULL,
    field_14   INT NOT NULL,
    field_15   INT NOT NULL,
    field_16   INT NOT NULL,

    CONSTRAINT fk_video_comment_reactions
        FOREIGN KEY(video_comment_id)
            REFERENCES video_comments(id)
);
