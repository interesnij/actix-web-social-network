-- Your SQL goes here

ALTER TABLE new_friends_perms ADD COLUMN can_create_post "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_photo "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_good "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_video "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_planner "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_doc "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_music "char";
ALTER TABLE new_friends_perms ADD COLUMN can_create_survey "char";
