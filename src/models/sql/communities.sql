-- Категории сообществ -------
CREATE TABLE community_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    avatar TEXT,
    order INT DEFAULT 0
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    category_id INT NOT NULL,
    avatar TEXT,
    order INT DEFAULT 0,

    CONSTRAINT fk_community_subcategories
        FOREIGN KEY(category_id)
            REFERENCES community_categories(id)
);

-- сообщества -------
-- описание типов и вариантов приватностей
--  PRIVATE, CLOSED, PUBLIC = 'PRI','CLO','PUB'
--  DELETED, PRIVATE_DELETED, CLOSED_DELETED = '_DELO', '_DELP', '_DELC'
--  BANNER_OPEN, BANNER_PRIVATE, BANNER_CLOSED = '_BANO', '_BANP', '_BANC'
--  SUSPENDED_OPEN, SUSPENDED_PRIVATE, SUSPENDED_CLOSED = '_SUSO', '_SUSP', '_SUSC'
--  BLOCKED_OPEN, BLOCKED_PRIVATE, BLOCKED_CLOSED = '_BLOO', '_BLOP', '_BLOC'
--  TYPE = (
--      (CLOSED, 'Закрытый'),(PRIVATE, 'Приватный'),(PUBLIC, 'Открытый'),
--      (DELETED, 'Открытый удалённый'),(PRIVATE_DELETED, 'Приватный удалённый'),(CLOSED_DELETED, 'Закрытый удалённый'),
--      (BANNER_OPEN, 'Открытый баннер'),(BANNER_PRIVATE, 'Приватный баннер'),(BANNER_CLOSED, 'Закрытый баннер'),
--      (SUSPENDED_OPEN, 'Открытый замороженный'),(SUSPENDED_PRIVATE, 'Приватный замороженный'), (SUSPENDED_CLOSED, 'Закрытый замороженный'),
--      (BLOCKED_OPEN, 'Открытый блокнутый'),(BLOCKED_PRIVATE, 'Приватный блокнутый'), (BLOCKED_CLOSED, 'Закрытый блокнутый'),
--  )

--  CHILD, STAND, VER_SEND, VER = 'CH', 'ST', 'VS', 'VE'
--  PERM = (
--      (CHILD, 'Детская'),(STAND, 'Обычные права'),(VER_SEND, 'Запрос на проверку'),(VER, 'Провернный'),
--  )

CREATE TABLE communities (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR NOT NULL,
    _type VARCHAR NOT NULL,
    perm VARCHAR NOT NULL,
    level INT DEFAULT 100,
    have_link VARCHAR,
    b_avatar TEXT,
    s_avatar TEXT,
    cover TEXT,
    category_id INT NOT NULL,
    creator_id INT NOT NULL,
    created TIMESTAMP NOT NULL,

    CONSTRAINT fk_community_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_community_category
        FOREIGN KEY(category_id)
            REFERENCES community_subcategories(id)
);


-- Члены сообщества -------
CREATE TABLE communities_memberships (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    community_id INT NOT NULL,
    is_administrator BOOLEAN NOT NULL DEFAULT false,
    is_moderator BOOLEAN NOT NULL DEFAULT false,
    is_editor BOOLEAN NOT NULL DEFAULT false,
    is_advertiser BOOLEAN NOT NULL DEFAULT false,
    created TIMESTAMP NOT NULL,
    visited INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_communities_memberships_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_communities_memberships_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

-- Исключения/Включения участников беседы -------
-- описание типов

--  NO_VALUE, YES_ITEM, NO_ITEM = 0, 1, 2
--  ITEM = (
--      (NO_VALUE, 'Не активно'),
--      (YES_ITEM, 'Может иметь действия с элементом'),
--      (NO_ITEM, 'Не может иметь действия с элементом'),
--  )

CREATE TABLE community_ie_settings (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,

    can_see_info INT NOT NULL DEFAULT 0,
    can_see_member INT NOT NULL DEFAULT 0,
    can_send_message INT NOT NULL DEFAULT 0,
    can_see_doc INT NOT NULL DEFAULT 0,
    can_see_music INT NOT NULL DEFAULT 0,
    can_see_survey INT NOT NULL DEFAULT 0,
    can_see_post INT NOT NULL DEFAULT 0,
    can_see_post_comment INT NOT NULL DEFAULT 0,
    can_see_photo INT NOT NULL DEFAULT 0,
    can_see_photo_comment INT NOT NULL DEFAULT 0,
    can_see_good INT NOT NULL DEFAULT 0,
    can_see_good_comment INT NOT NULL DEFAULT 0,
    can_see_video INT NOT NULL DEFAULT 0,
    can_see_video_comment INT NOT NULL DEFAULT 0,
    can_see_planner INT NOT NULL DEFAULT 0,
    can_see_planner_comment INT NOT NULL DEFAULT 0,

    can_add_post INT NOT NULL DEFAULT 0,
    can_add_photo INT NOT NULL DEFAULT 0,
    can_add_good INT NOT NULL DEFAULT 0,
    can_add_video INT NOT NULL DEFAULT 0,
    can_add_planner INT NOT NULL DEFAULT 0,
    can_add_doc INT NOT NULL DEFAULT 0,
    can_add_music INT NOT NULL DEFAULT 0,

    can_create_post INT NOT NULL DEFAULT 0,
    can_create_photo INT NOT NULL DEFAULT 0,
    can_create_good INT NOT NULL DEFAULT 0,
    can_create_video INT NOT NULL DEFAULT 0,
    can_create_planner INT NOT NULL DEFAULT 0,
    can_create_doc INT NOT NULL DEFAULT 0,
    can_create_music INT NOT NULL DEFAULT 0,
    can_create_survey INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_community_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES communities_memberships(id)
);

CREATE TABLE community_info (
    id SERIAL PRIMARY KEY,
    community_id INT NOT NULL,

    posts INT NOT NULL DEFAULT 0,
    members INT NOT NULL DEFAULT 0,
    photos INT NOT NULL DEFAULT 0,
    goods INT NOT NULL DEFAULT 0,
    tracks INT NOT NULL DEFAULT 0,
    videos INT NOT NULL DEFAULT 0,
    docs INT NOT NULL DEFAULT 0,
    articles INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_community_info
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

-- Настройки приватности пользователя -------
  --ALL_CAN,MEMBERSHIPS,CREATOR,STAFF,MEMBERSHIPS_BUT,SOME_MEMBERSHIPS = 1,2,3,4,5,6
  --PERM = (
  --    (ALL_CAN, 'Все пользователи'),
  --    (MEMBERSHIPS, 'Подписчики'),
  --    (CREATOR, 'Создатель'),
  --    (MEMBERSHIPS_BUT, 'Подписчики, кроме'),
  --    (SOME_MEMBERSHIPS, 'Некоторые подписчики'),
  --    (STAFF, 'Персонал'),
  --)
CREATE TABLE community_private (
    id                SERIAL PRIMARY KEY,
    community_id      INT NOT NULL,
    can_see_member    INT NOT NULL, -- Кто видит сообщества
    can_see_info      INT NOT NULL, -- Кто видит информацию
    can_send_message  INT NOT NULL, -- Кто пишет сообщения
    can_see_post      INT NOT NULL, -- Кто видит записи
    can_see_photo     INT NOT NULL, -- Кто видит фотографии
    can_see_good      INT NOT NULL, -- Кто видит товары
    can_see_video     INT NOT NULL, -- Кто видит видеозаписи
    can_see_music     INT NOT NULL, -- Кто видит аудиозапис
    can_see_planner   INT NOT NULL, -- Кто видит раздел планирования
    can_see_doc       INT NOT NULL, -- Кто видит документы
    can_see_survey    INT NOT NULL,  -- Кто видит опросы

    CONSTRAINT fk_community_private
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления сообщества -------
CREATE TABLE community_notifications (
    id                   SERIAL PRIMARY KEY,
    community_id         INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    community_invite     BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления записей -------
CREATE TABLE community_post_notifications (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,
    comment                 BOOLEAN NOT NULL DEFAULT true,
    comment_reply           BOOLEAN NOT NULL DEFAULT true,
    mention                 BOOLEAN NOT NULL DEFAULT true,
    comment_mention         BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,
    liked                   BOOLEAN NOT NULL DEFAULT true,
    disliked                BOOLEAN NOT NULL DEFAULT true,
    comment_liked           BOOLEAN NOT NULL DEFAULT true,
    comment_disliked        BOOLEAN NOT NULL DEFAULT true,
    comment_reply_liked     BOOLEAN NOT NULL DEFAULT true,
    comment_reply_disliked  BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_post_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления фотографий -------
CREATE TABLE community_photo_notifications (
    id                      SERIAL PRIMARY KEY,
    community_id            INT NOT NULL,
    comment                 BOOLEAN NOT NULL DEFAULT true,
    comment_reply           BOOLEAN NOT NULL DEFAULT true,
    mention                 BOOLEAN NOT NULL DEFAULT true,
    comment_mention         BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,
    liked                   BOOLEAN NOT NULL DEFAULT true,
    disliked                BOOLEAN NOT NULL DEFAULT true,
    comment_liked           BOOLEAN NOT NULL DEFAULT true,
    comment_disliked        BOOLEAN NOT NULL DEFAULT true,
    comment_reply_liked     BOOLEAN NOT NULL DEFAULT true,
    comment_reply_disliked  BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_photo_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления видеозаписей -------
CREATE TABLE community_video_notifications (
    id                      SERIAL PRIMARY KEY,
    community_id            INT NOT NULL,
    comment                 BOOLEAN NOT NULL DEFAULT true,
    comment_reply           BOOLEAN NOT NULL DEFAULT true,
    mention                 BOOLEAN NOT NULL DEFAULT true,
    comment_mention         BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,
    liked                   BOOLEAN NOT NULL DEFAULT true,
    disliked                BOOLEAN NOT NULL DEFAULT true,
    comment_liked           BOOLEAN NOT NULL DEFAULT true,
    comment_disliked        BOOLEAN NOT NULL DEFAULT true,
    comment_reply_liked     BOOLEAN NOT NULL DEFAULT true,
    comment_reply_disliked  BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_video_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления товаров -------
CREATE TABLE community_good_notifications (
    id                      SERIAL PRIMARY KEY,
    community_id            INT NOT NULL,
    comment                 BOOLEAN NOT NULL DEFAULT true,
    comment_reply           BOOLEAN NOT NULL DEFAULT true,
    mention                 BOOLEAN NOT NULL DEFAULT true,
    comment_mention         BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,
    liked                   BOOLEAN NOT NULL DEFAULT true,
    disliked                BOOLEAN NOT NULL DEFAULT true,
    comment_liked           BOOLEAN NOT NULL DEFAULT true,
    comment_disliked        BOOLEAN NOT NULL DEFAULT true,
    comment_reply_liked     BOOLEAN NOT NULL DEFAULT true,
    comment_reply_disliked  BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_good_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

-- Уведомления аудиозаписей -------
CREATE TABLE community_music_notifications (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    repost        BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_music_notifications
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);

------------------
------------------
-- Изменение порядка следования списков сообщества

-- Порядок следования фотоальбома -------
CREATE TABLE community_photo_list_position (
    id        SERIAL PRIMARY KEY,
    community INT DEFAULT 0,  -- Сообщество
    list      INT DEFAULT 0,  -- Фотоальбом
    position  INT DEFAULT 0,  -- Порядок отображения
    _type     INT DEFAULT 1   -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE community_post_list_position (
    id           SERIAL PRIMARY KEY,
    community    INT DEFAULT 0,     -- Сообщество
    list         INT DEFAULT 0,     -- Список записей
    position     INT DEFAULT 0,     -- Порядок отображения
    _type        INT DEFAULT 1      -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка аудиозаписей -------
CREATE TABLE community_music_list_position (
    id         SERIAL PRIMARY KEY,
    community  INT DEFAULT 0,       -- Сообщество
    list       INT DEFAULT 0,       -- Список аудиозаписей
    position   INT DEFAULT 0,       -- Порядок отображения
    _type      INT DEFAULT 1        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка товаров -------
CREATE TABLE community_good_list_position (
    id        SERIAL PRIMARY KEY,
    community INT DEFAULT 0,       -- Сообщество
    list      INT DEFAULT 0,       -- Список товаров
    position  INT DEFAULT 0,       -- Порядок отображения
    _type     INT DEFAULT 1        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка видеозаписей -------
CREATE TABLE community_video_list_position (
    id        SERIAL PRIMARY KEY,
    community INT DEFAULT 0,     -- Сообщество
    list      INT DEFAULT 0,     -- Список видеозаписей
    position  INT DEFAULT 0,     -- Порядок отображения
    _type     INT DEFAULT 1      -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка опросов -------
CREATE TABLE community_survey_list_position (
    id        SERIAL PRIMARY KEY,
    community INT DEFAULT 0,     -- Сообщество
    list      INT DEFAULT 0,     -- Список опросов
    position  INT DEFAULT 0,     -- Порядок отображения
    _type     INT DEFAULT 1      -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка документов -------
CREATE TABLE community_doc_list_position (
    id         SERIAL PRIMARY KEY,
    community  INT DEFAULT 0,     -- Сообщество
    list       INT DEFAULT 0,     -- Список документов
    position   INT DEFAULT 0,     -- Порядок отображения
    _type      INT DEFAULT 1      -- 1 - открыт, 0 - недоступен (например, удален)
);
