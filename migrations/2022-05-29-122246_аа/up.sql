-- Your SQL goes here

ALTER TABLE community_video_notifications 
ADD COLUMN reactions BOOLEAN NOT NULL DEFAULT TRUE;
