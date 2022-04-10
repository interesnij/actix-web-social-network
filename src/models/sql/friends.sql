
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

    can_see_info             CHAR NOT NULL,
    can_see_community        CHAR NOT NULL,
    can_see_friend           CHAR NOT NULL,
    can_send_message         CHAR NOT NULL,
    can_add_in_chat          CHAR NOT NULL,
    can_see_doc              CHAR NOT NULL,
    can_see_music            CHAR NOT NULL,
    can_see_survey           CHAR NOT NULL,
    can_see_post             CHAR NOT NULL,
    can_see_post_comment     CHAR NOT NULL,
    can_see_photo            CHAR NOT NULL,
    can_see_photo_comment    CHAR NOT NULL,
    can_see_good             CHAR NOT NULL,
    can_see_good_comment     CHAR NOT NULL,
    can_see_video            CHAR NOT NULL,
    can_see_video_comment    CHAR NOT NULL,
    can_see_planner          CHAR NOT NULL,
    can_see_planner_comment  CHAR NOT NULL,

    can_copy_post            CHAR NOT NULL,
    can_copy_photo           CHAR NOT NULL,
    can_copy_good            CHAR NOT NULL,
    can_copy_video           CHAR NOT NULL,
    can_copy_planner         CHAR NOT NULL,
    can_copy_doc             CHAR NOT NULL,
    can_copy_music           CHAR NOT NULL,
    can_copy_survey          CHAR NOT NULL,

    can_create_post          CHAR NOT NULL,
    can_create_photo         CHAR NOT NULL,
    can_create_good          CHAR NOT NULL,
    can_create_video         CHAR NOT NULL,
    can_create_planner       CHAR NOT NULL,
    can_create_doc           CHAR NOT NULL,
    can_create_music         CHAR NOT NULL,
    can_create_survey        CHAR NOT NULL,

    CONSTRAINT fk_connect_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES friends(id)
);
