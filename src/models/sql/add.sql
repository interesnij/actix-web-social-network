-- добавляем поле в таблицу
ALTER TABLE serve
ADD is_default boolean not null default false;

-- удаляем таблицу
DROP TABLE dbo.PurchaseOrderDetail;

-- переименовываем поле
ALTER TABLE smiles RENAME COLUMN category_id TO smile_categories_id;
ALTER TABLE stickers RENAME COLUMN category_id TO sticker_categories_id;

-- меняем тип поля
ALTER TABLE Customers
ALTER COLUMN FirstName TYPE VARCHAR(50);

-- переименовываем таблицу
ALTER TALBE table_name RENAME TO new_table_name;

-- таблица не ставится почему то
CREATE TABLE community_member_ie_settings (
    id                      SERIAL PRIMARY KEY,
    community_member        INT NOT NULL,

    can_see_info            "char",
    can_see_member          "char",
    can_send_message        "char",
    can_see_doc             "char",
    can_see_music           "char",
    can_see_survey          "char",
    can_see_post            "char",
    can_see_post_comment    "char",
    can_see_photo           "char",
    can_see_photo_comment   "char",
    can_see_good            "char",
    can_see_good_comment    "char",
    can_see_video           "char",
    can_see_video_comment   "char",
    can_see_planner         "char",
    can_see_planner_comment "char",

    can_copy_post            "char",
    can_copy_photo           "char",
    can_copy_good            "char",
    can_copy_video           "char",
    can_copy_planner         "char",
    can_copy_doc             "char",
    can_copy_music           "char",
    can_copy_survey          "char",

    can_create_post         "char",
    can_create_photo        "char",
    can_create_good         "char",
    can_create_video        "char",
    can_create_planner      "char",
    can_create_doc          "char",
    can_create_music        "char",
    can_create_survey       "char",

    CONSTRAINT fk_community_ie_settings
        FOREIGN KEY(community_member)
            REFERENCES communities_memberships(id)
);
