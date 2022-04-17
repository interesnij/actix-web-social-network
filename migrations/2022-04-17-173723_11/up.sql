-- Your SQL goes here

DROP TABLE friends_perms;

CREATE TABLE friend_perms (
    id                       SERIAL PRIMARY KEY,
    user_id                  INT NOT NULL, 

    CONSTRAINT fk_friend_perms
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
