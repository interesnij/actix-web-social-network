CREATE TABLE post_categories (
    id    SERIAL PRIMARY KEY,
    name  VARCHAR(100),
    position INT NOT NULL DEFAULT 0
);

CREATE TABLE post_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    types           VARCHAR NOT NULL,
    description     VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    count           INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,
    position        INT DEFAULT 0,

    can_see_el      INT DEFAULT 1,
    can_see_comment INT DEFAULT 1,
    create_el       INT DEFAULT 7,
    create_comment  INT DEFAULT 1,
    copy_el         INT DEFAULT 1,

    CONSTRAINT fk_post_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_post_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE posts (
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(5000),
    community_id    INT,
    category_id     INT,
    creator_id      INT NOT NULL,
    list_id         INT NOT NULL,
    types           VARCHAR(6) NOT NULL,
    attach          VARCHAR(200),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    votes_on        BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT DEFAULT 0,
    view            INT DEFAULT 0,
    liked           INT DEFAULT 0,
    disliked        INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,
    position        INT DEFAULT 0,

    CONSTRAINT fk_posts_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_posts_category
        FOREIGN KEY(category_id)
            REFERENCES post_categories(id),

    CONSTRAINT fk_posts_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_posts_list
        FOREIGN KEY(list_id)
            REFERENCES post_lists(id)
);

CREATE TABLE post_comments (
    id          SERIAL PRIMARY KEY,
    item_id     INT NOT NULL,
    creator_id  INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    attach      VARCHAR(200),
    types       VARCHAR(6) NOT NULL,
    created     TIMESTAMP NOT NULL,

    liked       INT DEFAULT 0,
    disliked    INT DEFAULT 0,
    repost      INT DEFAULT 0,

    CONSTRAINT fk_post_comment
        FOREIGN KEY(item_id)
            REFERENCES posts(id),

    CONSTRAINT fk_user_post_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_post_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES post_comments(id)
);
CREATE INDEX post_comments_item_id_idx ON post_comments (item_id);
CREATE INDEX post_comments_creator_id_idx ON post_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_post_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    list_id INT NOT NULL,

   CONSTRAINT fk_user_post_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_post_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES post_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_post_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    list_id       INT NOT NULL,

   CONSTRAINT fk_community_post_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_post_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES post_lists(id)
);

-- включения и исключения для пользователей касательно конкретного списка записей -------
CREATE TABLE post_list_perm (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    list_id         INT NOT NULL,
    can_see_item    INT DEFAULT 0,
    can_see_comment INT DEFAULT 0,
    create_item     INT DEFAULT 0,
    create_comment  INT DEFAULT 0,
    can_copy        INT DEFAULT 0,

   CONSTRAINT fk_post_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_post_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES post_lists(id)
);
