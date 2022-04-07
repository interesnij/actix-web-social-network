
-- подписчики -------
CREATE TABLE friends (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    target_user_id INT NOT NULL,
    visited INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_friends_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_friends_target_user
         FOREIGN KEY(target_user)
             REFERENCES users(id)
);
