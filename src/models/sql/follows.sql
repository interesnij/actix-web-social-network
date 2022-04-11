
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

CREATE TABLE follow_ie_settings (
    id                       SERIAL PRIMARY KEY,
    user_id                  INT NOT NULL,

    can_see_info             "char" NOT NULL,
    can_see_community        "char" NOT NULL,
    can_see_friend           "char" NOT NULL,
    can_send_message         "char" NOT NULL,
    can_add_in_chat          "char" NOT NULL,
    can_see_doc              "char" NOT NULL,
    can_see_music            "char" NOT NULL,
    can_see_survey           "char" NOT NULL,
    can_see_post             "char" NOT NULL,
    can_see_post_comment     "char" NOT NULL,
    can_see_photo            "char" NOT NULL,
    can_see_photo_comment    "char" NOT NULL,
    can_see_good             "char" NOT NULL,
    can_see_good_comment     "char" NOT NULL,
    can_see_video            "char" NOT NULL,
    can_see_video_comment    "char" NOT NULL,
    can_see_planner          "char" NOT NULL,
    can_see_planner_comment  "char" NOT NULL,

    can_copy_post            "char" NOT NULL,
    can_copy_photo           "char" NOT NULL,
    can_copy_good            "char" NOT NULL,
    can_copy_video           "char" NOT NULL,
    can_copy_planner         "char" NOT NULL,
    can_copy_doc             "char" NOT NULL,
    can_copy_music           "char" NOT NULL,
    can_copy_survey          "char" NOT NULL,

    can_create_post          "char" NOT NULL,
    can_create_photo         "char" NOT NULL,
    can_create_good          "char" NOT NULL,
    can_create_video         "char" NOT NULL,
    can_create_planner       "char" NOT NULL,
    can_create_doc           "char" NOT NULL,
    can_create_music         "char" NOT NULL,
    can_create_survey        "char" NOT NULL,

    CONSTRAINT fk_follow_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES follows(id)
);
