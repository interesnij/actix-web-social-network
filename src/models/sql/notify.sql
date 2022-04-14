CREATE TABLE notifications (
    id                    SERIAL PRIMARY KEY,
    recipient_id          INT,
    user_id               INT NOT NULL,
    created               TIMESTAMP NOT NULL,
    verb                  VARCHAR(150) NOT NULL,
    status                "char" NOT NULL,
    types                 SMALLINT NOT NULL,
    object_id             INT NOT NULL,
    community_id          INT,
    action_community_id   INT,
    user_set_id           INT,
    object_set_id         INT,

    CONSTRAINT fk_notifications_recipient
        FOREIGN KEY(recipient_id)
            REFERENCES users(id),
    CONSTRAINT fk_notifications_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_notifications_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),
    CONSTRAINT fk_notifications_action_community
        FOREIGN KEY(action_community_id)
            REFERENCES communitys(id)
);

CREATE TABLE wall_objects (
    id                    SERIAL PRIMARY KEY,
    user_id               INT NOT NULL,
    created               TIMESTAMP NOT NULL,
    verb                  VARCHAR(150) NOT NULL,
    status                "char" NOT NULL,
    types                 SMALLINT NOT NULL,
    object_id             INT NOT NULL,
    community_id          INT,
    action_community_id   INT,
    user_set_id           INT,
    object_set_id         INT,

    CONSTRAINT fk_wall_objects_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_wall_objects_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),
    CONSTRAINT fk_wall_objects_action_community
        FOREIGN KEY(action_community_id)
            REFERENCES communitys(id)
)
