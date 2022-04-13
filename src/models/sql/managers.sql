CREATE TABLE moderateds (
    id            SERIAL PRIMARY KEY,
    description   VARCHAR(500),
    verified      BOOLEAN NOT NULL DEFAULT false,
    status        "char" NOT NULL,
    types         SMALLINT NOT NULL,
    object_id     INT NOT NULL,
    created       TIMESTAMP NOT NULL,
    count         INT DEFAULT 0
);

CREATE TABLE moderated_reports (
    id                  SERIAL PRIMARY KEY,
    reporter_id         INT NOT NULL,
    moderated_object_id INT NOT NULL,
    description         VARCHAR(500),
    types               "char" NOT NULL,
    created             TIMESTAMP NOT NULL,

    CONSTRAINT fk_moderated_reports_reporter
        FOREIGN KEY(reporter_id)
            REFERENCES users(id),

    CONSTRAINT fk_moderated_reports_moderated_object
        FOREIGN KEY(moderated_object_id)
            REFERENCES moderateds(id)
);

CREATE TABLE moderated_penalties (
    id                  SERIAL PRIMARY KEY,
    manager_id          INT NOT NULL,
    moderated_object_id INT NOT NULL,
    expiration          TIMESTAMP,
    types               SMALLINT NOT NULL,
    object_id           INT NOT NULL,
    status              "char" NOT NULL,
    created             TIMESTAMP NOT NULL,

    CONSTRAINT fk_moderated_penalties_reporter
        FOREIGN KEY(manager_id)
            REFERENCES users(id),

    CONSTRAINT fk_moderated_penalties_moderated_object
        FOREIGN KEY(moderated_object_id)
            REFERENCES moderateds(id)
);


CREATE TABLE moderated_logs (
    id              SERIAL PRIMARY KEY,
    manager_id      INT NOT NULL,
    object_id       INT NOT NULL,
    action          "char" NOT NULL,
    description     VARCHAR(500),
    types           SMALLINT NOT NULL,
    created         TIMESTAMP NOT NULL,
    time_to_suspend TIMESTAMP,

    CONSTRAINT fk_moderated_logs_manager
        FOREIGN KEY(manager_id)
            REFERENCES users(id)
);

CREATE TABLE staff_logs (
    id          SERIAL PRIMARY KEY,
    types       SMALLINT NOT NULL,
    action      "char" NOT NULL,
    manager_id  INT NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,

    CONSTRAINT fk_staff_logs_manager
        FOREIGN KEY(manager_id)
            REFERENCES users(id)
);

CREATE TABLE support_users (
    id          SERIAL PRIMARY KEY,
    manager_id  INT NOT NULL,
    level       SMALLINT DEFAULT 0,
    points      INT DEFAULT 0,
    chats       INT DEFAULT 0,
    created     TIMESTAMP NOT NULL
);
