-- Your SQL goes here

DROP TABLE friend_perms;

CREATE TABLE friend_perms2 (
    id                       SERIAL PRIMARY KEY,
    user_id                INT NOT NULL,

    can_see_info             "char",
    can_see_community        "char",
    can_see_friend           "char",
    can_send_message         "char",
    can_add_in_chat          "char",
    can_see_doc              "char",
    
   CONSTRAINT fk_friends_perms
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
