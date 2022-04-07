-- Списки фотографий -------
-- описание типов и вариантов приватностей

--  MAIN, LIST, WALL, AVATAR = 'MAI', 'LIS', 'WAL', 'AVA'
--  DELETED = '_DEL'
--  CLOSED, CLOSED_WALL, CLOSED_AVATAR, CLOSED_MAIN = '_CLO', '_CLOWA', '_CLOAM', '_CLOMA'
--  SUSPENDED, SUSPENDED_WALL, SUSPENDED_AVATAR, SUSPENDED_MAIN = '_SUS', '_SUSWA', '_SUSAV', '_SUSMA'
--  ALL_CAN,FRIENDS,EACH_OTHER,FRIENDS_BUT,SOME_FRIENDS,MEMBERS,CREATOR,ADMINS,MEMBERS_BUT,SOME_MEMBERS = 1,2,3,4,5,6,7,8,9,10
--  TYPE = (
--      (MAIN, 'Основной'),(LIST, 'Пользовательский'),(WALL, 'Фото со стены'),(AVATAR, 'Фото со страницы'),
--      (DELETED, 'Удалённый'),
--      (CLOSED, 'Закрытый менеджером'),(CLOSED_MAIN, 'Закрытый основной'),(CLOSED_WALL, 'Закрытый со стены'),(CLOSED_AVATAR, 'Закрытый со страницы'),
--      (SUSPENDED, 'Замороженный'),(SUSPENDED_MAIN, 'Замороженный основной'),(SUSPENDED_WALL, 'Замороженный со стены'),(SUSPENDED_AVATAR, 'Замороженный со страницы'),
--  )
--  PERM = (
--          (ALL_CAN, 'Все пользователи'),
--          (FRIENDS, 'Друзья'),
--          (EACH_OTHER, 'Друзья,друзья друзей'),
--          (CREATOR, 'Только я'),
--          (FRIENDS_BUT, 'Друзья, кроме'),
--          (SOME_FRIENDS, 'Некоторые друзья'),
--          (MEMBERS, 'Подписчики'),
--          (ADMINS, 'Администраторы'),
--          (MEMBERS_BUT, 'Подписчики, кроме'),
--          (SOME_MEMBERS, 'Некоторые подписчики'),
--          )

CREATE TABLE photo_lists (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    description TEXT,
    cover_photo TEXT,
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

    CONSTRAINT fk_photo_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_photo_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);


-- Фотографии -------
-- описание типов и вариантов приватностей
-- PUBLISHED, DELETED, CLOSED, MESSAGE = 'PUB','_DEL','_CLO','_MES'
--  TYPE = (
--      (PUBLISHED, 'Опубликовано'),(DELETED, 'Удалено'),(CLOSED, 'Закрыто модератором'),(MESSAGE, 'Загруженный в сообщениях'),
--  )

CREATE TABLE photos (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    list_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    preview TEXT NOT NULL,
    file TEXT NOT NULL,
    description TEXT,
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

-- Комментарии к фотографиям -------
--EDITED, PUBLISHED, DRAFT = 'EDI', 'PUB', '_DRA'
--  DELETED, EDITED_DELETED = '_DEL', '_DELE'
--  CLOSED, EDITED_CLOSED = '_CLO', '_CLOE'
--  TYPE = (
--      (PUBLISHED, 'Опубликовано'),(EDITED, 'Изменённый'),(DRAFT, 'Черновик'),
--      (DELETED, 'Удалённый'), (EDITED_DELETED, 'Удалённый изменённый'),
--      (CLOSED, 'Закрытый менеджером'), (EDITED_CLOSED, 'Закрытый изменённый'),
--  )
CREATE TABLE photo_comments (
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

    CONSTRAINT fk_photo_comment
        FOREIGN KEY(item_id)
            REFERENCES photos(id),

    CONSTRAINT fk_user_photo_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_photo_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES photo_comments(id)
);
CREATE INDEX photo_comments_item_id_idx ON photo_comments (item_id);
CREATE INDEX photo_comments_creator_id_idx ON photo_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_photo_list_collections (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,

   CONSTRAINT fk_user_photo_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_photo_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_photo_list_collections (
    id SERIAL PRIMARY KEY,
    community_id INT,
    list_id INT,

   CONSTRAINT fk_community_photo_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_photo_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);

CREATE TABLE photo_list_perm (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,
    can_see_item INT DEFAULT 0,
    can_see_comment INT DEFAULT 0,
    create_item INT DEFAULT 0,
    create_comment INT DEFAULT 0,
    can_copy INT DEFAULT 0,

   CONSTRAINT fk_photo_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_photo_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES photo_lists(id)
);
