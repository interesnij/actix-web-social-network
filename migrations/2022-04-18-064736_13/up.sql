-- Your SQL goes here

DROP TABLE friend_perms2;

CREATE TABLE friends_perms (
    id                       SERIAL PRIMARY KEY,
    user_id                  INT NOT NULL,

    can_see_info             "char",
    can_see_community        "char",
    can_see_friend           "char",
    can_send_message         "char",
    can_add_in_chat          "char",
    can_see_doc              "char"
);
