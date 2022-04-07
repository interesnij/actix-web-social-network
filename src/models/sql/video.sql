CREATE TABLE video_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    order INT NOT NULL DEFAULT 0
);


CREATE TABLE video_lists (
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    description TEXT,
    created TIMESTAMP NOT NULL,
    count INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,

    can_see_el INT NOT NULL DEFAULT 1,
    can_see_comment INT NOT NULL DEFAULT 1,
    create_el INT NOT NULL DEFAULT 7,
    create_comment INT NOT NULL DEFAULT 1,
    copy_el INT NOT NULL DEFAULT 1,

    CONSTRAINT fk_video_lists_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_video_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id)
);

CREATE TABLE videos (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    community_id INT,
    creator_id INT NOT NULL,
    list_id INT NOT NULL,
    _type VARCHAR NOT NULL,
    preview TEXT NOT NULL,
    image TEXT,
    file TEXT NOT NULL,
    description TEXT,
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    votes_on BOOLEAN NOT NULL DEFAULT true,
    created TIMESTAMP NOT NULL,

    comment INT NOT NULL DEFAULT 0,
    view INT NOT NULL DEFAULT 0,
    liked INT NOT NULL DEFAULT 0,
    disliked INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,
    copy INT NOT NULL DEFAULT 0,
    order INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_videos_creator
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_videos_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

    CONSTRAINT fk_videos_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_comments (
    id SERIAL PRIMARY KEY,
    item_id INT NOT NULL,
    creator_id INT NOT NULL,
    sticker_id INT,
    parent_id INT,
    _text TEXT NOT NULL,
    attach VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL,

    liked INT NOT NULL DEFAULT 0,
    disliked INT NOT NULL DEFAULT 0,
    repost INT NOT NULL DEFAULT 0,

    CONSTRAINT fk_video_comment
        FOREIGN KEY(item_id)
            REFERENCES videos(id),

    CONSTRAINT fk_user_video_comment
        FOREIGN KEY(creator_id)
            REFERENCES users(id),

    CONSTRAINT fk_video_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES video_comments(id)
);
CREATE INDEX video_comments_item_id_idx ON video_comments (item_id);
CREATE INDEX video_comments_creator_id_idx ON video_comments (creator_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_video_list_collections (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,

   CONSTRAINT fk_user_video_list_collections_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_user_video_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_video_list_collections (
    id SERIAL PRIMARY KEY,
    community_id INT,
    list_id INT,

   CONSTRAINT fk_community_video_list_collections_community
        FOREIGN KEY(community_id)
            REFERENCES communities(id),

   CONSTRAINT fk_community_video_list_collections_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);

CREATE TABLE video_list_perm (
    id SERIAL PRIMARY KEY,
    user_id INT,
    list_id INT,
    can_see_item INT DEFAULT 0,
    can_see_comment INT DEFAULT 0,
    create_item INT DEFAULT 0,
    create_comment INT DEFAULT 0,
    can_copy INT DEFAULT 0,

   CONSTRAINT fk_video_list_perm_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

   CONSTRAINT fk_video_list_perm_list
        FOREIGN KEY(list_id)
            REFERENCES video_lists(id)
);
