-- пользователи -------
CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    phone         VARCHAR(14) NOT NULL,
    types         VARCHAR(6) NOT NULL,
    gender        VARCHAR(3) NOT NULL,
    device        VARCHAR(2) NOT NULL,
    language      VARCHAR(2) NOT NULL,
    perm          INT DEFAULT 1,
    level         INT DEFAULT 100,
    password      VARCHAR(100) NOT NULL,
    have_link     VARCHAR(100),
    city          VARCHAR(100),
    status        VARCHAR(100),
    b_avatar      VARCHAR(500),
    s_avatar      VARCHAR(500),
    email         VARCHAR(100),
    birthday      TIMESTAMP NOT NULL,
    last_activity TIMESTAMP NOT NULL,

    UNIQUE(phone),
    UNIQUE(email)
);

-- профили пользователей -------
CREATE TABLE user_profile (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    posts          INT DEFAULT 0,
    views_post     INT DEFAULT 0,
    friends        INT DEFAULT 0,
    follows        INT DEFAULT 0,
    communities    INT DEFAULT 0,
    photos         INT DEFAULT 0,
    goods          INT DEFAULT 0,
    docs           INT DEFAULT 0,
    tracks         INT DEFAULT 0,
    videos         INT DEFAULT 0,
    articles       INT DEFAULT 0,
    _time          TIMESTAMP,
    height         FLOAT,
    activity       VARCHAR(500),
    interests      VARCHAR(500),
    favorite_music VARCHAR(500),
    favorite_films VARCHAR(500),
    favorite_books VARCHAR(500),
    favorite_game  VARCHAR(500),
    about          VARCHAR(500),

    CONSTRAINT fk_user_profile
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- местоположения пользователей -------
CREATE TABLE user_location (
    id SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    city_ru     VARCHAR(100),
    city_en     VARCHAR(100),
    city_lat    FLOAT,
    city_lon    FLOAT,

    region_ru   VARCHAR(100),
    region_en   VARCHAR(100),
    country_ru  VARCHAR(100),
    country_en  VARCHAR(100),

    CONSTRAINT fk_user_location
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- айпи пользователей -------
CREATE TABLE ip_user (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    ip      VARCHAR,

    CONSTRAINT fk_ip_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Анкета аккаунта -------
CREATE TABLE user_anketa (
    id                    SERIAL PRIMARY KEY,
    user_id               INT NOT NULL,
    political_preferences VARCHAR(500),
    worldview             VARCHAR(500),
    mainthing_in_life     VARCHAR(500),
    mainthing_in_people   VARCHAR(500),
    attitude_to_smoking   VARCHAR(500),
    attitude_to_alcohol   VARCHAR(500),
    inspiration           VARCHAR(500),

    CONSTRAINT fk_user_anketa
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Причина удаления аккаунта -------
CREATE TABLE user_delete_anketa (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    answer  VARCHAR(6),
    other   VARCHAR(200),

    CONSTRAINT fk_user_delete_anketa
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Статус отношений -------
CREATE TABLE user_love_status (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    male_status   VARCHAR(6),
    female_status VARCHAR(6),

    CONSTRAINT fk_user_love_status
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Муж/Жена -------
CREATE TABLE user_partner_one (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    partner_id  INT NOT NULL,

    CONSTRAINT fk_user_partner_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_partner_one_partner
         FOREIGN KEY(partner_id)
             REFERENCES users(id)
);

-- Мама -------
CREATE TABLE user_mom_one (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    mom_id  INT NOT NULL,

    CONSTRAINT fk_user_mom_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_mom_one_mom
         FOREIGN KEY(mom_id)
             REFERENCES users(id)
);

-- Папа -------
CREATE TABLE user_dad_one (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    dad_id  INT NOT NULL,

    CONSTRAINT fk_user_dad_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_dad_one_dad
         FOREIGN KEY(dad_id)
             REFERENCES users(id)
);

-- Братья, сёстры -------
CREATE TABLE user_brother_sister (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,

    CONSTRAINT fk_user_brother_sister_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_brother_sister_one_target
         FOREIGN KEY(target_id)
             REFERENCES users(id)
);

-- Дети -------
CREATE TABLE user_children_one (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    child_id  INT NOT NULL,

    CONSTRAINT fk_user_children_sister_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_children_sister_one_child
         FOREIGN KEY(child_id)
             REFERENCES users(id)
);

-- Внуки -------
CREATE TABLE user_grandsons_one (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    grandson_id INT NOT NULL,

    CONSTRAINT fk_user_grandsons_sister_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_grandsons_sister_one_grandson
         FOREIGN KEY(grandson_id)
             REFERENCES users(id)
);

-- Коллеги -------
CREATE TABLE user_colleagues_one (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    colleague_id  INT NOT NULL,

    CONSTRAINT fk_user_colleagues_sister_one_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_colleagues_sister_one_colleague
         FOREIGN KEY(colleague_id)
             REFERENCES users(id)
);

-- Черный список -------
CREATE TABLE user_blocks (
    id               SERIAL PRIMARY KEY,
    user_id          INT NOT NULL,
    blocked_user_id  INT NOT NULL,

    CONSTRAINT fk_user_blocks_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_blocks_blocked_user
         FOREIGN KEY(blocked_user_id)
             REFERENCES users(id)
);

------------------
------------------
-- Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их группировать) -------
CREATE TABLE list_uc (
    id    SERIAL PRIMARY KEY,
    types INT DEFAULT 0,      -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL,    -- название
    owner INT NOT NULL       -- владелец
);

-- Ключи рекомендаций -------
CREATE TABLE featured_uc (
    id            SERIAL PRIMARY KEY,
    owner         INT NOT NULL,                  -- кто получает рекомендации
    list_id       INT,                             -- список, если есть
    user_id       INT,                  -- рекомендуемый друг
    community_id  INT,                  -- рекомендуемое сообщество
    mute          BOOLEAN NOT NULL DEFAULT false, -- не получать рекомендации источника
    sleep         TIMESTAMP,                       -- не получать рекомендации источника до указанного времени

    CONSTRAINT fk_featured_uc_list
         FOREIGN KEY(list_id)
             REFERENCES list_uc(id)

);
-- Ключи новостей -------
CREATE TABLE news_uc (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                  -- кто получает новости
    list_id      INT,
    user_id      INT,                            -- новости друга
    community    INT,                       -- новости сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать новости источника
    sleep        TIMESTAMP,                      -- не получать новости источника до указанного времени

    CONSTRAINT fk_news_uc_list
         FOREIGN KEY(list_id)
             REFERENCES list_uc(id)
);
-- Ключи уыедомлений -------
CREATE TABLE notify_uc (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                  -- кто получает уведомления
    list_id      INT,
    user_id      INT,                            -- уведомления друга
    community    INT,                       -- уведомления сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать уведомления источника
    sleep        TIMESTAMP,                      -- не получать уведомления источника до указанного времени

    CONSTRAINT fk_notify_uc_list
         FOREIGN KEY(list_id)
             REFERENCES list_uc(id)
);

------------------
------------------
-- Изменение порядка следования списков пользователя

-- Порядок следования фотоальбома -------
CREATE TABLE user_photo_list_position (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,     -- Пользователь
    list          INT NOT NULL,     -- Фотоальбом
    position      INT DEFAULT 0, -- Порядок отображения
    types         INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE user_post_list_position (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list         INT NOT NULL,     -- Список записей
    position     INT DEFAULT 0, -- Порядок отображения
    types        INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка аудиозаписей -------
CREATE TABLE user_music_list_position (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,     -- Пользователь
    list          INT NOT NULL,     -- Список аудиозаписей
    position      INT DEFAULT 0,     -- Порядок отображения
    types         INT DEFAULT 1      -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка товаров -------
CREATE TABLE user_good_list_position (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list         INT NOT NULL,     -- Список товаров
    position     INT DEFAULT 0, -- Порядок отображения
    types        INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка видеозаписей -------
CREATE TABLE user_video_list_position (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list         INT NOT NULL,     -- Список видеозаписей
    position     INT DEFAULT 0, -- Порядок отображения
    types        INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка опросов -------
CREATE TABLE user_survey_list_position (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list         INT NOT NULL,     -- Список опросов
    position     INT DEFAULT 0, -- Порядок отображения
    types        INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка документов -------
CREATE TABLE user_doc_list_position (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,     -- Пользователь
    list           INT NOT NULL,     -- Список документов
    position       INT DEFAULT 0, -- Порядок отображения
    types          INT DEFAULT 1     -- 1 - открыт, 0 - недоступен (например, удален)
);

------------------
------------------
-- Приватность пользователя

-- Настройка дизайна -------
CREATE TABLE color_settings (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    color     VARCHAR(50) NOT NULL,

    CONSTRAINT fk_color_settings
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Настройки приватности пользователя -------
-- 1:Все пользователи; 4:Друзья; 5:Друзья и друзья друзей;6:Только я
-- 17:Друзья, кроме; 18:Некоторые друзья
CREATE TABLE user_private (
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,
    can_see_community INT DEFAULT 0, -- Кто видит сообщества
    can_see_info      INT DEFAULT 0,      -- Кто видит информацию
    can_see_friend    INT DEFAULT 0,    -- Кто видит друзей
    can_send_message  INT DEFAULT 0,  -- Кто пишет сообщения
    can_add_in_chat   INT DEFAULT 0,   -- Кто приглашает в беседы
    can_see_post      INT DEFAULT 0,      -- Кто видит записи
    can_see_photo     INT DEFAULT 0,     -- Кто видит фотографии
    can_see_good      INT DEFAULT 0,      -- Кто видит товары
    can_see_video     INT DEFAULT 0,     -- Кто видит видеозаписи
    can_see_music     INT DEFAULT 0,     -- Кто видит аудиозапис
    can_see_planner   INT DEFAULT 0,   -- Кто видит раздел планирования
    can_see_doc       INT DEFAULT 0,       -- Кто видит документы
    can_see_survey    INT DEFAULT 0,    -- Кто видит опросы

    CONSTRAINT fk_user_private
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);


-- Уведомления профиля -------
CREATE TABLE user_profile_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL,
    connection_request   BOOLEAN DEFAULT true,
    connection_confirmed BOOLEAN DEFAULT true,
    community_invite     BOOLEAN DEFAULT true,

    CONSTRAINT fk_user_profile_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления записей -------
CREATE TABLE user_post_notifications (
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

    CONSTRAINT fk_user_post_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления фотографий -------
CREATE TABLE user_photo_notifications (
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

    CONSTRAINT fk_user_photo_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления видеозаписей -------
CREATE TABLE user_video_notifications (
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

    CONSTRAINT fk_user_video_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления товаров -------
CREATE TABLE user_good_notifications (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,
    comment                 BOOLEAN DEFAULT true,
    comment_reply           BOOLEAN DEFAULT true,
    mention                 BOOLEAN DEFAULT true,
    comment_mention         BOOLEAN DEFAULT true,
    repost                  BOOLEAN DEFAULT true,
    liked                   BOOLEAN DEFAULT true,
    disliked                BOOLEAN DEFAULT true,
    comment_liked           BOOLEAN DEFAULT true,
    comment_disliked        BOOLEAN DEFAULT true,
    comment_reply_liked     BOOLEAN DEFAULT true,
    comment_reply_disliked  BOOLEAN DEFAULT true,

    CONSTRAINT fk_user_good_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления аудиозаписей -------
CREATE TABLE user_music_notifications (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    repost     BOOLEAN DEFAULT true,

    CONSTRAINT fk_user_music_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);


------------------
------------------
-- Смайлы и стикеры

-- Популярные смайлы -------
CREATE TABLE user_populate_smiles (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    smile_id  INT NOT NULL,
    count     INT DEFAULT 0,

    CONSTRAINT fk_user_populate_smiles_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_populate_smiles_smile
        FOREIGN KEY(smile_id)
            REFERENCES smiles(id)
);

-- Популярные стикеры -------
CREATE TABLE user_populate_stickers (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    sticker_id  INT NOT NULL,
    count       INT DEFAULT 0,

    CONSTRAINT fk_user_populate_stickers_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_populate_stickers_sticker
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id)
);
