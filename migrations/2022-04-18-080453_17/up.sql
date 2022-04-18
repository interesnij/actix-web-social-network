-- Your SQL goes here

ALTER TABLE new_friends_perms ADD COLUMN can_copy_post "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_photo "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_good "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_video "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_planner "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_doc "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_music "char";
ALTER TABLE new_friends_perms ADD COLUMN can_copy_survey "char";
