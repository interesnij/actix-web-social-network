table! {
    color_settings (id) {
        id -> Int4,
        user_id -> Int4,
        color -> Varchar,
    }
}

table! {
    custom_links (id) {
        id -> Int4,
        link -> Varchar,
    }
}

table! {
    featured_uc (id) {
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
    ip_user (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Nullable<Varchar>,
    }
}

table! {
    list_uc (id) {
        id -> Int4,
        types -> Nullable<Int4>,
        name -> Varchar,
        owner -> Int4,
    }
}

table! {
    news_uc (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    notify_uc (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community -> Nullable<Int4>,
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
        position -> Int4,
        description -> Nullable<Varchar>,
    }
}

table! {
    smiles (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int4,
        smile_categorie_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
    }
}

table! {
    sticker_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int4,
        creator_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
    }
}

table! {
    stickers (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        position -> Int4,
        sticker_categorie_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
    }
}

table! {
    user_anketa (id) {
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
        user_id -> Int4,
        blocked_user_id -> Int4,
    }
}

table! {
    user_brother_sister (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_children_one (id) {
        id -> Int4,
        user_id -> Int4,
        child_id -> Int4,
    }
}

table! {
    user_colleagues_one (id) {
        id -> Int4,
        user_id -> Int4,
        colleague_id -> Int4,
    }
}

table! {
    user_dad_one (id) {
        id -> Int4,
        user_id -> Int4,
        dad_id -> Int4,
    }
}

table! {
    user_delete_anketa (id) {
        id -> Int4,
        user_id -> Int4,
        answer -> Nullable<Bpchar>,
        other -> Nullable<Varchar>,
    }
}

table! {
    user_doc_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
    }
}

table! {
    user_good_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
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
    user_grandsons_one (id) {
        id -> Int4,
        user_id -> Int4,
        grandson_id -> Int4,
    }
}

table! {
    user_location (id) {
        id -> Int4,
        user_id -> Int4,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        city_lat -> Nullable<Float8>,
        city_lon -> Nullable<Float8>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
    }
}

table! {
    user_love_status (id) {
        id -> Int4,
        user_id -> Int4,
        male_status -> Nullable<Varchar>,
        female_status -> Nullable<Varchar>,
    }
}

table! {
    user_mom_one (id) {
        id -> Int4,
        user_id -> Int4,
        mom_id -> Int4,
    }
}

table! {
    user_music_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
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
    user_partner_one (id) {
        id -> Int4,
        user_id -> Int4,
        partner_id -> Int4,
    }
}

table! {
    user_photo_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
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
    user_post_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
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
    user_private (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_community -> Bpchar,
        can_see_info -> Bpchar,
        can_see_friend -> Bpchar,
        can_send_message -> Bpchar,
        can_add_in_chat -> Bpchar,
        can_see_post -> Bpchar,
        can_see_photo -> Bpchar,
        can_see_good -> Bpchar,
        can_see_video -> Bpchar,
        can_see_music -> Bpchar,
        can_see_planner -> Bpchar,
        can_see_doc -> Bpchar,
        can_see_survey -> Bpchar,
    }
}

table! {
    user_profile (id) {
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
    user_profile_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        connection_request -> Nullable<Bool>,
        connection_confirmed -> Nullable<Bool>,
        community_invite -> Nullable<Bool>,
    }
}

table! {
    user_survey_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
    }
}

table! {
    user_video_list_position (id) {
        id -> Int4,
        user_id -> Int4,
        list -> Int4,
        position -> Nullable<Int4>,
        types -> Bpchar,
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
        gender -> Bpchar,
        device -> Bpchar,
        language -> Bpchar,
        perm -> Int2,
<<<<<<< HEAD
        level -> Nullable<Int2>,
=======
        level -> Int2, 
>>>>>>> 3446212 (J)
        password -> Varchar,
        have_link -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        birthday -> Timestamp,
        last_activity -> Timestamp,
    }
}

joinable!(color_settings -> users (user_id));
joinable!(featured_uc -> list_uc (list_id));
joinable!(ip_user -> users (user_id));
joinable!(news_uc -> list_uc (list_id));
joinable!(notify_uc -> list_uc (list_id));
joinable!(smiles -> smile_categories (smile_categorie_id));
joinable!(stickers -> sticker_categories (sticker_categorie_id));
joinable!(user_anketa -> users (user_id));
joinable!(user_delete_anketa -> users (user_id));
joinable!(user_good_notifications -> users (user_id));
joinable!(user_location -> users (user_id));
joinable!(user_love_status -> users (user_id));
joinable!(user_music_notifications -> users (user_id));
joinable!(user_photo_notifications -> users (user_id));
joinable!(user_populate_smiles -> smiles (smile_id));
joinable!(user_populate_smiles -> users (user_id));
joinable!(user_populate_stickers -> stickers (sticker_id));
joinable!(user_populate_stickers -> users (user_id));
joinable!(user_post_notifications -> users (user_id));
joinable!(user_private -> users (user_id));
joinable!(user_profile -> users (user_id));
joinable!(user_profile_notifications -> users (user_id));
joinable!(user_video_notifications -> users (user_id));

allow_tables_to_appear_in_same_query!(
    color_settings,
    custom_links,
    featured_uc,
    ip_user,
    list_uc,
    news_uc,
    notify_uc,
    phone_codes,
    smile_categories,
    smiles,
    sticker_categories,
    stickers,
    user_anketa,
    user_blocks,
    user_brother_sister,
    user_children_one,
    user_colleagues_one,
    user_dad_one,
    user_delete_anketa,
    user_doc_list_position,
    user_good_list_position,
    user_good_notifications,
    user_grandsons_one,
    user_location,
    user_love_status,
    user_mom_one,
    user_music_list_position,
    user_music_notifications,
    user_partner_one,
    user_photo_list_position,
    user_photo_notifications,
    user_populate_smiles,
    user_populate_stickers,
    user_post_list_position,
    user_post_notifications,
    user_private,
    user_profile,
    user_profile_notifications,
    user_survey_list_position,
    user_video_list_position,
    user_video_notifications,
    users,
);
