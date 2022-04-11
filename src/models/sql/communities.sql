-- Категории сообществ -------
CREATE TABLE community_categories (
    id SERIAL PRIMARY KEY,  -- id объекта
    name VARCHAR(100),           -- название
    avatar VARCHAR(500),            -- аватар
    position INT DEFAULT 0     -- порядковый номер
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategories (
    id          SERIAL PRIMARY KEY,       -- id объекта
    name        VARCHAR(100),                  -- название
    category_id INT NOT NULL,             -- id категории
    avatar      VARCHAR(500),                     -- аватар
    position    INT DEFAULT 0,            -- порядковый номер

    CONSTRAINT fk_community_subcategories -- связь с категорией
        FOREIGN KEY(category_id)
            REFERENCES community_categories(id)
);

CREATE TABLE communities (
    id            SERIAL PRIMARY KEY,     -- id объекта
    name          VARCHAR(100) NOT NULL,  -- название
    description   VARCHAR(500),           -- описание
    status        CHAR(1) NOT NULL,          -- статус
    types         SMALLINT NOT NULL,      -- тип
    perm          CHAR(1) NOT NULL,          -- приватность
    level         SMALLINT DEFAULT 100,   -- уровень доверия
    have_link     VARCHAR(100),           -- красивая ссылка
    b_avatar      VARCHAR(500),           -- большой аватар
    s_avatar      VARCHAR(500),           -- маленький аватар
    cover         VARCHAR(500),           -- баннер
    category_id   INT NOT NULL,           -- id категории
    creator_id    INT NOT NULL,           -- id создателя
    created       TIMESTAMP NOT NULL,     -- когда создано

    CONSTRAINT fk_community_creator   -- связь с пользователем
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_community_category  -- связь с категорией
        FOREIGN KEY(category_id)
            REFERENCES community_subcategories(id)
);


-- Члены сообщества -------
CREATE TABLE communities_memberships (
    id                SERIAL PRIMARY KEY,             -- id объекта
    user_id           INT NOT NULL,                   -- id пользователя
    community_id      INT NOT NULL,                   -- id сообщества
    is_administrator  BOOLEAN NOT NULL DEFAULT false, -- админ?
    is_moderator      BOOLEAN NOT NULL DEFAULT false, -- Модератор?
    is_editor         BOOLEAN NOT NULL DEFAULT false, -- Редактор?
    is_advertiser     BOOLEAN NOT NULL DEFAULT false, -- Рекламщик?
    created           TIMESTAMP NOT NULL,             -- Создано
    visited           INT NOT NULL DEFAULT 0,         -- Визиты в сообщество

    CONSTRAINT fk_communities_memberships_user        -- связь с пользователем
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_communities_memberships_community   -- связь с сообществом
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE community_ie_settings (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,

    can_see_info            CHAR(1) NOT NULL,
    can_see_member          CHAR(1) NOT NULL,
    can_send_message        CHAR(1) NOT NULL,
    can_see_doc             CHAR(1) NOT NULL,
    can_see_music           CHAR(1) NOT NULL,
    can_see_survey          CHAR(1) NOT NULL,
    can_see_post            CHAR(1) NOT NULL,
    can_see_post_comment    CHAR(1) NOT NULL,
    can_see_photo           CHAR(1) NOT NULL,
    can_see_photo_comment   CHAR(1) NOT NULL,
    can_see_good            CHAR(1) NOT NULL,
    can_see_good_comment    CHAR(1) NOT NULL,
    can_see_video           CHAR(1) NOT NULL,
    can_see_video_comment   CHAR(1) NOT NULL,
    can_see_planner         CHAR(1) NOT NULL,
    can_see_planner_comment CHAR(1) NOT NULL,

    can_add_post            CHAR(1) NOT NULL,
    can_add_photo           CHAR(1) NOT NULL,
    can_add_good            CHAR(1) NOT NULL,
    can_add_video           CHAR(1) NOT NULL,
    can_add_planner         CHAR(1) NOT NULL,
    can_add_doc             CHAR(1) NOT NULL,
    can_add_music           CHAR(1) NOT NULL,
    can_add_survey          CHAR(1) NOT NULL,

    can_create_post         CHAR(1) NOT NULL,
    can_create_photo        CHAR(1) NOT NULL,
    can_create_good         CHAR(1) NOT NULL,
    can_create_video        CHAR(1) NOT NULL,
    can_create_planner      CHAR(1) NOT NULL,
    can_create_doc          CHAR(1) NOT NULL,
    can_create_music        CHAR(1) NOT NULL,
    can_create_survey       CHAR(1) NOT NULL,

    CONSTRAINT fk_community_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES communities_memberships(id)
);

CREATE TABLE community_info (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,

    posts        INT DEFAULT 0,
    members      INT DEFAULT 0,
    photos       INT DEFAULT 0,
    goods        INT DEFAULT 0,
    tracks       INT DEFAULT 0,
    videos       INT DEFAULT 0,
    docs         INT DEFAULT 0,
    articles     INT DEFAULT 0,
    survey       INT DEFAULT 0,

    CONSTRAINT fk_community_info
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE community_private (
    id                SERIAL PRIMARY KEY,
    community_id      INT NOT NULL,
    can_see_member    CHAR(1) NOT NULL, -- Кто видит сообщества
    can_see_info      CHAR(1) NOT NULL, -- Кто видит информацию
    can_send_message  CHAR(1) NOT NULL, -- Кто пишет сообщения
    can_see_post      CHAR(1) NOT NULL, -- Кто видит записи
    can_see_photo     CHAR(1) NOT NULL, -- Кто видит фотографии
    can_see_good      CHAR(1) NOT NULL, -- Кто видит товары
    can_see_video     CHAR(1) NOT NULL, -- Кто видит видеозаписи
    can_see_music     CHAR(1) NOT NULL, -- Кто видит аудиозапис
    can_see_planner   CHAR(1) NOT NULL, -- Кто видит раздел планирования
    can_see_doc       CHAR(1) NOT NULL, -- Кто видит документы
    can_see_survey    CHAR(1) NOT NULL, -- Кто видит опросы

    can_see_settings  CHAR(1) NOT NULL, -- Кто видит настройки
    can_see_log       CHAR(1) NOT NULL, -- Кто видит логи
    can_see_stat      CHAR(1) NOT NULL, -- Кто видит статистику
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
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,       -- Сообщество
    list         INT DEFAULT 0,       -- Фотоальбом
    position     INT DEFAULT 0,       -- Порядок отображения
    types        CHAR(1) NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE community_post_list_position (
    id              SERIAL PRIMARY KEY,
    community_id    INT NOT NULL,      -- Сообщество
    list            INT DEFAULT 0,      -- Список записей
    position        INT DEFAULT 0,      -- Порядок отображения
    types           CHAR(1) NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка аудиозаписей -------
CREATE TABLE community_music_list_position (
    id             SERIAL PRIMARY KEY,  --
    community_id   INT NOT NULL,       -- Сообщество
    list           INT DEFAULT 0,       -- Список аудиозаписей
    position       INT DEFAULT 0,       -- Порядок отображения
    types          CHAR(1) NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка товаров -------
CREATE TABLE community_good_list_position (
    id           SERIAL PRIMARY KEY,  --
    community_id INT NOT NULL,        -- Сообщество
    list         INT DEFAULT 0,       -- Список товаров
    position     INT DEFAULT 0,       -- Порядок отображения
    types        CHAR(1) NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка видеозаписей -------
CREATE TABLE community_video_list_position (
    id           SERIAL PRIMARY KEY, --
    community_id INT NOT NULL,      -- Сообщество
    list         INT DEFAULT 0,      -- Список видеозаписей
    position     INT DEFAULT 0,      -- Порядок отображения
    types        CHAR(1) NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка опросов -------
CREATE TABLE community_survey_list_position (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,      -- Сообщество
    list          INT DEFAULT 0,      -- Список опросов
    position      INT DEFAULT 0,      -- Порядок отображения
    types         CHAR(1) NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка документов -------
CREATE TABLE community_doc_list_position (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,      -- Сообщество
    list          INT DEFAULT 0,      -- Список документов
    position      INT DEFAULT 0,      -- Порядок отображения
    types         CHAR(1) NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);
