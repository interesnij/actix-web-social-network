use actix_web::HttpRequest;
use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::models::User;

const CONN: diesel::PgConnection = establish_connection();

pub fn add_music_list(pk: i32) -> String {
    use crate::schema::music_lists::dsl::music_lists;
    use crate::models::MusicList;

    let mut name = "".to_string();
    let mut link = "".to_string();

    let list = music_lists
        .filter(schema::music_lists::id.eq(pk))
        .filter(schema::music_lists::types.lt(10))
        .load::<MusicList>(&CONN)
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
    &list.user_id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_music_list btn_default pointer'>".to_string() +
    &image + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_music_list pointer'>".to_string() +
    &list.name + &"</h6><p>Плейлист <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Треков: ".to_string() +
    &list.count.to_string() + &"</p></div><span class='playlist_share'></span></div></div></div>".to_string()
}

pub fn post_elements(attach: String, user_id: i32) -> String {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();

    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    let user: User = users
        .filter(schema::users::id.eq(user_id))
        .filter(schema::users::types.lt(10))
        .load::<User>(&CONN)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    for item in v.iter() {
        let pk: i32 = item[3..].parse().unwrap();
        let first_char = item.chars().nth(0).unwrap();
        let code = &item[..3];

        if first_char == 'l' {
            if code == "lmu".to_string() {
                block = block + &add_music_list(pk);
            }
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
