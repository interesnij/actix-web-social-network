use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::utils::User;

pub fn add_post_list(pk: i32) -> String {
    use crate::schema::post_lists::dsl::post_lists;
    use crate::models::PostList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut image = "".to_string();
    let mut owner = "".to_string();

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
        image = community.get_bb_avatar();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        image = creator.get_bb_avatar();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
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
    let mut owner = "".to_string();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        owner = creator.id.to_string();
    }

    let mut image = "".to_string();
    if list.image.is_some() {
        image = "<img src='".to_owned() + &list.image.as_ref().unwrap() + &"' style='width:120px;height:120px;' alt='image'>".to_string();
    }
    else {
        image = "<svg fill='currentColor' class='svg_default border' style='width:120px;height:120px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M15 6H3v2h12V6zm0 4H3v2h12v-2zM3 16h8v-2H3v2zM17 6v8.18c-.31-.11-.65-.18-1-.18-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3V8h3V6h-5z'/></svg>".to_string();
    }
    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
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
    let mut owner = "".to_string();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
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
pub fn add_video_list(pk: i32) -> String {
    use crate::schema::video_lists::dsl::video_lists;
    use crate::models::VideoList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut owner = "".to_string();

    let list = video_lists
        .filter(schema::video_lists::id.eq(pk))
        .filter(schema::video_lists::types.lt(10))
        .load::<VideoList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    if list.community_id.is_some() {
        let community = list.get_community();
        name = community.name.clone();
        link = community.get_link().clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
    &"' &playlist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_video_list btn_default pointer'><svg fill='currentColor' class='svg_default border' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_video_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список видеозаписей: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Видеозаписей: ".to_string() +
    &list.count.to_string() + &"</p></div><span class='playlist_share'></span></div></div></div>".to_string();
}
pub fn add_photo_list(pk: i32) -> String {
    use crate::schema::photo_lists::dsl::photo_lists;
    use crate::models::PhotoList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut owner = "".to_string();

    let list = photo_lists
        .filter(schema::photo_lists::id.eq(pk))
        .filter(schema::photo_lists::types.lt(10))
        .load::<PhotoList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    if list.community_id.is_some() {
        let community = list.get_community();
        name = community.name.clone();
        link = community.get_link().clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.get_link().clone();
        owner = creator.id.to_string();
    }

    return "<div class='custom_color mb-1 text-center has-background-img
    position-relative box-shadow' owner-pk='".to_string() + &owner +
    &"' &photolist-pk='".to_string() + &list.id.to_string() +
    &"' style='width: 100%;flex-basis: 100%;'>
    <figure class='background-img'><img src='".to_string() +
    &list.cover_photo.as_ref().unwrap() + &"' </figure><div class='container'>
    <i class='figure avatar120 mr-0 rounded-circle bg-none'></i><br>
    <h4 class='load_photo_list pointer'><a>".to_string() + &list.name +
    &"</a></h4><p class='lead'><a class='ajax underline' href='".to_string() +
    &link + &"'>".to_string() + &name + &"</a></p>
    <hr class='my-3'><a class='load_photo_list pointer'>".to_string() +
    &list.count_items_ru() + &"</a><div class='row'></div></div></div>".to_string();
}

pub fn add_good_list(pk: i32) -> String {
    use crate::schema::good_lists::dsl::good_lists;
    use crate::models::GoodList;
    let _connection = establish_connection();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut image = "".to_string();

    let list = good_lists
        .filter(schema::good_lists::id.eq(pk))
        .filter(schema::good_lists::types.lt(10))
        .load::<GoodList>(&_connection)
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

    return "<div goodlist-pk='".to_string() + &list.id.to_string() +
    &"' style='padding: 7px;width: 100%;flex-basis: 100%'><div class='media mb-2'>
    <div class='media-body'><h6 class='content-color-primary mb-0 load_good_list pointer'>
    <a>".to_string() + &list.name.to_string() + &"</a></h6></div><span class='small'></span></div>
    <div class='centered no-gutters'><figure class='mx-auto mb-3' style='width:120px'>
    <img class='load_good_list pointer image_fit_small' src='".to_string()
     + &image + &"' style='border-radius:50%' /></figure></div>
     <h5 class='mb-2 header-color-primary text-center'><a href='" + &link +
     &"' class='ajax underline'>".to_string() + &name + &"</a></h5>
     <h6 class='card-subtitle header-color-secondary text-center'>".to_string() +
      &list.count_items_ru() + &"</h6></div>".to_string();
}
pub fn add_photo(pk: i32, case: String) -> String {
    use crate::schema::photos::dsl::photos;
    use crate::models::Photo;
    let _connection = establish_connection();

    let photo = photos
        .filter(schema::photos::id.eq(pk))
        .filter(schema::photos::types.lt("a"))
        .load::<Photo>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='photo'><div class='progressive replace ".to_string() + &case + &" pointer' data-href='".to_string() + &photo.file + &"' photo-pk='".to_string() + &photo.file + &"'><img class='preview image_fit' width='20' height='15' loading='lazy' src='".to_string() + &photo.preview + &"' alt='img'></div></div>".to_string();
}
pub fn add_video(pk: i32, case: String) -> String {
    use crate::schema::videos::dsl::videos;
    use crate::models::Video;
    let _connection = establish_connection();

    let video = videos
        .filter(schema::videos::id.eq(pk))
        .filter(schema::videos::types.lt("a"))
        .load::<Video>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='video'><img class='image_fit' src='" + video.image + "' alt='img'><div class='video_icon_play_v2 " + case + "' video-pk='" + pk + "' video-counter=''></div></div>".to_string();
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
        let code = &item[..3];

        let html = match code {
            "pho" => add_photo(pk, "post_photo".to_string()),
            "vid" => add_video(pk, "post_video".to_string()),

            "lmu" => add_music_list(pk),
            "ldo" => add_doc_list(pk),
            "ldo" => add_post_list(pk),
            "lvi" => add_video_list(pk),
            "lph" => add_photo_list(pk),
            "lgo" => add_good_list(pk),
            _ => "".to_string(),
        };
        block = block + &html;
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
