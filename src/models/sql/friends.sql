
-- подписчики -------
CREATE TABLE friends (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    target_user_id INT NOT NULL,
    visited        INT DEFAULT 0,

    CONSTRAINT fk_friends_user
         FOREIGN KEY(user_id)
             REFERENCES users(id),

    CONSTRAINT fk_friends_target_user
         FOREIGN KEY(target_user_id)
             REFERENCES users(id)
);

CREATE TABLE connect_ie_settings (
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

    CONSTRAINT fk_connect_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES friends(id)
);
