table! {
    custom_link (id) {
        id -> Int4,
        link -> Varchar,
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
    phone_codes (id) {
        id -> Int4,
        phone -> Varchar,
        code -> Int4,
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
    user_brother_sister_one (id) {
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
        answer -> Nullable<Varchar>,
        other -> Nullable<Varchar>,
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
    user_partner_one (id) {
        id -> Int4,
        user_id -> Int4,
        partner_id -> Int4,
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
        activity -> Nullable<Text>,
        interests -> Nullable<Text>,
        favorite_music -> Nullable<Text>,
        favorite_films -> Nullable<Text>,
        favorite_books -> Nullable<Text>,
        favorite_game -> Nullable<Text>,
        about -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        _type -> Varchar,
        gender -> Varchar,
        device -> Varchar,
        language -> Varchar,
        perm -> Varchar,
        level -> Nullable<Int4>,
        password -> Varchar,
        have_link -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        b_avatar -> Nullable<Text>,
        s_avatar -> Nullable<Text>,
        email -> Varchar,
    }
}

joinable!(ip_user -> users (user_id));
joinable!(user_anketa -> users (user_id));
joinable!(user_delete_anketa -> users (user_id));
joinable!(user_location -> users (user_id));
joinable!(user_love_status -> users (user_id));
joinable!(user_profile -> users (user_id));

allow_tables_to_appear_in_same_query!(
    custom_link,
    ip_user,
    phone_codes,
    user_anketa,
    user_brother_sister_one,
    user_children_one,
    user_colleagues_one,
    user_dad_one,
    user_delete_anketa,
    user_grandsons_one,
    user_location,
    user_love_status,
    user_mom_one,
    user_partner_one,
    user_profile,
    users,
);
