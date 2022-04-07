-- Списки документов -------
-- описание типов и вариантов приватностей

--  MAIN, LIST, DELETED, CLOSED, CLOSED_MAIN, SUSPENDED, SUSPENDED_MAIN = 'MAI','LIS','_DEL','_CLO','_CLOMA','_SUS','_SUSMA'
--  ALL_CAN,FRIENDS,EACH_OTHER,FRIENDS_BUT,SOME_FRIENDS,MEMBERS,CREATOR,ADMINS,MEMBERS_BUT,SOME_MEMBERS = 1,2,3,4,5,6,7,8,9,10
--  TYPE = (
--      (MAIN, 'Основной'),(LIST, 'Пользовательский'),
--      (DELETED, 'Удалённый'),
--      (CLOSED_MAIN, 'Закрытый основной'),(CLOSED, 'Закрытый'),
--      (SUSPENDED_MAIN, 'Замороженный основной'),(SUSPENDED, 'Замороженный'),
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

CREATE TABLE doc_lists (
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

    can_see_el INT NOT NULL DEFAULT 1,
    can_see_comment INT NOT NULL DEFAULT 1,
    create_el INT NOT NULL DEFAULT 7,
    create_comment INT NOT NULL DEFAULT 1,
    copy_el INT NOT NULL DEFAULT 1,
    order INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_doc_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_doc_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);


-- Документы -------
-- описание типов и вариантов приватностей
--PUBLISHED, DELETED, CLOSED = 'PUB','_DEL','_CLO'
--  BOOK, ARTICLE, PUBLIC, FILE, OTHER = 'BOO','ART','PU','FIL','OTH'
--  TYPE = (
--      (PUBLISHED, 'Опубликовано'),(DELETED, 'Удалено'),(CLOSED, 'Закрыто модератором'),
--  )
--  TYPE_2 = (
--      (BOOK, 'Книга'),(ARTICLE, 'Статья'),(PUBLIC, 'Заметка'),(FILE, 'Файл'),(OTHER, 'Другое'),
--  )

CREATE TABLE docs (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    list_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    _type_2 VARCHAR NOT NULL,
    file TEXT NOT NULL,
    created TIMESTAMP NOT NULL,

    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,
    view INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_docs_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_docs_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_docs_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_doc_list_collections (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,

   CONSTRAINT fk_user_doc_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_doc_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_doc_list_collections (
    id SERIAL PRIMARY KEY,
    community_id INT,
    list_id INT,

   CONSTRAINT fk_community_doc_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_doc_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);

CREATE TABLE doc_list_perm (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,
    can_see_item INT DEFAULT 0,
    create_item INT DEFAULT 0,
    can_copy INT DEFAULT 0,

   CONSTRAINT fk_doc_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_doc_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);
