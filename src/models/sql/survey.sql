
CREATE TABLE survey_lists (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(100) NOT NULL,
    community_id  INT,
    creator_id    INT NOT NULL,
    types         CHAR NOT NULL,
    description   VARCHAR(500),
    created       TIMESTAMP NOT NULL,
    count         INT DEFAULT 0,
    repost        INT DEFAULT 0,
    copy          INT DEFAULT 0,

    can_see_el    CHAR NOT NULL,
    create_el     CHAR NOT NULL,
    copy_el       CHAR NOT NULL,
    position      CHAR NOT NULL,

    CONSTRAINT fk_survey_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_survey_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE surveys (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    community_id  INT,
    creator_id    INT NOT NULL,
    list_id       INT NOT NULL,
    types         CHAR NOT NULL,
    image         VARCHAR(500),
    is_anonymous  BOOLEAN NOT NULL DEFAULT false,
    is_multiple   BOOLEAN NOT NULL DEFAULT false,
    is_no_edited  BOOLEAN NOT NULL DEFAULT false,
    time_end      TIMESTAMP,
    created       TIMESTAMP NOT NULL,

    repost        INT DEFAULT 0,
    copy          INT DEFAULT 0,
    position      INT DEFAULT 0,
    view          INT DEFAULT 0,
    vote          INT DEFAULT 0,

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
    user_id   INT NOT NULL,
    list_id   INT NOT NULL,

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
    community_id  INT NOT NULL,
    list_id       INT NOT NULL,

   CONSTRAINT fk_community_survey_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_survey_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);

CREATE TABLE survey_list_perm (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    list_id       INT NOT NULL,
    can_see_item  CHAR NOT NULL,
    create_item   CHAR NOT NULL,
    can_copy      CHAR NOT NULL,

   CONSTRAINT fk_survey_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_survey_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES survey_lists(id)
);
