
CREATE TABLE chats (
    id                SERIAL PRIMARY KEY,
    name              VARCHAR(100),                    -- название
    types             VARCHAR(6) NOT NULL,          -- тип (перечень выше)
    image             VARCHAR(500),                      -- ссылка на аватар
    description       VARCHAR(500),                      -- описание
    community_id      INT,                       -- id сообщества
    creator_id        INT NOT NULL,              -- id создателя
    position          INT DEFAULT 0,             -- порядковый номер
    members           INT DEFAULT 0,             -- кол-во участников
    created           TIMESTAMP NOT NULL,        -- когда создан

    can_add_members   INT DEFAULT 1,                       -- кто добавляет участников
    can_fix_item      INT DEFAULT 3,                       -- кто закрепляет сообщения чата
    can_mention       INT DEFAULT 1,                       -- кто упоминает о чате
    can_add_admin     INT DEFAULT 3,                       -- кто работает с админами
    can_add_design    INT DEFAULT 2,                       -- кто работает с дизайном
    can_see_settings  INT DEFAULT 2,                       -- кто видит настройки
    can_see_log       INT DEFAULT 2,                       -- кто видит логи чата

    CONSTRAINT fk_chat_creator                   -- связь с создателем
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_chat_community                 -- связь с сообществом
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE chat_users (
    id                SERIAL PRIMARY KEY,            -- id объекта
    user_id           INT NOT NULL,                  -- id пользователя
    chat_id           INT NOT NULL,                  -- id чата
    types             VARCHAR(6) NOT NULL,              -- тип
    is_administrator  BOOLEAN NOT NULL DEFAULT false,-- админ ли?
    created           TIMESTAMP NOT NULL,            -- создано
    no_disturb        TIMESTAMP,                     -- не беспокоить до...

    CONSTRAINT fk_chat_users_user                    -- связь с пользователем
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_chat_users_chat                    -- связь с чатом
        FOREIGN KEY(chat_id)
            REFERENCES chats(id)
);

CREATE TABLE chat_ie_settings (
    id                SERIAL PRIMARY KEY,     -- id объекта
    user_id           INT NOT NULL,           -- id пользователя

    can_add_in_chat   INT NOT NULL DEFAULT 0, -- кто добавляет участников
    can_add_fix       INT NOT NULL DEFAULT 0, -- кто закрепляет сообщения
    can_send_mention  INT NOT NULL DEFAULT 0, -- кто упоминает о чате
    can_add_admin     INT NOT NULL DEFAULT 0, -- кто работает с админами
    can_add_design    INT NOT NULL DEFAULT 0, -- кто работает с дизайном
    can_see_settings  INT NOT NULL DEFAULT 0, -- кто видит настройки
    can_see_log       INT NOT NULL DEFAULT 0, -- кто видит логи

    CONSTRAINT fk_chat_ie_settings            -- связь с пользователем
        FOREIGN KEY(user_id)
            REFERENCES chat_users(id)
);

CREATE TABLE messages (
    id           SERIAL PRIMARY KEY,            -- id объекта
    creator_id   INT NOT NULL,                  -- id создателя
    chat_id      INT NOT NULL,                  -- id чата
    parent_id    INT,                           -- сообщение-родитель
    sticker_id   INT,                           -- id стикера
    repost_id    INT,                           -- id поста
    created      TIMESTAMP NOT NULL,            -- когда создано
    content      VARCHAR(5000),                          -- текст
    unread       BOOLEAN NOT NULL DEFAULT true, -- не прочитано?
    types        VARCHAR NOT NULL,              -- тип
    attach       VARCHAR(200),                          -- прикрепленные объекты
    voice        VARCHAR(500),                          -- ссылка на голосовое

    CONSTRAINT fk_message_creator               -- связь с создателем
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_message_chat                  -- связь с чатом
        FOREIGN KEY(chat_id)
          REFERENCES chats(id),

    CONSTRAINT fk_message_parent                -- связь с родителем (на какое ответ)
        FOREIGN KEY(parent_id)
          REFERENCES messages(id),

    CONSTRAINT fk_message_sticker                -- связь со стикером
        FOREIGN KEY(sticker_id)
          REFERENCES stickers(id),

    CONSTRAINT fk_message_post               -- связь с постом (репост в сообщения)
        FOREIGN KEY(repost_id)
          REFERENCES posts(id)
);

-- Копии сообщений перед изменением -------
CREATE TABLE message_versions (
    id SERIAL        PRIMARY KEY,           -- id объекта
    message_id       INT NOT NULL,                   -- id сообщения
    sticker_id       INT,                   -- id стикера
    repost_id        INT,                   -- id поста
    parent_id        INT,
    created          TIMESTAMP NOT NULL,    -- когда создано
    content          VARCHAR(5000),                  -- текст
    attach           VARCHAR(200),                  -- прикрепленные объекты

    CONSTRAINT fk_message_versions_message  -- связь с сообщением
        FOREIGN KEY(message_id)
          REFERENCES messages(id)
);


-- Особые сообщения для пользователей -------
CREATE TABLE message_options (
    id             SERIAL PRIMARY KEY,              -- id объекта
    message_id     INT NOT NULL,                             -- id сообщения
    user_id        INT NOT NULL,                             -- id пользователя
    is_deleted     BOOLEAN NOT NULL DEFAULT false,  -- сообщение удалено?
    is_favourite   BOOLEAN NOT NULL DEFAULT false,  -- сообщение в избранном?

    CONSTRAINT fk_message_options_message           -- связь с сообщением
        FOREIGN KEY(message_id)
          REFERENCES messages(id),

    CONSTRAINT fk_message_options_creator           -- связь с пользователем
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Пересланные сообщения -------
CREATE TABLE message_transfers (
    id          SERIAL PRIMARY KEY,            -- id объекта
    message_id  INT NOT NULL,                  -- id сообщения
    transfer_id INT NOT NULL,                  -- id пересылаемого сообщения

    CONSTRAINT fk_message_transfers_message    -- связь с сообщением
        FOREIGN KEY(message_id)
          REFERENCES messages(id),

    CONSTRAINT fk_message_transfers_transfer   -- связь с пересылаемым сообщением
        FOREIGN KEY(transfer_id)
            REFERENCES messages(id)
);
