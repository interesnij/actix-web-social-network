CREATE TABLE notifications (
    id                    SERIAL PRIMARY KEY,
    recipient_id          INT,
    creator_id            INT,
    created               TIMESTAMP NOT NULL,
    verb                  VARCHAR,
    status                VARCHAR,
    type                  VARCHAR,
    object_id             INT NOT NULL,
    community_id          INT,
    action_community_id   INT,
    user_set_id           INT,
    object_set_id         INT,
)

CREATE TABLE wall_objects (
    id                    SERIAL PRIMARY KEY,
    creator_id            INT,
    created               TIMESTAMP NOT NULL,
    verb                  VARCHAR,
    status                VARCHAR,
    type                  VARCHAR,
    object_id             INT NOT NULL,
    community_id          INT,
    action_community_id   INT,
    user_set_id           INT,
    object_set_id         INT,
)
