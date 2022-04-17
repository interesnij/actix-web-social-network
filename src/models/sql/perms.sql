CREATE TABLE friends_perms (
    id                       SERIAL PRIMARY KEY,
    friend_id                INT NOT NULL,

    can_see_info             "char",
    can_see_community        "char",
    can_see_friend           "char",
    can_send_message         "char",
    can_add_in_chat          "char",
    can_see_doc              "char",
    can_see_music            "char",
    can_see_survey           "char",
    can_see_post             "char",
    can_see_post_comment     "char",
    can_see_photo            "char",
    can_see_photo_comment    "char",
    can_see_good             "char",
    can_see_good_comment     "char",
    can_see_video            "char",
    can_see_video_comment    "char",
    can_see_planner          "char",
    can_see_planner_comment  "char",

    can_copy_post            "char",
    can_copy_photo           "char",
    can_copy_good            "char",
    can_copy_video           "char",
    can_copy_planner         "char",
    can_copy_doc             "char",
    can_copy_music           "char",
    can_copy_survey          "char",

    can_create_post          "char",
    can_create_photo         "char",
    can_create_good          "char",
    can_create_video         "char",
    can_create_planner       "char",
    can_create_doc           "char",
    can_create_music         "char",
    can_create_survey        "char",

    CONSTRAINT fk_friends_perms
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

CREATE TABLE follows_perms (
    id                       SERIAL PRIMARY KEY,
    follow_id                INT NOT NULL,

    can_see_info             "char",
    can_see_community        "char",
    can_see_friend           "char",
    can_send_message         "char",
    can_add_in_chat          "char",
    can_see_doc              "char",
    can_see_music            "char",
    can_see_survey           "char",
    can_see_post             "char",
    can_see_post_comment     "char",
    can_see_photo            "char",
    can_see_photo_comment    "char",
    can_see_good             "char",
    can_see_good_comment     "char",
    can_see_video            "char",
    can_see_video_comment    "char",
    can_see_planner          "char",
    can_see_planner_comment  "char",

    can_copy_post            "char",
    can_copy_photo           "char",
    can_copy_good            "char",
    can_copy_video           "char",
    can_copy_planner         "char",
    can_copy_doc             "char",
    can_copy_music           "char",
    can_copy_survey          "char",

    can_create_post          "char",
    can_create_photo         "char",
    can_create_good          "char",
    can_create_video         "char",
    can_create_planner       "char",
    can_create_doc           "char",
    can_create_music         "char",
    can_create_survey        "char",

    CONSTRAINT fk_follows_perms
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);

CREATE TABLE community_members_perm (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,

    can_see_info            "char",
    can_see_member          "char",
    can_send_message        "char",
    can_see_doc             "char",
    can_see_music           "char",
    can_see_survey          "char",
    can_see_post            "char",
    can_see_post_comment    "char",
    can_see_photo           "char",
    can_see_photo_comment   "char",
    can_see_good            "char",
    can_see_good_comment    "char",
    can_see_video           "char",
    can_see_video_comment   "char",
    can_see_planner         "char",
    can_see_planner_comment "char",

    can_copy_post           "char",
    can_copy_photo          "char",
    can_copy_good           "char",
    can_copy_video          "char",
    can_copy_planner        "char",
    can_copy_doc            "char",
    can_copy_music          "char",
    can_copy_survey         "char",

    can_create_post         "char",
    can_create_photo        "char",
    can_create_good         "char",
    can_create_video        "char",
    can_create_planner      "char",
    can_create_doc          "char",
    can_create_music        "char",
    can_create_survey       "char",

    CONSTRAINT fk_community_members_perm
        FOREIGN KEY(community_member)
            REFERENCES communities_memberships(id)
);
