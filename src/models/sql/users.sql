-- пользователи -------

CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    phone         VARCHAR(14) NOT NULL,
    types         SMALLINT NOT NULL,
    gender        "char" NOT NULL,
    device        "char" NOT NULL,
    language      "char" NOT NULL,
    perm          SMALLINT NOT NULL,
    level         SMALLINT NOT NULL DEFAULT 100,
    password      VARCHAR(500) NOT NULL,
    link          VARCHAR(100) NOT NULL,
    city          VARCHAR(100),
    status        VARCHAR(100),
    b_avatar      VARCHAR(500),
    s_avatar      VARCHAR(500),
    email         VARCHAR(100),
    birthday      DATE NOT NULL,
    last_activity TIMESTAMP NOT NULL,

    UNIQUE(phone),
    UNIQUE(email)
);

-- профили пользователей -------
CREATE TABLE user_profiles (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    posts          INT NOT NULL,
    views_post     INT NOT NULL,
    friends        INT NOT NULL,
    follows        INT NOT NULL,
    communities    INT NOT NULL,
    photos         INT NOT NULL,
    goods          INT NOT NULL,
    docs           INT NOT NULL,
    tracks         INT NOT NULL,
    videos         INT NOT NULL,
    articles       INT NOT NULL,
    --messages     INT NOT NULL,
    planners       INT NOT NULL,
    avatar_id      INT,
    activity       VARCHAR(500),
    interests      VARCHAR(500),
    favorite_music VARCHAR(500),
    favorite_films VARCHAR(500),
    favorite_books VARCHAR(500),
    favorite_game  VARCHAR(500),
    about          VARCHAR(500),
    survey         INT NOT NULL,
    playlist       INT NOT NULL, --id сохраненного плейлиста.

    CONSTRAINT fk_user_profile
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- местоположения пользователей -------
CREATE TABLE user_locations (
    id SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    city_ru     VARCHAR(100),
    city_en     VARCHAR(100),
    region_ru   VARCHAR(100),
    region_en   VARCHAR(100),
    country_ru  VARCHAR(100),
    country_en  VARCHAR(100),

    CONSTRAINT fk_user_location
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- айпи пользователей -------
CREATE TABLE ip_users (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    ip      VARCHAR NOT NULL,

    CONSTRAINT fk_ip_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Анкета аккаунта -------
CREATE TABLE user_anketas (
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
CREATE TABLE user_delete_anketas (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    answer  "char" NOT NULL,
    other   VARCHAR(200),
    created TIMESTAMP NOT NULL,

    CONSTRAINT fk_user_delete_anketa
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Статус отношений -------
CREATE TABLE user_love_statuss (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    male_status   VARCHAR(6),
    female_status VARCHAR(6),

    CONSTRAINT fk_user_love_status
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- Муж/Жена -------
CREATE TABLE user_partner_ones (
    id          SERIAL PRIMARY KEY,
    partner_user_i     INT NOT NULL,
    partner_id  INT NOT NULL,

    CONSTRAINT fk_user_partner_one_user
         FOREIGN KEY(partner_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_partner_one_partner
         FOREIGN KEY(partner_id)
             REFERENCES users(id)
);

-- Мама -------
CREATE TABLE user_mom_ones (
    id      SERIAL PRIMARY KEY,
    mom_user_i INT NOT NULL,
    mom_id  INT NOT NULL,

    CONSTRAINT fk_user_mom_one_user
         FOREIGN KEY(mom_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_mom_one_mom
         FOREIGN KEY(mom_id)
             REFERENCES users(id)
);

-- Папа -------
CREATE TABLE user_dad_ones (
    id      SERIAL PRIMARY KEY,
    dad_user_i INT NOT NULL,
    dad_id  INT NOT NULL,

    CONSTRAINT fk_user_dad_one_user
         FOREIGN KEY(dad_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_dad_one_dad
         FOREIGN KEY(dad_id)
             REFERENCES users(id)
);

-- Братья, сёстры -------
CREATE TABLE user_brother_sisters (
    id        SERIAL PRIMARY KEY,
    brother_user_i   INT NOT NULL,
    brother_target_id INT NOT NULL,

    CONSTRAINT fk_user_brother_sister_one_user
         FOREIGN KEY(brother_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_brother_sister_one_target
         FOREIGN KEY(brother_target_id)
             REFERENCES users(id)
);

-- Дети -------
CREATE TABLE user_children_ones (
    id        SERIAL PRIMARY KEY,
    child_user_i   INT NOT NULL,
    child_id  INT NOT NULL,

    CONSTRAINT fk_user_children_sister_one_user
         FOREIGN KEY(child_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_children_sister_one_child
         FOREIGN KEY(child_id)
             REFERENCES users(id)
);

-- Внуки -------
CREATE TABLE user_grandsons_ones (
    id          SERIAL PRIMARY KEY,
    grandson_user_i     INT NOT NULL,
    grandson_id INT NOT NULL,

    CONSTRAINT fk_user_grandsons_sister_one_user
         FOREIGN KEY(grandson_user_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_grandsons_sister_one_grandson
         FOREIGN KEY(grandson_id)
             REFERENCES users(id)
);

-- Коллеги -------
CREATE TABLE user_colleagues_ones (
    id            SERIAL PRIMARY KEY,
    user_colleague_i       INT NOT NULL,
    colleague_id  INT NOT NULL,

    CONSTRAINT fk_user_colleagues_sister_one_user
         FOREIGN KEY(user_colleague_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_colleagues_sister_one_colleague
         FOREIGN KEY(colleague_id)
             REFERENCES users(id)
);

-- Черный список -------
CREATE TABLE user_blocks (
    id               SERIAL PRIMARY KEY,
    user_block_i          INT NOT NULL,
    blocked_user_id  INT NOT NULL,

    CONSTRAINT fk_user_blocks_user
         FOREIGN KEY(user_block_i)
             REFERENCES users(id),

    CONSTRAINT fk_user_blocks_blocked_user
         FOREIGN KEY(blocked_user_id)
             REFERENCES users(id)
);

------------------
------------------
-- Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их группировать) -------
CREATE TABLE list_user_communities_keys (
    id    SERIAL PRIMARY KEY,
    types "char" NOT NULL,      -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL,    -- название
    owner INT NOT NULL        -- владелец
);

-- Ключи рекомендаций -------
CREATE TABLE featured_user_communities (
    id            SERIAL PRIMARY KEY,
    owner         INT NOT NULL,                  -- кто получает рекомендации
    list_id       INT,                             -- список, если есть
    user_id       INT,                  -- рекомендуемый друг
    community_id  INT,                  -- рекомендуемое сообщество
    mute          BOOLEAN NOT NULL DEFAULT false, -- не получать рекомендации источника
    sleep         TIMESTAMP                       -- не получать рекомендации источника до указанного времени

);
-- Ключи новостей -------
CREATE TABLE news_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                  -- кто получает новости
    list_id      INT,
    user_id      INT,                            -- новости друга
    community_id INT,                       -- новости сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать новости источника
    sleep        TIMESTAMP                     -- не получать новости источника до указанного времени
);
-- Ключи уыедомлений -------
CREATE TABLE notify_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                  -- кто получает уведомления
    list_id      INT,
    user_id      INT,                            -- уведомления друга
    community_id INT,                       -- уведомления сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать уведомления источника
    sleep        TIMESTAMP                      -- не получать уведомления источника до указанного времени
);

------------------
------------------
-- Изменение порядка следования списков пользователя

-- Порядок следования фотоальбома -------
CREATE TABLE user_photo_list_positions (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,     -- Пользователь
    list_id          INT NOT NULL,     -- Фотоальбом
    position      SMALLINT NOT NULL, -- Порядок отображения
    types         "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE user_post_list_positions (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list_id         INT NOT NULL,     -- Список записей
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка аудиозаписей -------
CREATE TABLE user_music_list_positions (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,     -- Пользователь
    list_id          INT NOT NULL,     -- Список аудиозаписей
    position      SMALLINT NOT NULL,     -- Порядок отображения
    types         "char" NOT NULL      -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка товаров -------
CREATE TABLE user_good_list_positions (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list_id         INT NOT NULL,     -- Список товаров
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка видеозаписей -------
CREATE TABLE user_video_list_positions (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list_id         INT NOT NULL,     -- Список видеозаписей
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка опросов -------
CREATE TABLE user_survey_list_positions (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,     -- Пользователь
    list_id         INT NOT NULL,     -- Список опросов
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка документов -------
CREATE TABLE user_doc_list_positions (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,     -- Пользователь
    list_id        INT NOT NULL,     -- Список документов
    position       SMALLINT NOT NULL, -- Порядок отображения
    types          "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

------------------
------------------
-- Приватность пользователя

-- Настройка дизайна -------
CREATE TABLE design_settings (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    background "char" NOT NULL,

    CONSTRAINT fk_design_settings
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Настройки приватности пользователя -------
-- 1:Все пользователи; 4:Друзья; 5:Друзья и друзья друзей;6:Только я
-- 17:Друзья, кроме; 18:Некоторые друзья
CREATE TABLE user_privates (
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,
    can_see_all       "char" NOT NULL, -- Для кого профиль открыт...
    can_see_community "char" NOT NULL, -- Кто видит сообщества
    can_see_info      "char" NOT NULL,      -- Кто видит информацию
    can_see_friend    "char" NOT NULL,    -- Кто видит друзей
    can_send_message  "char" NOT NULL,  -- Кто пишет сообщения
    can_add_in_chat   "char" NOT NULL,   -- Кто приглашает в беседы
    can_see_post      "char" NOT NULL,      -- Кто видит записи
    can_see_photo     "char" NOT NULL,     -- Кто видит фотографии
    can_see_good      "char" NOT NULL,      -- Кто видит товары
    can_see_video     "char" NOT NULL,     -- Кто видит видеозаписи
    can_see_music     "char" NOT NULL,     -- Кто видит аудиозапис
    can_see_planner   "char" NOT NULL,   -- Кто видит раздел планирования
    can_see_doc       "char" NOT NULL,       -- Кто видит документы
    can_see_survey    "char" NOT NULL,    -- Кто видит опросы

    CONSTRAINT fk_user_private
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);


-- Уведомления профиля -------
CREATE TABLE user_profile_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    community_invite     BOOLEAN NOT NULL DEFAULT true,

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
    reactions               BOOLEAN NOT NULL DEFAULT true,

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
    reactions               BOOLEAN NOT NULL DEFAULT true,

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
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_video_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления товаров -------
CREATE TABLE user_good_notifications (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,
    comment                 BOOLEAN NOT NULL DEFAULT true,
    comment_reply           BOOLEAN NOT NULL DEFAULT true,
    mention                 BOOLEAN NOT NULL DEFAULT true,
    comment_mention         BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,
    reactions               BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_good_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления опросов -------
CREATE TABLE user_survey_notifications (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,
    vote                    BOOLEAN NOT NULL DEFAULT true,
    repost                  BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_survey_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

-- Уведомления аудиозаписей -------
CREATE TABLE user_music_notifications (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    repost     BOOLEAN NOT NULL DEFAULT true,

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
    count     INT NOT NULL DEFAULT 0,

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
    count       INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_user_populate_stickers_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_user_populate_stickers_sticker
        FOREIGN KEY(sticker_id)
            REFERENCES stickers(id)
);


-- Уведомления сообщества -------
CREATE TABLE user_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    user_invite          BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
