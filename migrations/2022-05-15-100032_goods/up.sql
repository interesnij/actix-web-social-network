-- Категории товаров -------
CREATE TABLE good_categories (
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(100) NOT NULL,
    avatar    VARCHAR(500),
    position  SMALLINT NOT NULL
);

-- Суб-категории товаров -------
CREATE TABLE good_subcategories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(200) NOT NULL,
    category_id INT NOT NULL,
    avatar      VARCHAR(500),
    position    SMALLINT NOT NULL,

    CONSTRAINT fk_good_subcategories
        FOREIGN KEY(category_id)
            REFERENCES good_categories(id)
);

CREATE TABLE good_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    user_id         INT NOT NULL,
    types           SMALLINT NOT NULL,
    description     VARCHAR(500),
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

    CONSTRAINT fk_good_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_good_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE goods (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    community_id    INT,
    category_id     INT,
    user_id         INT NOT NULL,
    good_list_id    INT NOT NULL,
    price           INT,
    types           "char" NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    votes_on        BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT NOT NULL,
    view            INT NOT NULL,
    liked           INT NOT NULL,
    disliked        INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,

    CONSTRAINT fk_goods_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_goods_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_goods_category
        FOREIGN KEY(category_id)
            REFERENCES good_subcategories(id),

    CONSTRAINT fk_goods_list
        FOREIGN KEY(good_list_id)
            REFERENCES good_lists(id)
);

CREATE TABLE good_comments (
    id          SERIAL PRIMARY KEY,
    good_id     INT NOT NULL,
    user_id     INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    attach      VARCHAR(200),
    types       "char" NOT NULL,
    created     TIMESTAMP NOT NULL,

    liked       INT NOT NULL,
    disliked    INT NOT NULL,
    repost      INT NOT NULL,

    CONSTRAINT fk_good_comment
        FOREIGN KEY(good_id)
            REFERENCES goods(id),

    CONSTRAINT fk_user_good_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_sticker_good_comment
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id),

    CONSTRAINT fk_good_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES good_comments(id)
);
CREATE INDEX good_comments_good_id_idx ON good_comments (good_id);
CREATE INDEX good_comments_user_id_idx ON good_comments (user_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_good_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    good_list_id INT NOT NULL,

   CONSTRAINT fk_user_good_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_good_list_collections_list
        FOREIGN KEY(good_list_id)
            REFERENCES good_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_good_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    good_list_id INT NOT NULL,

   CONSTRAINT fk_community_good_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

   CONSTRAINT fk_community_good_list_collections_list
        FOREIGN KEY(good_list_id)
            REFERENCES good_lists(id)
);

CREATE TABLE good_list_perms (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    good_list_id    INT NOT NULL,
    can_see_item    "char",
    can_see_comment "char",
    create_item     "char",
    create_comment  "char",
    can_copy        "char",

   CONSTRAINT fk_good_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_good_list_perm_list
        FOREIGN KEY(good_list_id)
            REFERENCES good_lists(id)
);
-- Your SQL goes here