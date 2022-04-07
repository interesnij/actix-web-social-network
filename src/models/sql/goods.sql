-- Категории товаров -------
CREATE TABLE good_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    avatar TEXT,
    order INT DEFAULT 0
);

-- Суб-категории товаров -------
CREATE TABLE good_subcategories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    category_id INT NOT NULL,
    avatar TEXT,
    order INT DEFAULT 0,

    CONSTRAINT fk_good_subcategories
        FOREIGN KEY(category_id)
            REFERENCES good_categories(id)
);

CREATE TABLE good_lists (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    description TEXT,
    created TIMESTAMP NOT NULL,
    count INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,

    can_see_el INT NOT NULL DEFAULT 1,
    can_see_comment INT NOT NULL DEFAULT 1,
    create_el INT NOT NULL DEFAULT 7,
    create_comment INT NOT NULL DEFAULT 1,
    copy_el INT NOT NULL DEFAULT 1,

    CONSTRAINT fk_good_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_good_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);


-- Фотографии -------
-- описание типов и вариантов приватностей
-- PUBLISHED, DELETED, CLOSED, MESSAGE = 'PUB','_DEL','_CLO','_MES'
--  TYPE = (
--      (PUBLISHED, 'Опубликовано'),(DELETED, 'Удалено'),(CLOSED, 'Закрыто модератором'),(MESSAGE, 'Загруженный в сообщениях'),
--  )

CREATE TABLE goods (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    community_id INT,
    category_id INT,
    creator_id INT NOT NULL,
    list_id INT NOT NULL,
    price INT,
    _type VARCHAR NOT NULL,
    preview TEXT NOT NULL,
    file TEXT NOT NULL,
    description TEXT,
    image TEXT,
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    votes_on BOOLEAN NOT NULL DEFAULT true,
    created TIMESTAMP NOT NULL,

    comment INT NOT NULL DEFAULT 0,
    view INT NOT NULL DEFAULT 0,
    liked INT NOT NULL DEFAULT 0,
    disliked INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_goods_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_goods_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_goods_category
        FOREIGN KEY(category_id)
            REFERENCES good_subcategories(id),

    CONSTRAINT fk_goods_list
        FOREIGN KEY(list_id)
            REFERENCES good_lists(id)
);

-- Комментарии к фотографиям -------
--EDITED, PUBLISHED, DRAFT = 'EDI', 'PUB', '_DRA'
--  DELETED, EDITED_DELETED = '_DEL', '_DELE'
--  CLOSED, EDITED_CLOSED = '_CLO', '_CLOE'
--  TYPE = (
--      (PUBLISHED, 'Опубликовано'),(EDITED, 'Изменённый'),(DRAFT, 'Черновик'),
--      (DELETED, 'Удалённый'), (EDITED_DELETED, 'Удалённый изменённый'),
--      (CLOSED, 'Закрытый менеджером'), (EDITED_CLOSED, 'Закрытый изменённый'),
--  )
CREATE TABLE good_comments (
    id SERIAL PRIMARY KEY,
    item_id INT NOT NULL,
    creator_id INT NOT NULL,
    sticker_id INT,
    parent_id INT,
    _text TEXT NOT NULL,
    attach VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL,

    liked INT NOT NULL DEFAULT 0,
    disliked INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_good_comment
        FOREIGN KEY(item_id)
            REFERENCES goods(id),

    CONSTRAINT fk_user_good_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_good_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES good_comments(id)
);
CREATE INDEX good_comments_item_id_idx ON good_comments (item_id);
CREATE INDEX good_comments_creator_id_idx ON good_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_good_list_collections (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,

   CONSTRAINT fk_user_good_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_good_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES good_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_good_list_collections (
    id SERIAL PRIMARY KEY,
    community_id INT,
    list_id INT,

   CONSTRAINT fk_community_good_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_good_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES good_lists(id)
);

CREATE TABLE good_list_perm (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,
    can_see_item INT DEFAULT 0,
    can_see_comment INT DEFAULT 0,
    create_item INT DEFAULT 0,
    create_comment INT DEFAULT 0,
    can_copy INT DEFAULT 0,

   CONSTRAINT fk_good_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_good_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES good_lists(id)
);
