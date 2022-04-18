-- Your SQL goes here

ALTER TABLE new_friends_perms RENAME can_create_post TO c_post;
ALTER TABLE new_friends_perms RENAME can_create_photo TO c_photo;
ALTER TABLE new_friends_perms RENAME can_create_good TO c_good;
ALTER TABLE new_friends_perms RENAME can_create_video TO c_video;
ALTER TABLE new_friends_perms RENAME can_create_planner TO c_planner;
ALTER TABLE new_friends_perms RENAME can_create_doc TO c_doc;
ALTER TABLE new_friends_perms RENAME can_create_music TO c_music;
ALTER TABLE new_friends_perms RENAME can_create_survey TO c_survey;
