-- Your SQL goes here

CREATE TABLE post_votes (
  id      SERIAL PRIMARY KEY,
  vote    SMALLINT NOT NULL,
  user_id INT NOT NULL,
  post_id INT NOT NULL,

  CONSTRAINT fk_post_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_post_votes_post
      FOREIGN KEY(post_id)
          REFERENCES posts(id)
);

CREATE TABLE post_comment_votes (
  id              SERIAL PRIMARY KEY,
  vote            SMALLINT NOT NULL,
  user_id         INT NOT NULL,
  post_comment_id INT NOT NULL,

  CONSTRAINT fk_post_comment_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_post_comment_votes_post
      FOREIGN KEY(post_comment_id)
          REFERENCES post_comments(id)
);

-------------------------

CREATE TABLE photo_votes (
  id       SERIAL PRIMARY KEY,
  vote     SMALLINT NOT NULL,
  user_id  INT NOT NULL,
  photo_id INT NOT NULL,

  CONSTRAINT fk_photo_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_photo_votes_photo
      FOREIGN KEY(photo_id)
          REFERENCES photos(id)
);

CREATE TABLE photo_comment_votes (
  id               SERIAL PRIMARY KEY,
  vote             SMALLINT NOT NULL,
  user_id          INT NOT NULL,
  photo_comment_id INT NOT NULL,

  CONSTRAINT fk_photo_comment_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_photo_comment_votes_photo
      FOREIGN KEY(photo_comment_id)
          REFERENCES photo_comments(id)
);

-------------------------

CREATE TABLE good_votes (
  id       SERIAL PRIMARY KEY,
  vote     SMALLINT NOT NULL,
  user_id  INT NOT NULL,
  good_id  INT NOT NULL,

  CONSTRAINT fk_good_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_good_votes_good
      FOREIGN KEY(good_id)
          REFERENCES goods(id)
);

CREATE TABLE good_comment_votes (
  id               SERIAL PRIMARY KEY,
  vote             SMALLINT NOT NULL,
  user_id          INT NOT NULL,
  good_comment_id INT NOT NULL,

  CONSTRAINT fk_good_comment_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_good_comment_votes_good
      FOREIGN KEY(good_comment_id)
          REFERENCES good_comments(id)
);

-------------------------

CREATE TABLE video_votes (
  id       SERIAL PRIMARY KEY,
  vote     SMALLINT NOT NULL,
  user_id  INT NOT NULL,
  video_id INT NOT NULL,

  CONSTRAINT fk_video_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_video_votes_video
      FOREIGN KEY(video_id)
          REFERENCES videos(id)
);

CREATE TABLE video_comment_votes (
  id               SERIAL PRIMARY KEY,
  vote             SMALLINT NOT NULL,
  user_id          INT NOT NULL,
  video_comment_id INT NOT NULL,

  CONSTRAINT fk_video_comment_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_video_comment_votes_video
      FOREIGN KEY(video_comment_id)
          REFERENCES video_comments(id)
);

------------------------------

CREATE TABLE support_user_votes (
  id          SERIAL PRIMARY KEY,
  vote        SMALLINT NOT NULL,
  user_id     INT NOT NULL,
  manager_id  INT NOT NULL,

  CONSTRAINT fk_video_comment_votes_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_video_comment_votes_manager
      FOREIGN KEY(manager_id)
          REFERENCES users(id)
);

------------------------------
CREATE TABLE survey_answers (
    id           SERIAL PRIMARY KEY,
    content      VARCHAR (100) NOT NULL,
    survey_id    INT NOT NULL,
    vote         INT NOT NULL,
    position        INT NOT NULL,

   CONSTRAINT fk_survey_answers_survey
        FOREIGN KEY(survey_id)
            REFERENCES surveys(id)
);

CREATE TABLE survey_votes (
    id               SERIAL PRIMARY KEY,
    user_id          INT NOT NULL,
    survey_answer_id INT NOT NULL,

   CONSTRAINT fk_survey_votes_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_survey_votes_answer
        FOREIGN KEY(survey_answer_id)
            REFERENCES survey_answers(id)
);
