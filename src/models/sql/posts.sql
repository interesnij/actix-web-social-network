CREATE TABLE post_categories (
    id    SERIAL PRIMARY KEY,
    name  VARCHAR(100) NOT NULL,
    position SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE post_lists (
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

    CONSTRAINT fk_post_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_post_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE posts (
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(5000),
    community_id      INT,
    post_categorie_id INT,
    user_id           INT NOT NULL,
    post_list_id      INT NOT NULL,
    types           "char" NOT NULL,
    attach          VARCHAR(200),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT NOT NULL,
    view            INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,
    is_signature    BOOLEAN NOT NULL DEFAULT false,
    parent_id       INT,
    reactions       INT NOT NULL,

    CONSTRAINT fk_posts_parent
        FOREIGN KEY(parent_id)
            REFERENCES posts(id),

    CONSTRAINT fk_posts_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_posts_category
        FOREIGN KEY(post_categorie_id)
            REFERENCES post_categories(id),

    CONSTRAINT fk_posts_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_posts_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);

CREATE TABLE post_comments (
    id          SERIAL PRIMARY KEY,
    post_id     INT NOT NULL,
    user_id     INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    attach      VARCHAR(200),
    types       "char" NOT NULL,
    created     TIMESTAMP NOT NULL,
    repost      INT NOT NULL,
    reactions   INT NOT NULL,

    CONSTRAINT fk_post_comment
        FOREIGN KEY(post_id)
            REFERENCES posts(id),

    CONSTRAINT fk_user_post_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_sticker_post_comment
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id),

    CONSTRAINT fk_post_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES post_comments(id)
);
CREATE INDEX post_comments_post_id_idx ON post_comments (post_id);
CREATE INDEX post_comments_user_id_idx ON post_comments (user_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_post_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    post_list_id INT NOT NULL,

   CONSTRAINT fk_user_post_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_post_list_collections_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_post_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    post_list_id  INT NOT NULL,

   CONSTRAINT fk_community_post_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

   CONSTRAINT fk_community_post_list_collections_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);

-- включения и исключения для пользователей касательно конкретного списка записей -------
CREATE TABLE post_list_perms (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    post_list_id    INT NOT NULL,
    can_see_item    "char",
    can_see_comment "char",
    create_item     "char",
    create_comment  "char",
    can_copy        "char",

   CONSTRAINT fk_post_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_post_list_perm_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);

CREATE TABLE post_reactions (
    id         SERIAL PRIMARY KEY,
    post_id    INT NOT NULL,
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

    CONSTRAINT fk_post_reactions
        FOREIGN KEY(post_id)
            REFERENCES posts(id)
);

CREATE TABLE post_comment_reactions (
    id         SERIAL PRIMARY KEY,
    post_comment_id INT NOT NULL,
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

    CONSTRAINT fk_post_comment_reactions
        FOREIGN KEY(post_comment_id)
            REFERENCES post_comments(id)
);
