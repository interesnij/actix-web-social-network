
CREATE TABLE survey_lists (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(100) NOT NULL,
    community_id  INT,
    user_id    INT NOT NULL,
    types         "char" NOT NULL,
    description   VARCHAR(500),
    created       TIMESTAMP NOT NULL,
    count         INT DEFAULT 0,
    repost        INT DEFAULT 0,
    copy          INT DEFAULT 0,
    position      SMALLINT DEFAULT 0,

    can_see_el    "char" NOT NULL,
    create_el     "char" NOT NULL,
    copy_el       "char" NOT NULL,

    CONSTRAINT fk_survey_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_survey_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE surveys (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    community_id  INT,
    user_id    INT NOT NULL,
    survey_list_id       INT NOT NULL,
    types         "char" NOT NULL,
    image         VARCHAR(500),
    is_anonymous  BOOLEAN NOT NULL DEFAULT false,
    is_multiple   BOOLEAN NOT NULL DEFAULT false,
    is_no_edited  BOOLEAN NOT NULL DEFAULT false,
    time_end      TIMESTAMP,
    created       TIMESTAMP NOT NULL,

    view        INT DEFAULT 0,
    repost        INT DEFAULT 0,
    copy          INT DEFAULT 0,
    position      SMALLINT DEFAULT 0,
    vote          INT DEFAULT 0,

    CONSTRAINT fk_surveys_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_surveys_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_surveys_list
        FOREIGN KEY(survey_list_id)
            REFERENCES survey_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_survey_list_collections (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    survey_list_id   INT NOT NULL,

   CONSTRAINT fk_user_survey_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_survey_list_collections_list
        FOREIGN KEY(survey_list_id)
            REFERENCES survey_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_survey_list_collections (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    survey_list_id       INT NOT NULL,

   CONSTRAINT fk_community_survey_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_survey_list_collections_list
        FOREIGN KEY(survey_list_id)
            REFERENCES survey_lists(id)
);

CREATE TABLE survey_list_perms (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    survey_list_id       INT NOT NULL,
    can_see_item  "char",
    create_item   "char",
    can_copy      "char",

   CONSTRAINT fk_survey_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_survey_list_perm_list
        FOREIGN KEY(survey_list_id)
            REFERENCES survey_lists(id)
);
