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
    community_doc_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Nullable<Int4>,
        position -> Nullable<Int2>,
        types -> Char,
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
    community_ie_settings (id) {
        id -> Int4,
        community_member -> Int4,
        can_see_info -> Nullable<Char>,
        can_see_member -> Nullable<Char>,
        can_send_message -> Nullable<Char>,
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
        can_add_post -> Nullable<Char>,
        can_add_photo -> Nullable<Char>,
        can_add_good -> Nullable<Char>,
        can_add_video -> Nullable<Char>,
        can_add_planner -> Nullable<Char>,
        can_add_doc -> Nullable<Char>,
        can_add_music -> Nullable<Char>,
        can_add_survey -> Nullable<Char>,
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
    custom_links (id) {
        id -> Int4,
        link -> Varchar,
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
        creator_id -> Nullable<Int4>,
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

joinable!(color_settings -> users (user_id));
joinable!(communities_memberships -> communitys (community_id));
joinable!(communities_memberships -> users (user_id));
joinable!(community_good_notifications -> communitys (community_id));
joinable!(community_ie_settings -> communities_memberships (community_member));
joinable!(community_infos -> communitys (community_id));
joinable!(community_music_notifications -> communitys (community_id));
joinable!(community_notifications -> communitys (community_id));
joinable!(community_photo_notifications -> communitys (community_id));
joinable!(community_post_notifications -> communitys (community_id));
joinable!(community_privates -> communitys (community_id));
joinable!(community_subcategorys -> community_categorys (category_id));
joinable!(community_survey_notifications -> communitys (community_id));
joinable!(community_video_notifications -> communitys (community_id));
joinable!(communitys -> community_subcategorys (community_subcategory_id));
joinable!(communitys -> users (user_id));
joinable!(ip_users -> users (user_id));
joinable!(smiles -> smile_categories (smile_categorie_id));
joinable!(stickers -> sticker_categories (sticker_categorie_id));
joinable!(user_anketas -> users (user_id));
joinable!(user_delete_anketas -> users (user_id));
joinable!(user_good_notifications -> users (user_id));
joinable!(user_locations -> users (user_id));
joinable!(user_love_statuss -> users (user_id));
joinable!(user_music_notifications -> users (user_id));
joinable!(user_notifications -> users (user_id));
joinable!(user_photo_notifications -> users (user_id));
joinable!(user_populate_smiles -> smiles (smile_id));
joinable!(user_populate_smiles -> users (user_id));
joinable!(user_populate_stickers -> stickers (sticker_id));
joinable!(user_populate_stickers -> users (user_id));
joinable!(user_post_notifications -> users (user_id));
joinable!(user_privates -> users (user_id));
joinable!(user_profile_notifications -> users (user_id));
joinable!(user_profiles -> users (user_id));
joinable!(user_survey_notifications -> users (user_id));
joinable!(user_video_notifications -> users (user_id));

allow_tables_to_appear_in_same_query!(
    color_settings,
    communities_memberships,
    community_categorys,
    community_doc_list_positions,
    community_good_list_positions,
    community_good_notifications,
    community_ie_settings,
    community_infos,
    community_music_list_positions,
    community_music_notifications,
    community_notifications,
    community_photo_list_positions,
    community_photo_notifications,
    community_post_list_positions,
    community_post_notifications,
    community_privates,
    community_subcategorys,
    community_survey_list_positions,
    community_survey_notifications,
    community_video_list_positions,
    community_video_notifications,
    communitys,
    custom_links,
    featured_user_communities,
    ip_users,
    list_user_communities_keys,
    news_user_communities,
    notify_user_communities,
    phone_codes,
    smile_categories,
    smiles,
    sticker_categories,
    stickers,
    user_anketas,
    user_blocks,
    user_brother_sisters,
    user_children_ones,
    user_colleagues_ones,
    user_dad_ones,
    user_delete_anketas,
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
    user_photo_list_positions,
    user_photo_notifications,
    user_populate_smiles,
    user_populate_stickers,
    user_post_list_positions,
    user_post_notifications,
    user_privates,
    user_profile_notifications,
    user_profiles,
    user_survey_list_positions,
    user_survey_notifications,
    user_video_list_positions,
    user_video_notifications,
    users,
);
