-- Your SQL goes here

CREATE TABLE community_banner_users (
    id            SERIAL PRIMARY KEY,
    community_id  INT NOT NULL,
    user_id       INT NOT NULL,

    CONSTRAINT fk_community_banner_users_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_community_banner_users_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
