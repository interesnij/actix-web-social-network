
CREATE TABLE doc_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    user_id         INT NOT NULL,
    types           SMALLINT NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    count           INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,

    can_see_el      "char" NOT NULL,
    create_el       "char" NOT NULL,
    copy_el         "char" NOT NULL,

    CONSTRAINT fk_doc_lists_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_doc_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);

CREATE TABLE docs (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(200) NOT NULL,
    community_id INT,
    user_id      INT NOT NULL,
    doc_list_id  INT NOT NULL,
    types        "char" NOT NULL,
    types_2      "char" NOT NULL,
    file         VARCHAR(500) NOT NULL,
    created      TIMESTAMP NOT NULL,

    view         INT NOT NULL,
    repost       INT NOT NULL,
    copy         INT NOT NULL,
    position     SMALLINT NOT NULL,

    CONSTRAINT fk_docs_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_docs_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_docs_list
        FOREIGN KEY(doc_list_id)
            REFERENCES doc_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_doc_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    doc_list_id INT NOT NULL,

   CONSTRAINT fk_user_doc_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_doc_list_collections_list
        FOREIGN KEY(doc_list_id)
            REFERENCES doc_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_doc_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    doc_list_id  INT NOT NULL,

   CONSTRAINT fk_community_doc_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

   CONSTRAINT fk_community_doc_list_collections_list
        FOREIGN KEY(doc_list_id)
            REFERENCES doc_lists(id)
);

CREATE TABLE doc_list_perms (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    doc_list_id   INT NOT NULL,
    can_see_item  "char",
    create_item   "char",
    can_copy      "char",

   CONSTRAINT fk_doc_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_doc_list_perm_list
        FOREIGN KEY(doc_list_id)
            REFERENCES doc_lists(id)
);
