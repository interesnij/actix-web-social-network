
CREATE TABLE photo_lists (
    id               SERIAL PRIMARY KEY,
    name             VARCHAR(100) NOT NULL,
    community_id     INT,
    creator_id       INT NOT NULL,
    types            "char" NOT NULL,
    description      VARCHAR(500),
    cover_photo      VARCHAR(500),
    created          TIMESTAMP NOT NULL,
    count            INT DEFAULT 0,
    repost           INT DEFAULT 0,
    copy             INT DEFAULT 0,
    position         INT DEFAULT 0,

    can_see_el       "char" NOT NULL,
    can_see_comment  "char" NOT NULL,
    create_el        "char" NOT NULL,
    create_comment   "char" NOT NULL,
    copy_el          "char" NOT NULL,

    CONSTRAINT fk_photo_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_photo_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);


CREATE TABLE photos (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    list_id         INT NOT NULL,
    types           "char" NOT NULL,
    preview         VARCHAR(500) NOT NULL,
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
    position        INT DEFAULT 0,

    CONSTRAINT fk_photos_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_photos_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_photos_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);


CREATE TABLE photo_comments (
    id          SERIAL PRIMARY KEY,
    item_id     INT NOT NULL,
    creator_id  INT NOT NULL,
    sticker_id  INT,
    parent_id   INT,
    content     VARCHAR(1000),
    attach      VARCHAR(200),
    created     TIMESTAMP NOT NULL,
    types       "char" NOT NULL,

    liked       INT DEFAULT 0,
    disliked    INT DEFAULT 0,
    repost      INT DEFAULT 0,

    CONSTRAINT fk_photo_comment
        FOREIGN KEY(item_id)
            REFERENCES photos(id),

    CONSTRAINT fk_user_photo_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_sticker_photo_comment
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id),

    CONSTRAINT fk_photo_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES photo_comments(id)
);
CREATE INDEX photo_comments_item_id_idx ON photo_comments (item_id);
CREATE INDEX photo_comments_creator_id_idx ON photo_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_photo_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    list_id INT NOT NULL,

   CONSTRAINT fk_user_photo_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_photo_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_photo_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    list_id       INT NOT NULL,

   CONSTRAINT fk_community_photo_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_photo_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);

CREATE TABLE photo_list_perm (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    list_id         INT NOT NULL,
    can_see_item    "char" NOT NULL,
    can_see_comment "char" NOT NULL,
    create_item     "char" NOT NULL,
    create_comment  "char" NOT NULL,
    can_copy        "char" NOT NULL,

   CONSTRAINT fk_photo_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_photo_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);
