table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
    }
}

table! {
    chat_ie_settings (id) {
        id -> Int4,
        chat_user_id -> Int4,
        can_add_in_chat -> Nullable<Char>,
        can_add_fix -> Nullable<Char>,
        can_send_mention -> Nullable<Char>,
        can_add_admin -> Nullable<Char>,
        can_add_design -> Nullable<Char>,
        can_see_settings -> Nullable<Char>,
        can_see_log -> Nullable<Char>,
    }
}

table! {
    chat_users (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        types -> Char,
        is_administrator -> Bool,
        created -> Timestamp,
        no_disturb -> Nullable<Timestamp>,
    }
}

table! {
    chats (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        types -> Int2,
        image -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        position -> Int2,
        members -> Int4,
        created -> Timestamp,
        can_add_members -> Char,
        can_fix_item -> Char,
        can_mention -> Char,
        can_add_admin -> Char,
        can_add_design -> Char,
        can_see_settings -> Char,
        can_see_log -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    communities_memberships (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        is_administrator -> Bool,
        is_moderator -> Bool,
        is_editor -> Bool,
        is_advertiser -> Bool,
        created -> Timestamp,
        visited -> Int4,
    }
}

table! {
    community_banner_users (id) {
        id -> Int4,
        community_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    community_categorys (id) {
        id -> Int4,
        name -> Varchar,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    community_doc_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        doc_list_id -> Int4,
    }
}

table! {
    community_doc_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_follows (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        view -> Bool,
        visited -> Int4,
    }
}

table! {
    community_good_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        good_list_id -> Int4,
    }
}

table! {
    community_good_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_good_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    community_infos (id) {
        id -> Int4,
        community_id -> Int4,
        posts -> Int4,
        members -> Int4,
        photos -> Int4,
        goods -> Int4,
        tracks -> Int4,
        videos -> Int4,
        docs -> Int4,
        articles -> Int4,
        survey -> Int4,
        planners -> Int4,
        avatar_id -> Nullable<Int4>,
    }
}

table! {
    community_invites (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        invite_creator -> Int4,
    }
}

table! {
    community_music_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        music_list_id -> Int4,
    }
}

table! {
    community_music_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_music_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        repost -> Bool,
    }
}

table! {
    community_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        community_invite -> Bool,
    }
}

table! {
    community_photo_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        photo_list_id -> Int4,
    }
}

table! {
    community_photo_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_photo_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    community_post_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    community_post_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_post_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    community_privates (id) {
        id -> Int4,
        community_id -> Int4,
        can_see_member -> Char,
        can_see_info -> Char,
        can_send_message -> Char,
        can_see_post -> Char,
        can_see_photo -> Char,
        can_see_good -> Char,
        can_see_video -> Char,
        can_see_music -> Char,
        can_see_planner -> Char,
        can_see_doc -> Char,
        can_see_survey -> Char,
        can_see_settings -> Char,
        can_see_log -> Char,
        can_see_stat -> Char,
        can_see_forum -> Char,
    }
}

table! {
    community_reposts (id) {
        id -> Int4,
        community_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    community_subcategorys (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    community_survey_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        survey_list_id -> Int4,
    }
}

table! {
    community_survey_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_survey_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        vote -> Bool,
    }
}

table! {
    community_video_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        video_list_id -> Int4,
    }
}

table! {
    community_video_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_video_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    community_visible_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_info -> Nullable<Char>,
        can_see_community -> Nullable<Char>,
        can_see_member -> Nullable<Char>,
        can_send_message -> Nullable<Char>,
        can_add_in_chat -> Nullable<Char>,
        can_see_doc -> Nullable<Char>,
        can_see_music -> Nullable<Char>,
        can_see_survey -> Nullable<Char>,
        can_see_post -> Nullable<Char>,
        can_see_post_comment -> Nullable<Char>,
        can_see_photo -> Nullable<Char>,
        can_see_photo_comment -> Nullable<Char>,
        can_see_good -> Nullable<Char>,
        can_see_good_comment -> Nullable<Char>,
        can_see_video -> Nullable<Char>,
        can_see_video_comment -> Nullable<Char>,
        can_see_planner -> Nullable<Char>,
        can_see_planner_comment -> Nullable<Char>,
        can_see_forum -> Nullable<Char>,
        can_see_forum_comment -> Nullable<Char>,
    }
}

table! {
    community_work_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_copy_post -> Nullable<Char>,
        can_copy_photo -> Nullable<Char>,
        can_copy_good -> Nullable<Char>,
        can_copy_video -> Nullable<Char>,
        can_copy_planner -> Nullable<Char>,
        can_copy_doc -> Nullable<Char>,
        can_copy_music -> Nullable<Char>,
        can_copy_survey -> Nullable<Char>,
        can_work_post -> Nullable<Char>,
        can_work_photo -> Nullable<Char>,
        can_work_good -> Nullable<Char>,
        can_work_video -> Nullable<Char>,
        can_work_planner -> Nullable<Char>,
        can_work_doc -> Nullable<Char>,
        can_work_music -> Nullable<Char>,
        can_work_survey -> Nullable<Char>,
    }
}

table! {
    communitys (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        types -> Int2,
        perm -> Char,
        level -> Int2,
        link -> Varchar,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        cover -> Nullable<Varchar>,
        community_subcategory_id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
    }
}

table! {
    custom_links (id) {
        id -> Int4,
        link -> Varchar,
        owner -> Int2,
    }
}

table! {
    design_settings (id) {
        id -> Int4,
        user_id -> Int4,
        background -> Char,
    }
}

table! {
    doc_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        doc_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        create_item -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    doc_list_reposts (id) {
        id -> Int4,
        doc_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    doc_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
    }
}

table! {
    doc_reposts (id) {
        id -> Int4,
        doc_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    docs (id) {
        id -> Int4,
        title -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        doc_list_id -> Int4,
        types -> Char,
        types_2 -> Char,
        file -> Varchar,
        created -> Timestamp,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
    }
}

table! {
    featured_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    follows (id) {
        id -> Int4,
        user_id -> Int4,
        followed_user -> Int4,
        view -> Bool,
        visited -> Int4,
    }
}

table! {
    follows_visible_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_info -> Nullable<Char>,
        can_see_community -> Nullable<Char>,
        can_see_friend -> Nullable<Char>,
        can_send_message -> Nullable<Char>,
        can_add_in_chat -> Nullable<Char>,
        can_see_doc -> Nullable<Char>,
        can_see_music -> Nullable<Char>,
        can_see_survey -> Nullable<Char>,
        can_see_post -> Nullable<Char>,
        can_see_post_comment -> Nullable<Char>,
        can_see_photo -> Nullable<Char>,
        can_see_photo_comment -> Nullable<Char>,
        can_see_good -> Nullable<Char>,
        can_see_good_comment -> Nullable<Char>,
        can_see_video -> Nullable<Char>,
        can_see_video_comment -> Nullable<Char>,
        can_see_planner -> Nullable<Char>,
        can_see_planner_comment -> Nullable<Char>,
    }
}

table! {
    follows_work_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_copy_post -> Nullable<Char>,
        can_copy_photo -> Nullable<Char>,
        can_copy_good -> Nullable<Char>,
        can_copy_video -> Nullable<Char>,
        can_copy_planner -> Nullable<Char>,
        can_copy_doc -> Nullable<Char>,
        can_copy_music -> Nullable<Char>,
        can_copy_survey -> Nullable<Char>,
        can_work_post -> Nullable<Char>,
        can_work_photo -> Nullable<Char>,
        can_work_good -> Nullable<Char>,
        can_work_video -> Nullable<Char>,
        can_work_planner -> Nullable<Char>,
        can_work_doc -> Nullable<Char>,
        can_work_music -> Nullable<Char>,
        can_work_survey -> Nullable<Char>,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_user_id -> Int4,
        visited -> Int4,
    }
}

table! {
    friends_visible_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_info -> Nullable<Char>,
        can_see_community -> Nullable<Char>,
        can_see_friend -> Nullable<Char>,
        can_send_message -> Nullable<Char>,
        can_add_in_chat -> Nullable<Char>,
        can_see_doc -> Nullable<Char>,
        can_see_music -> Nullable<Char>,
        can_see_survey -> Nullable<Char>,
        can_see_post -> Nullable<Char>,
        can_see_post_comment -> Nullable<Char>,
        can_see_photo -> Nullable<Char>,
        can_see_photo_comment -> Nullable<Char>,
        can_see_good -> Nullable<Char>,
        can_see_good_comment -> Nullable<Char>,
        can_see_video -> Nullable<Char>,
        can_see_video_comment -> Nullable<Char>,
        can_see_planner -> Nullable<Char>,
        can_see_planner_comment -> Nullable<Char>,
        can_see_all -> Nullable<Char>,
    }
}

table! {
    friends_work_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_copy_post -> Nullable<Char>,
        can_copy_photo -> Nullable<Char>,
        can_copy_good -> Nullable<Char>,
        can_copy_video -> Nullable<Char>,
        can_copy_planner -> Nullable<Char>,
        can_copy_doc -> Nullable<Char>,
        can_copy_music -> Nullable<Char>,
        can_copy_survey -> Nullable<Char>,
        can_work_post -> Nullable<Char>,
        can_work_photo -> Nullable<Char>,
        can_work_good -> Nullable<Char>,
        can_work_video -> Nullable<Char>,
        can_work_planner -> Nullable<Char>,
        can_work_doc -> Nullable<Char>,
        can_work_music -> Nullable<Char>,
        can_work_survey -> Nullable<Char>,
    }
}

table! {
    good_categories (id) {
        id -> Int4,
        name -> Varchar,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    good_comment_reactions (id) {
        id -> Int4,
        good_comment_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    good_comment_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        good_comment_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    good_comments (id) {
        id -> Int4,
        good_id -> Int4,
        user_id -> Int4,
        sticker_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        types -> Char,
        created -> Timestamp,
        repost -> Int4,
        reactions -> Int4,
    }
}

table! {
    good_images (id) {
        id -> Int4,
        good_id -> Int4,
        src -> Text,
    }
}

table! {
    good_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        good_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        can_see_comment -> Nullable<Char>,
        create_item -> Nullable<Char>,
        create_comment -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    good_list_reposts (id) {
        id -> Int4,
        good_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    good_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    good_reactions (id) {
        id -> Int4,
        good_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    good_reposts (id) {
        id -> Int4,
        good_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    good_subcategories (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    good_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        good_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    goods (id) {
        id -> Int4,
        title -> Varchar,
        community_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
        user_id -> Int4,
        good_list_id -> Int4,
        price -> Nullable<Int4>,
        types -> Char,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        comment_enabled -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        reactions -> Int4,
    }
}

table! {
    ip_users (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Varchar,
    }
}

table! {
    list_user_communities_keys (id) {
        id -> Int4,
        types -> Char,
        name -> Varchar,
        owner -> Int4,
    }
}

table! {
    message_options (id) {
        id -> Int4,
        message_id -> Int4,
        user_id -> Int4,
        is_deleted -> Bool,
        is_favourite -> Bool,
    }
}

table! {
    message_reactions (id) {
        id -> Int4,
        message_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    message_transfers (id) {
        id -> Int4,
        message_id -> Int4,
        transfer_id -> Int4,
    }
}

table! {
    message_versions (id) {
        id -> Int4,
        message_id -> Int4,
        sticker_id -> Nullable<Int4>,
        repost_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        created -> Timestamp,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
    }
}

table! {
    message_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        message_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        parent_id -> Nullable<Int4>,
        sticker_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        created -> Timestamp,
        content -> Nullable<Varchar>,
        unread -> Bool,
        types -> Int2,
        attach -> Nullable<Varchar>,
        voice -> Nullable<Varchar>,
        reactions -> Int4,
    }
}

table! {
    moderated_logs (id) {
        id -> Int4,
        user_id -> Int4,
        object_id -> Int4,
        action -> Char,
        description -> Nullable<Varchar>,
        types -> Int2,
        created -> Timestamp,
        time_to_suspend -> Nullable<Timestamp>,
    }
}

table! {
    moderated_penalties (id) {
        id -> Int4,
        user_id -> Int4,
        moderated_id -> Int4,
        expiration -> Nullable<Timestamp>,
        types -> Int2,
        object_id -> Int4,
        status -> Char,
        created -> Timestamp,
    }
}

table! {
    moderated_reports (id) {
        id -> Int4,
        user_id -> Int4,
        moderated_id -> Int4,
        description -> Nullable<Varchar>,
        types -> Char,
        created -> Timestamp,
    }
}

table! {
    moderateds (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        verified -> Bool,
        status -> Char,
        types -> Int2,
        object_id -> Int4,
        created -> Timestamp,
        count -> Int4,
    }
}

table! {
    music_albums (id) {
        id -> Int4,
        name -> Varchar,
        artist_id -> Nullable<Int4>,
        user_id -> Int4,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
    }
}

table! {
    music_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        music_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        create_item -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    music_list_reposts (id) {
        id -> Int4,
        music_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    music_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
    }
}

table! {
    music_reposts (id) {
        id -> Int4,
        music_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    musics (id) {
        id -> Int4,
        title -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        music_list_id -> Int4,
        genre_id -> Nullable<Int4>,
        album_id -> Nullable<Int4>,
        types -> Char,
        file -> Varchar,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
    }
}

table! {
    news_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        recipient_id -> Nullable<Int4>,
        user_id -> Int4,
        created -> Timestamp,
        verb -> Varchar,
        status -> Char,
        types -> Int2,
        object_id -> Int4,
        community_id -> Nullable<Int4>,
        action_community_id -> Nullable<Int4>,
        user_set_id -> Nullable<Int4>,
        object_set_id -> Nullable<Int4>,
    }
}

table! {
    notify_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    phone_codes (id) {
        id -> Int4,
        phone -> Varchar,
        code -> Int4,
    }
}

table! {
    photo_comment_reactions (id) {
        id -> Int4,
        photo_comment_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    photo_comment_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        photo_comment_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    photo_comments (id) {
        id -> Int4,
        photo_id -> Int4,
        user_id -> Int4,
        sticker_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        created -> Timestamp,
        types -> Char,
        repost -> Int4,
        reactions -> Int4,
    }
}

table! {
    photo_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        photo_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        can_see_comment -> Nullable<Char>,
        create_item -> Nullable<Char>,
        create_comment -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    photo_list_reposts (id) {
        id -> Int4,
        photo_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    photo_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        cover_photo -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    photo_reactions (id) {
        id -> Int4,
        photo_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    photo_reposts (id) {
        id -> Int4,
        photo_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    photo_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        photo_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    photos (id) {
        id -> Int4,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        photo_list_id -> Int4,
        types -> Char,
        preview -> Varchar,
        file -> Varchar,
        description -> Nullable<Varchar>,
        comment_enabled -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        reactions -> Int4,
    }
}

table! {
    post_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
    }
}

table! {
    post_comment_reactions (id) {
        id -> Int4,
        post_comment_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    post_comment_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        post_comment_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    post_comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        sticker_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        types -> Char,
        created -> Timestamp,
        repost -> Int4,
        reactions -> Int4,
    }
}

table! {
    post_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        can_see_comment -> Nullable<Char>,
        create_item -> Nullable<Char>,
        create_comment -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    post_list_reposts (id) {
        id -> Int4,
        post_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    post_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    post_reactions (id) {
        id -> Int4,
        post_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    post_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        post_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Nullable<Varchar>,
        community_id -> Nullable<Int4>,
        post_categorie_id -> Nullable<Int4>,
        user_id -> Int4,
        post_list_id -> Int4,
        types -> Char,
        attach -> Nullable<Varchar>,
        comment_enabled -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        is_signature -> Bool,
        parent_id -> Nullable<Int4>,
        reactions -> Int4,
    }
}

table! {
    reactions (id) {
        id -> Int4,
        types -> Int2,
        image -> Varchar,
        gif -> Varchar,
        name -> Varchar,
        is_active -> Bool,
        position -> Int2,
    }
}

table! {
    smile_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        description -> Nullable<Varchar>,
    }
}

table! {
    smiles (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        smile_categorie_id -> Int4,
        image -> Varchar,
    }
}

table! {
    sound_genres (id) {
        id -> Int4,
        name -> Varchar,
        count -> Int4,
        copy -> Int4,
    }
}

table! {
    staff_logs (id) {
        id -> Int4,
        types -> Int2,
        action -> Char,
        manager_id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
    }
}

table! {
    sticker_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        user_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

table! {
    stickers (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        sticker_categorie_id -> Int4,
        image -> Varchar,
    }
}

table! {
    support_user_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        manager_id -> Int4,
    }
}

table! {
    support_users (id) {
        id -> Int4,
        manager_id -> Int4,
        level -> Int2,
        points -> Int4,
        chats -> Int2,
        created -> Timestamp,
    }
}

table! {
    survey_answers (id) {
        id -> Int4,
        content -> Varchar,
        survey_id -> Int4,
        vote -> Int4,
        position -> Int4,
    }
}

table! {
    survey_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        survey_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        create_item -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    survey_list_reposts (id) {
        id -> Int4,
        survey_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    survey_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
    }
}

table! {
    survey_reposts (id) {
        id -> Int4,
        survey_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    survey_votes (id) {
        id -> Int4,
        user_id -> Int4,
        survey_answer_id -> Int4,
        survey_id -> Int4,
    }
}

table! {
    surveys (id) {
        id -> Int4,
        title -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        survey_list_id -> Int4,
        types -> Char,
        image -> Nullable<Varchar>,
        is_anonymous -> Bool,
        is_multiple -> Bool,
        is_no_edited -> Bool,
        time_end -> Nullable<Timestamp>,
        created -> Timestamp,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        vote -> Int4,
    }
}

table! {
    user_anketas (id) {
        id -> Int4,
        user_id -> Int4,
        political_preferences -> Nullable<Varchar>,
        worldview -> Nullable<Varchar>,
        mainthing_in_life -> Nullable<Varchar>,
        mainthing_in_people -> Nullable<Varchar>,
        attitude_to_smoking -> Nullable<Varchar>,
        attitude_to_alcohol -> Nullable<Varchar>,
        inspiration -> Nullable<Varchar>,
    }
}

table! {
    user_blocks (id) {
        id -> Int4,
        user_block_i -> Int4,
        blocked_user_id -> Int4,
    }
}

table! {
    user_brother_sisters (id) {
        id -> Int4,
        brother_user_i -> Int4,
        brother_target_id -> Int4,
    }
}

table! {
    user_children_ones (id) {
        id -> Int4,
        child_user_i -> Int4,
        child_id -> Int4,
    }
}

table! {
    user_colleagues_ones (id) {
        id -> Int4,
        user_colleague_i -> Int4,
        colleague_id -> Int4,
    }
}

table! {
    user_dad_ones (id) {
        id -> Int4,
        dad_user_i -> Int4,
        dad_id -> Int4,
    }
}

table! {
    user_delete_anketas (id) {
        id -> Int4,
        user_id -> Int4,
        answer -> Char,
        other -> Nullable<Varchar>,
        created -> Timestamp,
    }
}

table! {
    user_doc_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        doc_list_id -> Int4,
    }
}

table! {
    user_doc_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_good_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        good_list_id -> Int4,
    }
}

table! {
    user_good_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_good_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    user_grandsons_ones (id) {
        id -> Int4,
        grandson_user_i -> Int4,
        grandson_id -> Int4,
    }
}

table! {
    user_locations (id) {
        id -> Int4,
        user_id -> Int4,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
    }
}

table! {
    user_love_statuss (id) {
        id -> Int4,
        user_id -> Int4,
        male_status -> Nullable<Varchar>,
        female_status -> Nullable<Varchar>,
    }
}

table! {
    user_mom_ones (id) {
        id -> Int4,
        mom_user_i -> Int4,
        mom_id -> Int4,
    }
}

table! {
    user_music_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        music_list_id -> Int4,
    }
}

table! {
    user_music_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_music_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        repost -> Bool,
    }
}

table! {
    user_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        user_invite -> Bool,
    }
}

table! {
    user_partner_ones (id) {
        id -> Int4,
        partner_user_i -> Int4,
        partner_id -> Int4,
    }
}

table! {
    user_photo_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        photo_list_id -> Int4,
    }
}

table! {
    user_photo_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_photo_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    user_populate_smiles (id) {
        id -> Int4,
        user_id -> Int4,
        smile_id -> Int4,
        count -> Int4,
    }
}

table! {
    user_populate_stickers (id) {
        id -> Int4,
        user_id -> Int4,
        sticker_id -> Int4,
        count -> Int4,
    }
}

table! {
    user_post_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    user_post_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_post_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    user_privates (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_all -> Char,
        can_see_community -> Char,
        can_see_info -> Char,
        can_see_friend -> Char,
        can_send_message -> Char,
        can_add_in_chat -> Char,
        can_see_post -> Char,
        can_see_photo -> Char,
        can_see_good -> Char,
        can_see_video -> Char,
        can_see_music -> Char,
        can_see_planner -> Char,
        can_see_doc -> Char,
        can_see_survey -> Char,
    }
}

table! {
    user_profile_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        community_invite -> Bool,
    }
}

table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Int4,
        posts -> Int4,
        views_post -> Int4,
        friends -> Int4,
        follows -> Int4,
        communities -> Int4,
        photos -> Int4,
        goods -> Int4,
        docs -> Int4,
        tracks -> Int4,
        videos -> Int4,
        articles -> Int4,
        planners -> Int4,
        avatar_id -> Nullable<Int4>,
        activity -> Nullable<Varchar>,
        interests -> Nullable<Varchar>,
        favorite_music -> Nullable<Varchar>,
        favorite_films -> Nullable<Varchar>,
        favorite_books -> Nullable<Varchar>,
        favorite_game -> Nullable<Varchar>,
        about -> Nullable<Varchar>,
        survey -> Int4,
        playlist -> Int4,
    }
}

table! {
    user_reposts (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    user_survey_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        survey_list_id -> Int4,
    }
}

table! {
    user_survey_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_survey_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        vote -> Bool,
        repost -> Bool,
    }
}

table! {
    user_video_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        video_list_id -> Int4,
    }
}

table! {
    user_video_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_video_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        types -> Int2,
        gender -> Char,
        device -> Char,
        language -> Char,
        perm -> Int2,
        level -> Int2,
        password -> Varchar,
        link -> Varchar,
        city -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        birthday -> Date,
        last_activity -> Timestamp,
    }
}

table! {
    video_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int4,
    }
}

table! {
    video_comment_reactions (id) {
        id -> Int4,
        video_comment_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    video_comment_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        video_comment_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    video_comments (id) {
        id -> Int4,
        video_id -> Int4,
        user_id -> Int4,
        sticker_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        types -> Char,
        attach -> Nullable<Varchar>,
        created -> Timestamp,
        repost -> Int4,
        reactions -> Int4,
    }
}

table! {
    video_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        video_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        can_see_comment -> Nullable<Char>,
        create_item -> Nullable<Char>,
        create_comment -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    video_list_reposts (id) {
        id -> Int4,
        video_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    video_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    video_reactions (id) {
        id -> Int4,
        video_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    video_reposts (id) {
        id -> Int4,
        video_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

table! {
    video_votes (id) {
        id -> Int4,
        vote -> Int2,
        user_id -> Int4,
        video_id -> Int4,
        reaction -> Int2,
    }
}

table! {
    videos (id) {
        id -> Int4,
        title -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        video_list_id -> Int4,
        types -> Char,
        preview -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        file -> Varchar,
        description -> Nullable<Varchar>,
        comment_enabled -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        category_id -> Nullable<Int4>,
        reactions -> Int4,
    }
}

table! {
    wall_objects (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        verb -> Varchar,
        status -> Char,
        types -> Int2,
        object_id -> Int4,
        community_id -> Nullable<Int4>,
        action_community_id -> Nullable<Int4>,
        user_set_id -> Nullable<Int4>,
        object_set_id -> Nullable<Int4>,
    }
}
