
-- подписчики -------
CREATE TABLE follows (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    followed_user INT NOT NULL,
    view          BOOLEAN NOT NULL DEFAULT false,
    visited       INT DEFAULT 0,

    CONSTRAINT fk_follows_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_follows_followed_user
         FOREIGN KEY(followed_user)
             REFERENCES users(id)
);

-- заявки на вступление в закрытое сообщество -------
CREATE TABLE community_follows (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    community_id INT NOT NULL,
    view         BOOLEAN NOT NULL DEFAULT false,
    visited      INT DEFAULT 0,

    CONSTRAINT fk_community_follows_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_community_follows_community
         FOREIGN KEY(community_id)
             REFERENCES communities(id)
);
