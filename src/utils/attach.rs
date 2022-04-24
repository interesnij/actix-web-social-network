use actix_web::HttpRequest;
use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::models::User;


pub fn add_post_list(pk: i32) -> String {
    use crate::schema::post_lists::dsl::post_lists;
    use crate::models::PostList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut image = "".to_string();

    let list = post_lists
        .filter(schema::post_lists::id.eq(pk))
        .filter(schema::post_lists::types.lt(10))
        .load::<PostList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    if list.community_id.is_some() {
        let community = list.get_community();
        name = community.name.clone();
        link = community.get_link().clone();
        image = community.get_bb_avatar()
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        image = creator.get_bb_avatar()
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() +
    &list.get_str_id() +
    &"' &playlist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_post_list btn_default pointer'>".to_string() +
    &image + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_post_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список записей: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Записей: ".to_string() +
    &list.count.to_string() + &"</p></div><span class='playlist_share'></span></div></div></div>".to_string();
}

pub fn add_music_list(pk: i32) -> String {
    use crate::schema::music_lists::dsl::music_lists;
    use crate::models::MusicList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();

    let list = music_lists
        .filter(schema::music_lists::id.eq(pk))
        .filter(schema::music_lists::types.lt(10))
        .load::<MusicList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    if list.community_id.is_some() {
        let community = list.get_community();
        name = community.name.clone();
        link = community.get_link().clone();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
    }

    let mut image = "".to_string();
    if list.image.is_some() {
        image = "<img src='".to_owned() + &list.image.as_ref().unwrap() + &"' style='width:120px;height:120px;' alt='image'>".to_string();
    }
    else {
        image = "<svg fill='currentColor' class='svg_default border' style='width:120px;height:120px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M15 6H3v2h12V6zm0 4H3v2h12v-2zM3 16h8v-2H3v2zM17 6v8.18c-.31-.11-.65-.18-1-.18-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3V8h3V6h-5z'/></svg>".to_string();
    }
    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() +
    &list.get_str_id() +
    &"' &playlist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_music_list btn_default pointer'>".to_string() +
    &image + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_music_list pointer'>".to_string() +
    &list.name + &"</h6><p>Плейлист: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Треков: ".to_string() +
    &list.count.to_string() + &"</p></div><span class='playlist_share'></span></div></div></div>".to_string();
}

pub fn add_doc_list(pk: i32) -> String {
    use crate::schema::doc_lists::dsl::doc_lists;
    use crate::models::DocList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();

    let list = doc_lists
        .filter(schema::doc_lists::id.eq(pk))
        .filter(schema::doc_lists::types.lt(10))
        .load::<DocList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    if list.community_id.is_some() {
        let community = list.get_community();
        name = community.name.clone();
        link = community.get_link().clone();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() +
    &list.get_str_id() +
    &"' &playlist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_doc_list btn_default pointer'><svg fill='currentColor' class='svg_default border' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_doc_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список документов: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Треков: ".to_string() +
    &list.count.to_string() + &"</p></div><span class='playlist_share'></span></div></div></div>".to_string();
}

pub fn post_elements(attach: String, user_id: i32) -> String {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();

    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    let user: User = users
        .filter(schema::users::id.eq(user_id))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    for item in v.iter() {
        let pk: i32 = item[3..].parse().unwrap();
        let first_char = item.chars().nth(0).unwrap();

        if first_char == 'l' {
            let code = &item[..3];
            let html = match code {
                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "ldo" => add_post_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
