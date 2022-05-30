-- Категории сообществ -------
CREATE TABLE community_categorys (
    id SERIAL PRIMARY KEY,  -- id объекта
    name VARCHAR(100) NOT NULL,           -- название
    avatar VARCHAR(500),            -- аватар
    position SMALLINT NOT NULL      -- порядковый номер
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategorys (
    id          SERIAL PRIMARY KEY,       -- id объекта
    name        VARCHAR(100) NOT NULL,                  -- название
    category_id INT NOT NULL,             -- id категории
    avatar      VARCHAR(500),                     -- аватар
    position    SMALLINT NOT NULL,            -- порядковый номер

    CONSTRAINT fk_community_subcategories -- связь с категорией
        FOREIGN KEY(category_id)
            REFERENCES community_categorys(id)
);

CREATE TABLE communitys (
    id            SERIAL PRIMARY KEY,     -- id объекта
    name          VARCHAR(100) NOT NULL,  -- название
    description   VARCHAR(500),           -- описание
    status        VARCHAR(100),           -- статус
    types         SMALLINT NOT NULL,      -- тип
    perm          "char" NOT NULL,          -- приватность
    level         SMALLINT NOT NULL DEFAULT 100,   -- уровень доверия
    link          VARCHAR(100) NOT NULL,           -- красивая ссылка
    b_avatar      VARCHAR(500),           -- большой аватар
    s_avatar      VARCHAR(500),           -- маленький аватар
    cover         VARCHAR(500),           -- баннер
    community_subcategory_id   INT NOT NULL,           -- id категории
    user_id    INT NOT NULL,           -- id создателя
    created       TIMESTAMP NOT NULL,     -- когда создано

    CONSTRAINT fk_community_creator   -- связь с пользователем
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_community_category  -- связь с категорией
        FOREIGN KEY(community_subcategory_id)
            REFERENCES community_subcategorys(id)
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
            REFERENCES communitys(id)
);

CREATE TABLE community_infos (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,

    posts        INT NOT NULL,
    members      INT NOT NULL,
    photos       INT NOT NULL,
    goods        INT NOT NULL,
    tracks       INT NOT NULL,
    videos       INT NOT NULL,
    docs         INT NOT NULL,
    articles     INT NOT NULL,
    survey       INT NOT NULL,
    planners     INT NOT NULL,
    avatar_id    INT,

    CONSTRAINT fk_community_info
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE community_privates (
    id                SERIAL PRIMARY KEY,
    community_id      INT NOT NULL,
    can_see_member    "char" NOT NULL, -- Кто видит сообщества
    can_see_info      "char" NOT NULL, -- Кто видит информацию
    can_send_message  "char" NOT NULL, -- Кто пишет сообщения
    can_see_post      "char" NOT NULL, -- Кто видит записи
    can_see_photo     "char" NOT NULL, -- Кто видит фотографии
    can_see_good      "char" NOT NULL, -- Кто видит товары
    can_see_video     "char" NOT NULL, -- Кто видит видеозаписи
    can_see_music     "char" NOT NULL, -- Кто видит аудиозапис
    can_see_planner   "char" NOT NULL, -- Кто видит раздел планирования
    can_see_doc       "char" NOT NULL, -- Кто видит документы
    can_see_survey    "char" NOT NULL, -- Кто видит опросы

    can_see_settings  "char" NOT NULL, -- Кто видит настройки
    can_see_log       "char" NOT NULL, -- Кто видит логи
    can_see_stat      "char" NOT NULL, -- Кто видит статистику
    can_see_forum     "char" NOT NULL, -- Кто видит опросы
    CONSTRAINT fk_community_private
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
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
             REFERENCES communitys(id)
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
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_post_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
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
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_photo_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
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
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_video_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
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
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_good_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
);

-- Уведомления опросов -------
CREATE TABLE community_survey_notifications (
    id                      SERIAL PRIMARY KEY,
    community_id            INT NOT NULL,
    vote                    BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_survey_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
);

-- Уведомления аудиозаписей -------
CREATE TABLE community_music_notifications (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    repost        BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_community_music_notifications
         FOREIGN KEY(community_id)
             REFERENCES communitys(id)
);

------------------
------------------
-- Изменение порядка следования списков сообщества

-- Порядок следования фотоальбома -------
CREATE TABLE community_photo_list_positions (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,       -- Сообщество
    list_id      INT NOT NULL,       -- Фотоальбом
    position     SMALLINT NOT NULL,       -- Порядок отображения
    types        "char" NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE community_post_list_positions (
    id              SERIAL PRIMARY KEY,
    community_id    INT NOT NULL,      -- Сообщество
    list_id         INT NOT NULL,      -- Список записей
    position        SMALLINT NOT NULL,      -- Порядок отображения
    types           "char" NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка аудиозаписей -------
CREATE TABLE community_music_list_positions (
    id             SERIAL PRIMARY KEY,  --
    community_id   INT NOT NULL,       -- Сообщество
    list_id        INT NOT NULL,       -- Список аудиозаписей
    position       SMALLINT NOT NULL,       -- Порядок отображения
    types          "char" NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка товаров -------
CREATE TABLE community_good_list_positions (
    id           SERIAL PRIMARY KEY,  --
    community_id INT NOT NULL,        -- Сообщество
    list_id      INT NOT NULL,       -- Список товаров
    position     SMALLINT NOT NULL,       -- Порядок отображения
    types        "char" NOT NULL        -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка видеозаписей -------
CREATE TABLE community_video_list_positions (
    id           SERIAL PRIMARY KEY, --
    community_id INT NOT NULL,      -- Сообщество
    list_id      INT NOT NULL,      -- Список видеозаписей
    position     SMALLINT NOT NULL,      -- Порядок отображения
    types        "char" NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка опросов -------
CREATE TABLE community_survey_list_positions (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,      -- Сообщество
    list_id       INT NOT NULL,      -- Список опросов
    position      SMALLINT NOT NULL,      -- Порядок отображения
    types         "char" NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка документов -------
CREATE TABLE community_doc_list_positions (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,      -- Сообщество
    list_id       INT NOT NULL,      -- Список документов
    position      SMALLINT NOT NULL,      -- Порядок отображения
    types         "char" NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Черный список -------
CREATE TABLE community_banner_users (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    user_id       INT NOT NULL,

    CONSTRAINT fk_community_banner_users_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_community_banner_users_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
