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
        position -> Nullable<Int2>,
        members -> Nullable<Int4>,
        created -> Timestamp,
        can_add_members -> Char,
        can_fix_item -> Char,
        can_mention -> Char,
        can_add_admin -> Char,
        can_add_design -> Char,
        can_see_settings -> Char,
        can_see_log -> Char,
    }
}

table! {
    color_settings (id) {
        id -> Int4,
        user_id -> Int4,
        color -> Varchar,
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
    community_categorys (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        position -> Nullable<Int2>,
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
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
        types -> Char,
    }
}

table! {
    community_follows (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        view -> Bool,
        visited -> Nullable<Int4>,
    }
}

table! {
    community_good_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
    }
}

table! {
    community_infos (id) {
        id -> Int4,
        community_id -> Int4,
        posts -> Nullable<Int4>,
        members -> Nullable<Int4>,
        photos -> Nullable<Int4>,
        goods -> Nullable<Int4>,
        tracks -> Nullable<Int4>,
        videos -> Nullable<Int4>,
        docs -> Nullable<Int4>,
        articles -> Nullable<Int4>,
        survey -> Nullable<Int4>,
    }
}

table! {
    community_music_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
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
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
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
    }
}

table! {
    community_subcategorys (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        category_id -> Int4,
        avatar -> Nullable<Varchar>,
        position -> Nullable<Int2>,
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
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
    }
}

table! {
    communitys (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        status -> Char,
        types -> Int2,
        perm -> Char,
        level -> Nullable<Int2>,
        have_link -> Nullable<Varchar>,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        cover -> Nullable<Varchar>,
        community_subcategory_id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
    }
}

table! {
    connect_ie_settings (id) {
        id -> Int4,
        friend_id -> Int4,
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
        can_copy_post -> Nullable<Char>,
        can_copy_photo -> Nullable<Char>,
        can_copy_good -> Nullable<Char>,
        can_copy_video -> Nullable<Char>,
        can_copy_planner -> Nullable<Char>,
        can_copy_doc -> Nullable<Char>,
        can_copy_music -> Nullable<Char>,
        can_copy_survey -> Nullable<Char>,
        can_create_post -> Nullable<Char>,
        can_create_photo -> Nullable<Char>,
        can_create_good -> Nullable<Char>,
        can_create_video -> Nullable<Char>,
        can_create_planner -> Nullable<Char>,
        can_create_doc -> Nullable<Char>,
        can_create_music -> Nullable<Char>,
        can_create_survey -> Nullable<Char>,
    }
}

table! {
    custom_links (id) {
        id -> Int4,
        link -> Varchar,
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
    doc_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Char,
        description -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
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
        view -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
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
    follow_ie_settings (id) {
        id -> Int4,
        follow_id -> Int4,
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
        can_copy_post -> Nullable<Char>,
        can_copy_photo -> Nullable<Char>,
        can_copy_good -> Nullable<Char>,
        can_copy_video -> Nullable<Char>,
        can_copy_planner -> Nullable<Char>,
        can_copy_doc -> Nullable<Char>,
        can_copy_music -> Nullable<Char>,
        can_copy_survey -> Nullable<Char>,
        can_create_post -> Nullable<Char>,
        can_create_photo -> Nullable<Char>,
        can_create_good -> Nullable<Char>,
        can_create_video -> Nullable<Char>,
        can_create_planner -> Nullable<Char>,
        can_create_doc -> Nullable<Char>,
        can_create_music -> Nullable<Char>,
        can_create_survey -> Nullable<Char>,
    }
}

table! {
    follows (id) {
        id -> Int4,
        user_id -> Int4,
        followed_user -> Int4,
        view -> Bool,
        visited -> Nullable<Int4>,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_user_id -> Int4,
        visited -> Nullable<Int4>,
    }
}

table! {
    ip_users (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Nullable<Varchar>,
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
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
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
    photo_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Char,
        description -> Nullable<Varchar>,
        cover_photo -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
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
        votes_on -> Bool,
        created -> Timestamp,
        comment -> Nullable<Int4>,
        view -> Nullable<Int4>,
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
    }
}

table! {
    post_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int2,
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
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
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
    post_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Char,
        description -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
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
        votes_on -> Bool,
        created -> Timestamp,
        comment -> Nullable<Int4>,
        view -> Nullable<Int4>,
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int4>,
    }
}

table! {
    smile_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int2,
        description -> Nullable<Varchar>,
    }
}

table! {
    smiles (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int2,
        smile_categorie_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
    }
}

table! {
    sticker_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int2,
        user_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
    }
}

table! {
    stickers (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int2,
        sticker_categorie_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
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
    survey_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Char,
        description -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
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
        view -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        vote -> Nullable<Int4>,
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
        answer -> Nullable<Char>,
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
        position -> Nullable<Int2>,
        types -> Char,
    }
}

table! {
    user_good_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Nullable<Int2>,
        types -> Char,
    }
}

table! {
    user_good_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Nullable<Bool>,
        comment_reply -> Nullable<Bool>,
        mention -> Nullable<Bool>,
        comment_mention -> Nullable<Bool>,
        repost -> Nullable<Bool>,
        liked -> Nullable<Bool>,
        disliked -> Nullable<Bool>,
        comment_liked -> Nullable<Bool>,
        comment_disliked -> Nullable<Bool>,
        comment_reply_liked -> Nullable<Bool>,
        comment_reply_disliked -> Nullable<Bool>,
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
    user_music_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Nullable<Int2>,
        types -> Char,
    }
}

table! {
    user_music_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        repost -> Nullable<Bool>,
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
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
    }
}

table! {
    user_populate_smiles (id) {
        id -> Int4,
        user_id -> Int4,
        smile_id -> Int4,
        count -> Nullable<Int4>,
    }
}

table! {
    user_populate_stickers (id) {
        id -> Int4,
        user_id -> Int4,
        sticker_id -> Int4,
        count -> Nullable<Int4>,
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
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
    }
}

table! {
    user_privates (id) {
        id -> Int4,
        user_id -> Int4,
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
        connection_request -> Nullable<Bool>,
        connection_confirmed -> Nullable<Bool>,
        community_invite -> Nullable<Bool>,
    }
}

table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Int4,
        posts -> Nullable<Int4>,
        views_post -> Nullable<Int4>,
        friends -> Nullable<Int4>,
        follows -> Nullable<Int4>,
        communities -> Nullable<Int4>,
        photos -> Nullable<Int4>,
        goods -> Nullable<Int4>,
        docs -> Nullable<Int4>,
        tracks -> Nullable<Int4>,
        videos -> Nullable<Int4>,
        articles -> Nullable<Int4>,
        _time -> Nullable<Timestamp>,
        height -> Nullable<Float8>,
        activity -> Nullable<Varchar>,
        interests -> Nullable<Varchar>,
        favorite_music -> Nullable<Varchar>,
        favorite_films -> Nullable<Varchar>,
        favorite_books -> Nullable<Varchar>,
        favorite_game -> Nullable<Varchar>,
        about -> Nullable<Varchar>,
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
        position -> Nullable<Int2>,
        types -> Char,
    }
}

table! {
    user_survey_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        vote -> Nullable<Bool>,
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
        position -> Nullable<Int2>,
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
        liked -> Bool,
        disliked -> Bool,
        comment_liked -> Bool,
        comment_disliked -> Bool,
        comment_reply_liked -> Bool,
        comment_reply_disliked -> Bool,
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
        have_link -> Nullable<Varchar>,
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
        name -> Nullable<Varchar>,
        position -> Int4,
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
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
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
    video_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Char,
        description -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
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
        votes_on -> Bool,
        created -> Timestamp,
        comment -> Nullable<Int4>,
        view -> Nullable<Int4>,
        liked -> Nullable<Int4>,
        disliked -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        copy -> Nullable<Int4>,
        position -> Nullable<Int2>,
    }
}

joinable!(chat_ie_settings -> chat_users (chat_user_id));
joinable!(chat_users -> chats (chat_id));
joinable!(chat_users -> users (user_id));
joinable!(chats -> communitys (community_id));
joinable!(chats -> users (user_id));
joinable!(color_settings -> users (user_id));
joinable!(communities_memberships -> communitys (community_id));
joinable!(communities_memberships -> users (user_id));
joinable!(community_doc_list_collections -> communitys (community_id));
joinable!(community_doc_list_collections -> doc_lists (doc_list_id));
joinable!(community_follows -> communitys (community_id));
joinable!(community_follows -> users (user_id));
joinable!(community_good_notifications -> communitys (community_id));
joinable!(community_infos -> communitys (community_id));
joinable!(community_music_notifications -> communitys (community_id));
joinable!(community_notifications -> communitys (community_id));
joinable!(community_photo_list_collections -> communitys (community_id));
joinable!(community_photo_list_collections -> photo_lists (photo_list_id));
joinable!(community_photo_notifications -> communitys (community_id));
joinable!(community_post_list_collections -> communitys (community_id));
joinable!(community_post_list_collections -> post_lists (post_list_id));
joinable!(community_post_notifications -> communitys (community_id));
joinable!(community_privates -> communitys (community_id));
joinable!(community_subcategorys -> community_categorys (category_id));
joinable!(community_survey_list_collections -> communitys (community_id));
joinable!(community_survey_list_collections -> survey_lists (survey_list_id));
joinable!(community_survey_notifications -> communitys (community_id));
joinable!(community_video_list_collections -> communitys (community_id));
joinable!(community_video_list_collections -> video_lists (video_list_id));
joinable!(community_video_notifications -> communitys (community_id));
joinable!(communitys -> community_subcategorys (community_subcategory_id));
joinable!(communitys -> users (user_id));
joinable!(connect_ie_settings -> friends (friend_id));
joinable!(doc_list_perms -> doc_lists (doc_list_id));
joinable!(doc_list_perms -> users (user_id));
joinable!(doc_lists -> communitys (community_id));
joinable!(doc_lists -> users (user_id));
joinable!(docs -> communitys (community_id));
joinable!(docs -> doc_lists (doc_list_id));
joinable!(docs -> users (user_id));
joinable!(follow_ie_settings -> follows (follow_id));
joinable!(ip_users -> users (user_id));
joinable!(message_options -> messages (message_id));
joinable!(message_options -> users (user_id));
joinable!(message_versions -> messages (message_id));
joinable!(messages -> chats (chat_id));
joinable!(messages -> posts (post_id));
joinable!(messages -> stickers (sticker_id));
joinable!(messages -> users (user_id));
joinable!(photo_comments -> photos (photo_id));
joinable!(photo_comments -> stickers (sticker_id));
joinable!(photo_comments -> users (user_id));
joinable!(photo_list_perms -> photo_lists (photo_list_id));
joinable!(photo_list_perms -> users (user_id));
joinable!(photo_lists -> communitys (community_id));
joinable!(photo_lists -> users (user_id));
joinable!(photos -> communitys (community_id));
joinable!(photos -> photo_lists (photo_list_id));
joinable!(photos -> users (user_id));
joinable!(post_comments -> posts (post_id));
joinable!(post_comments -> stickers (sticker_id));
joinable!(post_comments -> users (user_id));
joinable!(post_list_perms -> post_lists (post_list_id));
joinable!(post_list_perms -> users (user_id));
joinable!(post_lists -> communitys (community_id));
joinable!(post_lists -> users (user_id));
joinable!(posts -> communitys (community_id));
joinable!(posts -> post_categories (post_categorie_id));
joinable!(posts -> post_lists (post_list_id));
joinable!(posts -> users (user_id));
joinable!(smiles -> smile_categories (smile_categorie_id));
joinable!(stickers -> sticker_categories (sticker_categorie_id));
joinable!(survey_list_perms -> survey_lists (survey_list_id));
joinable!(survey_list_perms -> users (user_id));
joinable!(survey_lists -> communitys (community_id));
joinable!(survey_lists -> users (user_id));
joinable!(surveys -> communitys (community_id));
joinable!(surveys -> survey_lists (survey_list_id));
joinable!(surveys -> users (user_id));
joinable!(user_anketas -> users (user_id));
joinable!(user_delete_anketas -> users (user_id));
joinable!(user_doc_list_collections -> doc_lists (doc_list_id));
joinable!(user_doc_list_collections -> users (user_id));
joinable!(user_good_notifications -> users (user_id));
joinable!(user_locations -> users (user_id));
joinable!(user_love_statuss -> users (user_id));
joinable!(user_music_notifications -> users (user_id));
joinable!(user_notifications -> users (user_id));
joinable!(user_photo_list_collections -> photo_lists (photo_list_id));
joinable!(user_photo_list_collections -> users (user_id));
joinable!(user_photo_notifications -> users (user_id));
joinable!(user_populate_smiles -> smiles (smile_id));
joinable!(user_populate_smiles -> users (user_id));
joinable!(user_populate_stickers -> stickers (sticker_id));
joinable!(user_populate_stickers -> users (user_id));
joinable!(user_post_list_collections -> post_lists (post_list_id));
joinable!(user_post_list_collections -> users (user_id));
joinable!(user_post_notifications -> users (user_id));
joinable!(user_privates -> users (user_id));
joinable!(user_profile_notifications -> users (user_id));
joinable!(user_profiles -> users (user_id));
joinable!(user_survey_list_collections -> survey_lists (survey_list_id));
joinable!(user_survey_list_collections -> users (user_id));
joinable!(user_survey_notifications -> users (user_id));
joinable!(user_video_list_collections -> users (user_id));
joinable!(user_video_list_collections -> video_lists (video_list_id));
joinable!(user_video_notifications -> users (user_id));
joinable!(video_comments -> stickers (sticker_id));
joinable!(video_comments -> users (user_id));
joinable!(video_comments -> videos (video_id));
joinable!(video_list_perms -> users (user_id));
joinable!(video_list_perms -> video_lists (video_list_id));
joinable!(video_lists -> communitys (community_id));
joinable!(video_lists -> users (user_id));
joinable!(videos -> communitys (community_id));
joinable!(videos -> users (user_id));
joinable!(videos -> video_lists (video_list_id));

allow_tables_to_appear_in_same_query!(
    chat_ie_settings,
    chat_users,
    chats,
    color_settings,
    communities_memberships,
    community_categorys,
    community_doc_list_collections,
    community_doc_list_positions,
    community_follows,
    community_good_list_positions,
    community_good_notifications,
    community_infos,
    community_music_list_positions,
    community_music_notifications,
    community_notifications,
    community_photo_list_collections,
    community_photo_list_positions,
    community_photo_notifications,
    community_post_list_collections,
    community_post_list_positions,
    community_post_notifications,
    community_privates,
    community_subcategorys,
    community_survey_list_collections,
    community_survey_list_positions,
    community_survey_notifications,
    community_video_list_collections,
    community_video_list_positions,
    community_video_notifications,
    communitys,
    connect_ie_settings,
    custom_links,
    doc_list_perms,
    doc_lists,
    docs,
    featured_user_communities,
    follow_ie_settings,
    follows,
    friends,
    ip_users,
    list_user_communities_keys,
    message_options,
    message_transfers,
    message_versions,
    messages,
    news_user_communities,
    notify_user_communities,
    phone_codes,
    photo_comments,
    photo_list_perms,
    photo_lists,
    photos,
    post_categories,
    post_comments,
    post_list_perms,
    post_lists,
    posts,
    smile_categories,
    smiles,
    sticker_categories,
    stickers,
    survey_list_perms,
    survey_lists,
    surveys,
    user_anketas,
    user_blocks,
    user_brother_sisters,
    user_children_ones,
    user_colleagues_ones,
    user_dad_ones,
    user_delete_anketas,
    user_doc_list_collections,
    user_doc_list_positions,
    user_good_list_positions,
    user_good_notifications,
    user_grandsons_ones,
    user_locations,
    user_love_statuss,
    user_mom_ones,
    user_music_list_positions,
    user_music_notifications,
    user_notifications,
    user_partner_ones,
    user_photo_list_collections,
    user_photo_list_positions,
    user_photo_notifications,
    user_populate_smiles,
    user_populate_stickers,
    user_post_list_collections,
    user_post_list_positions,
    user_post_notifications,
    user_privates,
    user_profile_notifications,
    user_profiles,
    user_survey_list_collections,
    user_survey_list_positions,
    user_survey_notifications,
    user_video_list_collections,
    user_video_list_positions,
    user_video_notifications,
    users,
    video_categories,
    video_comments,
    video_list_perms,
    video_lists,
    videos,
);
