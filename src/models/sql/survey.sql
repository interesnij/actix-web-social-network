
CREATE TABLE survey_lists (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR,
    community_id  INT,
    creator_id    INT NOT NULL,
    _type         VARCHAR NOT NULL,
    description   TEXT,
    created       TIMESTAMP NOT NULL,
    count         INT NOT NULL DEFAULT 0,
    repost        INT NOT NULL DEFAULT 0,
    copy          INT NOT NULL DEFAULT 0,

    can_see_el    INT NOT NULL DEFAULT 1,
    create_el     INT NOT NULL DEFAULT 7,
    copy_el       INT NOT NULL DEFAULT 1,
    order         INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_survey_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_survey_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE surveys (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR,
    community_id  INT,
    creator_id    INT NOT NULL,
    list_id       INT NOT NULL,
    _type         VARCHAR NOT NULL,
    image         TEXT,
    is_anonymous  BOOLEAN NOT NULL DEFAULT false,
    is_multiple   BOOLEAN NOT NULL DEFAULT false,
    is_no_edited  BOOLEAN NOT NULL DEFAULT false,
    time_end      TIMESTAMP,
    created       TIMESTAMP NOT NULL,

    repost        INT NOT NULL DEFAULT 0,
    copy          INT NOT NULL DEFAULT 0,
    order         INT NOT NULL DEFAULT 0,
    view          INT NOT NULL DEFAULT 0,
    vote          INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_surveys_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_surveys_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_surveys_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_survey_list_collections (
    id        SERIAL PRIMARY KEY,
    user_id   INT,
    list_id   INT,

   CONSTRAINT fk_user_survey_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_survey_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_survey_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT,
    list_id       INT,

   CONSTRAINT fk_community_survey_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_survey_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);

CREATE TABLE survey_list_perm (
    id            SERIAL PRIMARY KEY,
    user_id       INT,
    list_id       INT,
    can_see_item  INT DEFAULT 0,
    create_item   INT DEFAULT 0,
    can_copy      INT DEFAULT 0,

   CONSTRAINT fk_survey_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_survey_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);
