-- Your SQL goes here

CREATE TABLE community_invites (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    community_id   INT NOT NULL,
    invite_creator INT NOT NULL,

    CONSTRAINT fk_community_invites_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_community_invites_community
         FOREIGN KEY(community_id)
             REFERENCES communitys(id),

    CONSTRAINT fk_community_invites_creator
        FOREIGN KEY(invite_creator)
            REFERENCES users(id)
);
