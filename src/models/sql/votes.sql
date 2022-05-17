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

-----Reposts----
-------------------------
CREATE TABLE photo_reposts (
  id           SERIAL PRIMARY KEY,
  photo_id     INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_photo_reposts_user
      FOREIGN KEY(photo_id)
          REFERENCES photos(id),

  CONSTRAINT fk_photo_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_photo_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);

CREATE TABLE good_reposts (
  id           SERIAL PRIMARY KEY,
  good_id      INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_good_reposts_user
      FOREIGN KEY(good_id)
          REFERENCES goods(id),

  CONSTRAINT fk_good_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_good_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);

CREATE TABLE video_reposts (
  id           SERIAL PRIMARY KEY,
  video_id      INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_video_reposts_user
      FOREIGN KEY(video_id)
          REFERENCES videos(id),

  CONSTRAINT fk_video_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_video_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);

CREATE TABLE doc_reposts (
  id           SERIAL PRIMARY KEY,
  doc_id       INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_doc_reposts_user
      FOREIGN KEY(doc_id)
          REFERENCES docs(id),

  CONSTRAINT fk_doc_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_doc_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);

CREATE TABLE music_reposts (
  id           SERIAL PRIMARY KEY,
  music_id     INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_music_reposts_user
      FOREIGN KEY(music_id)
          REFERENCES musics(id),

  CONSTRAINT fk_music_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_music_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);

CREATE TABLE survey_reposts (
  id           SERIAL PRIMARY KEY,
  survey_id    INT NOT NULL,
  user_id      INT,
  community_id INT,
  message      BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_survey_reposts_user
      FOREIGN KEY(survey_id)
          REFERENCES surveys(id),

  CONSTRAINT fk_survey_reposts_user
      FOREIGN KEY(user_id)
          REFERENCES users(id),

  CONSTRAINT fk_survey_reposts_community
      FOREIGN KEY(community_id)
          REFERENCES communitys(id)
);
