
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

    can_see_info             CHAR(1) NOT NULL,
    can_see_community        CHAR(1) NOT NULL,
    can_see_friend           CHAR(1) NOT NULL,
    can_send_message         CHAR(1) NOT NULL,
    can_add_in_chat          CHAR(1) NOT NULL,
    can_see_doc              CHAR(1) NOT NULL,
    can_see_music            CHAR(1) NOT NULL,
    can_see_survey           CHAR(1) NOT NULL,
    can_see_post             CHAR(1) NOT NULL,
    can_see_post_comment     CHAR(1) NOT NULL,
    can_see_photo            CHAR(1) NOT NULL,
    can_see_photo_comment    CHAR(1) NOT NULL,
    can_see_good             CHAR(1) NOT NULL,
    can_see_good_comment     CHAR(1) NOT NULL,
    can_see_video            CHAR(1) NOT NULL,
    can_see_video_comment    CHAR(1) NOT NULL,
    can_see_planner          CHAR(1) NOT NULL,
    can_see_planner_comment  CHAR(1) NOT NULL,

    can_copy_post            CHAR(1) NOT NULL,
    can_copy_photo           CHAR(1) NOT NULL,
    can_copy_good            CHAR(1) NOT NULL,
    can_copy_video           CHAR(1) NOT NULL,
    can_copy_planner         CHAR(1) NOT NULL,
    can_copy_doc             CHAR(1) NOT NULL,
    can_copy_music           CHAR(1) NOT NULL,
    can_copy_survey          CHAR(1) NOT NULL,

    can_create_post          CHAR(1) NOT NULL,
    can_create_photo         CHAR(1) NOT NULL,
    can_create_good          CHAR(1) NOT NULL,
    can_create_video         CHAR(1) NOT NULL,
    can_create_planner       CHAR(1) NOT NULL,
    can_create_doc           CHAR(1) NOT NULL,
    can_create_music         CHAR(1) NOT NULL,
    can_create_survey        CHAR(1) NOT NULL,

    CONSTRAINT fk_connect_ie_settings
        FOREIGN KEY(user_id)
            REFERENCES friends(id)
);
