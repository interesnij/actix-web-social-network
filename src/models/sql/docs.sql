
CREATE TABLE doc_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    community_id    INT,
    creator_id      INT NOT NULL,
    types           CHAR NOT NULL,
    description     VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    count           INT DEFAULT 0,
    repost          INT DEFAULT 0,
    copy            INT DEFAULT 0,

    can_see_el      CHAR NOT NULL,
    can_see_comment CHAR NOT NULL,
    create_el       CHAR NOT NULL,
    create_comment  CHAR NOT NULL,
    copy_el         CHAR NOT NULL,
    position        CHAR NOT NULL,

    CONSTRAINT fk_doc_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_doc_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE docs (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(200) NOT NULL,
    community_id INT,
    creator_id   INT NOT NULL,
    list_id      INT NOT NULL,
    types        CHAR NOT NULL,
    types_2      CHAR NOT NULL,
    file         VARCHAR(500) NOT NULL,
    created      TIMESTAMP NOT NULL,

    repost       INT DEFAULT 0,
    copy         INT DEFAULT 0,
    position     INT DEFAULT 0,
    view         INT DEFAULT 0,

    CONSTRAINT fk_docs_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_docs_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_docs_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_doc_list_collections (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    list_id INT NOT NULL,

   CONSTRAINT fk_user_doc_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_doc_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_doc_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    list_id      INT NOT NULL,

   CONSTRAINT fk_community_doc_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_doc_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);

CREATE TABLE doc_list_perm (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    list_id       INT NOT NULL,
    can_see_item  CHAR NOT NULL,
    create_item   CHAR NOT NULL,
    can_copy      CHAR NOT NULL,

   CONSTRAINT fk_doc_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_doc_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES doc_lists(id)
);
