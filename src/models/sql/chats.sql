-- чаты -------
-- описание типов и вариантов приватностей

--  PUBLIC,PRIVATE,MANAGER,GROUP = 'PUB','PRI','MAN','GRO'
--  DELETED_PUBLIC,DELETED_PRIVATE,DELETED_MANAGER,DELETED_GROUP = '_DPUB','_DPRI','_DMAN','_DGRO'
--  CLOSED_PUBLIC,CLOSED_PRIVATE,CLOSED_MANAGER,CLOSED_GROUP = '_CPUB','_CPRI','_CMAN','_CGRO'
--  SUPPORT_1, SUPPORT_2, SUPPORT_3, SUPPORT_4, SUPPORT_5 = "SUP1", "SUP2", "SUP3", "SUP4", "SUP5"
--  DELETED_SUPPORT_1, DELETED_SUPPORT_2, DELETED_SUPPORT_3, DELETED_SUPPORT_4, DELETED_SUPPORT_5 = "_SU1", "_SU2", "_SU3", "_SU4", "_SU5"
--  ALL_CAN, CREATOR, CREATOR_ADMINS, MEMBERS_BUT, SOME_MEMBERS = 1,2,3,4,5

--  TYPE = (
--      (PUBLIC, 'Публичный'),(PRIVATE, 'Приватный'),(MANAGER, 'Служебный'),(GROUP, 'Групповой'),
--      (DELETED_PUBLIC, 'удал Публичный'),(DELETED_PRIVATE, 'удал Приватный'),(DELETED_MANAGER, 'удал Служебный'),(DELETED_GROUP, 'удал Групповой'),
--      (CLOSED_PUBLIC, 'закр. Публичный'),(CLOSED_PRIVATE, 'закр. Приватный'),(CLOSED_MANAGER, 'закр. Служебный'),(CLOSED_GROUP, 'закр. Групповой'),
--      (SUPPORT_1, 'Техподдержка 1'),(SUPPORT_2, 'Техподдержка 2'),(SUPPORT_3, 'Техподдержка 3'),(SUPPORT_4, 'Техподдержка 4'),(SUPPORT_5, 'Техподдержка 5'),
--      (DELETED_SUPPORT_1, 'удал Тех 1'),(DELETED_SUPPORT_2, 'удал Тех 2'),(DELETED_SUPPORT_3, 'удал Тех 3'),(DELETED_SUPPORT_4, 'удал Тех 4'),(DELETED_SUPPORT_5, 'удал Тех 5'),
--  )
--  ALL_PERM = ((ALL_CAN, 'Все участники'),(CREATOR, 'Создатель'),(CREATOR_ADMINS, 'Создатель и админы'),(MEMBERS_BUT, 'Участники кроме'),(SOME_MEMBERS, 'Некоторые участники'),)

CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    _type VARCHAR NOT NULL,
    image TEXT,
    description TEXT,
    community_id INT,
    creator_id INT NOT NULL,
    order INT NOT NULL DEFAULT 0,
    members INT NOT NULL DEFAULT 0,
    attach TEXT,
    created TIMESTAMP NOT NULL,

    can_add_members INT,
    can_fix_item INT,
    can_mention INT,
    can_add_admin INT,
    can_add_design INT,
    can_see_settings INT,
    can_see_log INT,

    CONSTRAINT fk_chat_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id)
);

-- Члены сообщества -------
-- описание типов

--  ACTIVE, EXITED, DELETED = "ACT", "EXI", "DEL"
--  TYPE = (
--      (ACTIVE, 'Действующий'),(EXITED, 'Вышедший'),(DELETED, 'Уделенный'),
--  )

CREATE TABLE chat_users (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    chat_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    is_administrator BOOLEAN NOT NULL DEFAULT false,
    created TIMESTAMP NOT NULL,
    no_disturb TIMESTAMP,

    CONSTRAINT fk_chat_users_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_chat_users_chat
        FOREIGN KEY(chat_id)
            REFERENCES chats(id)
);

-- Исключения/Включения участников беседы -------
-- описание типов

--  NO_VALUE, YES_ITEM, NO_ITEM = 0, 1, 2
--  ITEM = (
--      (NO_VALUE, 'Не активно'),
--      (YES_ITEM, 'Может иметь действия с элементом'),
--      (NO_ITEM, 'Не может иметь действия с элементом'),
--  )

CREATE TABLE chat_ie_settings (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,

    can_add_in_chat INT NOT NULL DEFAULT 0,
    can_add_fix INT NOT NULL DEFAULT 0,
    can_send_mention INT NOT NULL DEFAULT 0,
    can_add_admin INT NOT NULL DEFAULT 0,
    can_add_design INT NOT NULL DEFAULT 0,
    can_see_settings INT NOT NULL DEFAULT 0,
    can_see_log INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_chat_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES chat_users(id)
);


-- Сообщения -------
-- описание типов

--  PUBLISHED, EDITED, DELETED, CLOSED, DRAFT, MANAGER, PUBLISHED_FIXED, EDITED_FIXED = 'PUB','EDI','_DEL','_CLO','_DRA','MAN','PFIX','EFIX'
--  DELETED_EDITED, CLOSED_EDITED, DELETED_PUBLISHED_FIXED, CLOSED_PUBLISHED_FIXED, DELETED_EDITED_FIXED, CLOSED_EDITED_FIXED = '_DELE','_CLOE', '_DELPF','_CLOPF', '_DELEF','_CLOEF'
--  TYPE = (
--      (PUBLISHED_FIXED, 'Закреп опубл'),(EDITED_FIXED, 'Закреп измен'),(MANAGER, 'Служебное'),(PUBLISHED, 'Опубл'),(DELETED, 'Удалено'),(EDITED, 'Изменено'),(CLOSED, 'Закрыто модератором'),(DRAFT, 'Черновик'),
--      (DELETED_EDITED_FIXED, 'Удал измен закреп'),(DELETED_PUBLISHED_FIXED, 'Удал опубл закреп'),(CLOSED_EDITED_FIXED, 'Закр измен закреп'),(CLOSED_PUBLISHED_FIXED, 'Закр опубл закреп'),(CLOSED_PUBLISHED_FIXED, 'Закр опубл закреп'),(DELETED_EDITED, 'Удал измен'),(CLOSED_EDITED, 'Закр измен'),
--  )

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    creator_id INT NOT NULL,
    chat_id INT NOT NULL,
    parent_id INT,
    sticker_id INT,
    repost_id INT,
    created TIMESTAMP NOT NULL,
    _text TEXT,
    unread BOOLEAN NOT NULL DEFAULT true,
    _type VARCHAR NOT NULL,
    attach TEXT,
    voice TEXT,

    CONSTRAINT fk_message_creator
        FOREIGN KEY(message_id)
            REFERENCES users(id),

    CONSTRAINT fk_message_chat
        FOREIGN KEY(chat_id)
          REFERENCES chats(id),

    CONSTRAINT fk_message_parent
        FOREIGN KEY(parent_id)
          REFERENCES messages(id)
);

-- Копии сообщений перед изменением -------
CREATE TABLE message_versions (
    id SERIAL PRIMARY KEY,
    message_id INT,
    sticker_id INT,
    repost_id INT,
    created TIMESTAMP NOT NULL,
    _text TEXT,
    attach TEXT,

    CONSTRAINT fk_message_versions_message
        FOREIGN KEY(message_id)
          REFERENCES messages(id)
);


-- Особые сообщения для пользователей -------
CREATE TABLE message_options (
    id SERIAL PRIMARY KEY,
    message_id INT,
    user_id INT,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    is_favourite BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT fk_message_options_message
        FOREIGN KEY(message_id)
          REFERENCES messages(id),

    CONSTRAINT fk_message_options_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Пересланные сообщения -------
CREATE TABLE message_transfers (
    id SERIAL PRIMARY KEY,
    message_id INT,
    transfer_id INT,

    CONSTRAINT fk_message_transfers_message
        FOREIGN KEY(message_id)
          REFERENCES messages(id),

    CONSTRAINT fk_message_transfers_transfer
        FOREIGN KEY(transfer_id)
            REFERENCES messages(id)
);
