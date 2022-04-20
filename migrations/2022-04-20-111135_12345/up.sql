-- Your SQL goes here

ALTER TABLE community_visible_perms ADD COLUMN can_see_forum "char";
ALTER TABLE community_visible_perms ADD COLUMN can_see_forum_comment "char";
