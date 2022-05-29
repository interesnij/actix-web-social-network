
CREATE TABLE survey_lists (
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(100) NOT NULL,
    community_id  INT,
    user_id       INT NOT NULL,
    types         SMALLINT NOT NULL,
    description   VARCHAR(500),
    image         VARCHAR(500),
    created       TIMESTAMP NOT NULL,
    count         INT NOT NULL,
    repost        INT NOT NULL,
    copy          INT NOT NULL,
    position      SMALLINT NOT NULL,

    can_see_el    "char" NOT NULL,
    create_el     "char" NOT NULL,
    copy_el       "char" NOT NULL,

    CONSTRAINT fk_survey_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_survey_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE surveys (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    community_id    INT,
    user_id         INT NOT NULL,
    survey_list_id  INT NOT NULL,
    types         "char" NOT NULL,
    image         VARCHAR(500),
    is_anonymous  BOOLEAN NOT NULL DEFAULT false,
    is_multiple   BOOLEAN NOT NULL DEFAULT false,
    is_no_edited  BOOLEAN NOT NULL DEFAULT false,
    time_end      TIMESTAMP,
    created       TIMESTAMP NOT NULL,

    view          INT NOT NULL,
    repost        INT NOT NULL,
    copy          INT NOT NULL,
    position      SMALLINT NOT NULL,
    vote          INT NOT NULL,

    CONSTRAINT fk_surveys_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_surveys_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

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
            REFERENCES communitys(id),

   CONSTRAINT fk_community_survey_list_collections_list
        FOREIGN KEY(survey_list_id)
            REFERENCES survey_lists(id)
);

CREATE TABLE survey_list_perms (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    survey_list_id  INT NOT NULL,
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

CREATE TABLE survey_answers (
    id           SERIAL PRIMARY KEY,
    content      VARCHAR (100) NOT NULL,
    survey_id    INT NOT NULL,
    vote         INT NOT NULL,
    position     INT NOT NULL,

   CONSTRAINT fk_survey_answers_survey
        FOREIGN KEY(survey_id)
            REFERENCES surveys(id)
);

CREATE TABLE survey_votes (
    id               SERIAL PRIMARY KEY,
    user_id          INT NOT NULL,
    survey_answer_id INT NOT NULL,
    survey_id        INT NOT NULL,

   CONSTRAINT fk_survey_votes_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_survey_votes_answer
        FOREIGN KEY(survey_answer_id)
            REFERENCES survey_answers(id)
);
