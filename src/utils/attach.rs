//#[macro_use(concat_string)]
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
        link = community.link.clone();
        image = community.get_bb_avatar();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
        image = creator.get_bb_avatar();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
    &"' &postlist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_post_list btn_default pointer'>".to_string() +
    &image + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_post_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список записей: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Записей: ".to_string() +
    &list.count.to_string() + &"</p></div></div></div></div>".to_string();
}

pub fn add_edited_post_list(pk: i32) -> String {
    use crate::schema::post_lists::dsl::post_lists;
    use crate::models::PostList;
    let _connection = establish_connection();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' postlist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='lpo".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_postlist pointer'><svg width='50' height='50' viewBox='0 0 24 24'
    fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round'
    stroke-linejoin='round'><polygon points='5 3 19 12 5 21 5 3'></polygon></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_postlist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    post_attach_list_remove underline'>Открепить</small></div></div>".to_string();
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
        link = community.link.clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
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
pub fn add_edited_music_list(pk: i32) -> String {
    use crate::schema::music_lists::dsl::music_lists;
    use crate::models::MusicList;
    let _connection = establish_connection();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' playlist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='lmu".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_playlist pointer'><svg width='50' height='50' viewBox='0 0 24 24'
    fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round'
    stroke-linejoin='round'><polygon points='5 3 19 12 5 21 5 3'></polygon></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_playlist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    music_attach_list_remove underline'>Открепить</small></div></div>".to_string();
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
        link = community.link.clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
    &"' &doclist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_doc_list btn_default pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_doc_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список документов: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Документов: ".to_string() +
    &list.count.to_string() + &"</p></div></div></div></div>".to_string();
}
pub fn add_edited_doc_list(pk: i32) -> String {
    use crate::schema::doc_lists::dsl::doc_lists;
    use crate::models::DocList;
    let _connection = establish_connection();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' doclist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='ldo".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_doclist pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13'/></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_doclist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    doc_attach_list_remove underline'>Открепить</small></div></div>".to_string();
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
        link = community.link.clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
        owner = creator.id.to_string();
    }

    return "<div style='flex-basis: 100%;' class='card'>
    <div class='card-body' owner-pk='".to_string() + &owner +
    &"' &videolist-pk='".to_string() +
    &list.id.to_string() +
    &"' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
    <figure><a class='load_video_list btn_default pointer'><svg fill='currentColor' class='svg_default border' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
    <h6 class='my-0 mt-1 load_video_list pointer'>".to_string() +
    &list.name + &"</h6><p>Список видеозаписей: <a style='vertical-align: baseline;'
    class='ajax underline' href='".to_string() +
    &link + &"'>" + &name + &"</a><br>Видеозаписей: ".to_string() +
    &list.count.to_string() + &"</p></div></div></div></div>".to_string();
}
pub fn add_edited_video_list(pk: i32) -> String {
    use crate::schema::video_lists::dsl::video_lists;
    use crate::models::VideoList;
    let _connection = establish_connection();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' videolist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='lvi".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_videolist pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_videolist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    video_attach_list_remove underline'>Открепить</small></div></div>".to_string();
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
        link = community.link.clone();
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
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
pub fn add_edited_photo_list(pk: i32) -> String {
    use crate::schema::photo_lists::dsl::photo_lists;
    use crate::models::PhotoList;
    let _connection = establish_connection();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' photolist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='lph".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_photolist pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_photolist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    photo_attach_list_remove underline'>Открепить</small></div></div>".to_string();
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
        link = community.link.clone();
        image = community.get_bb_avatar()
    }
    else {
        let creator = list.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
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
pub fn add_edited_good_list(pk: i32) -> String {
    use crate::schema::good_lists::dsl::good_lists;
    use crate::models::GoodList;
    let _connection = establish_connection();

    let mut owner = "".to_string();

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
        owner = community.id.to_string();
    }
    else {
        let creator = list.get_creator();
        owner = creator.id.to_string();
    }

    return "<div class='folder' owner-pk='".to_string() + &owner.to_string() +
    &"' goodlist-pk='".to_string() + &list.id.to_string() +
    &"' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items[]'
    value='lgo".to_string() + &list.id.to_string() +
    &"'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
    <a class='nowrap'><div class='d-flex align-items-center justify-content-center
    w-100 load_goodlist pointer'><svg fill='currentColor' class='svg_default' viewBox='0 0 24 24'><g><rect fill='none' /><path d='M18,6h-2c0-2.21-1.79-4-4-4S8,3.79,8,6H6C4.9,6,4,6.9,4,8v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8C20,6.9,19.1,6,18,6z M12,4c1.1,0,2,0.9,2,2h-4C10,4.9,10.9,4,12,4z M18,20H6V8h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8h4v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8 h2V20z'/></g></svg>
    </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
    style='display: flex;'><p class='card-text file-name mb-0 load_goodlist pointer'>
    <a class='nowrap'>".to_string() + &list.name + &" (".to_string() +
    &list.count_items().to_string() + &")</a></p></div><small class='file-accessed pointer
    good_attach_list_remove underline'>Открепить</small></div></div>".to_string();
}

pub fn add_photo(pk: i32, case: String) -> String {
    use crate::schema::photos::dsl::photos;
    use crate::models::Photo;
    let _connection = establish_connection();

    let photo = photos
        .filter(schema::photos::id.eq(pk))
        .filter(schema::photos::types.eq("a"))
        .load::<Photo>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='photo'><div class='progressive replace ".to_string() + &case + &" pointer' data-href='".to_string() + &photo.file + &"' photo-pk='".to_string() + &photo.file + &"'><img class='preview image_fit' width='20' height='15' loading='lazy' src='".to_string() + &photo.preview + &"' alt='img'></div></div>".to_string();
}
pub fn add_edited_photo(pk: i32, case: String) -> String {
    use crate::schema::photos::dsl::photos;
    use crate::models::Photo;
    let _connection = establish_connection();

    let photo = photos
        .filter(schema::photos::id.eq(pk))
        .filter(schema::photos::types.eq("a"))
        .load::<Photo>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='photo'><span class='photo_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items[]' value='pho".to_string() + &photo.id.to_string() + &"'></span><div class='progressive replace ".to_string() + &case + &" pointer' data-href='".to_string() + &photo.file + &"' photo-pk='".to_string() + &photo.file + &"'><img class='preview image_fit' width='20' height='15' loading='lazy' src='".to_string() + &photo.preview + &"' alt='img'></div></div>".to_string();
}

pub fn add_video(pk: i32, case: String) -> String {
    use crate::schema::videos::dsl::videos;
    use crate::models::Video;
    let _connection = establish_connection();

    let video = videos
        .filter(schema::videos::id.eq(pk))
        .filter(schema::videos::types.eq("a"))
        .load::<Video>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='video'><img class='image_fit' src='".to_owned() + &video.get_image() + &"' alt='img'><div class='video_icon_play_v2 ".to_string() + &case + &"' video-pk='".to_string() + &pk.to_string() + &"' video-counter=''></div></div>".to_string();
}
pub fn add_edited_video(pk: i32, case: String) -> String {
    use crate::schema::videos::dsl::videos;
    use crate::models::Video;
    let _connection = establish_connection();

    let video = videos
        .filter(schema::videos::id.eq(pk))
        .filter(schema::videos::types.eq("a"))
        .load::<Video>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='video'><span class='video_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items[]' value='vid".to_string() + &video.id.to_string() + &"'></span><img class='image_fit' src='".to_owned() + &video.get_image() + &"' alt='img'><div class='video_icon_play_v2 ".to_string() + &case + &"' video-pk='" + &pk.to_string() + &"' video-counter=''></div></div>".to_string();
}

pub fn add_good(pk: i32) -> String {
    use crate::schema::goods::dsl::goods;
    use crate::models::Good;
    let _connection = establish_connection();

    let good = goods
        .filter(schema::goods::id.eq(pk))
        .filter(schema::goods::types.eq("a"))
        .load::<Good>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div class='card has-background-img good_detail mb-3 pointer' good-pk='".to_string() + &good.id.to_string() + &"' style='flex-basis: 100%;'><figure class='background-img shadow-dark'>".to_string() + &good.get_image() + &"</figure><div class='card-header'><div class='media'><div class='media-body'><h4 class='text-white mb-0'>".to_string() + &good.title + &"</h4></div></div></div><div class='card-body spantshirt'></div><div class='card-footer'><p class='small mb-1 text-success'>".to_string() + &good.get_price() + &"</p></div></div>".to_string();
}
pub fn add_edited_good(pk: i32) -> String {
    use crate::schema::goods::dsl::goods;
    use crate::models::Good;
    let _connection = establish_connection();

    let good = goods
        .filter(schema::goods::id.eq(pk))
        .filter(schema::goods::types.eq("a"))
        .load::<Good>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

        return "<div class='card has-background-img good_detail mb-3 pointer' good-pk='".to_string() + &good.id.to_string() + &"' style='flex-basis: 100%;'>
        <span class='good_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items[]' value='goo".to_string() + &good.id.to_string() + &"'></span>
        <figure class='background-img shadow-dark'>".to_string() + &good.get_image() + &"</figure><div class='card-header'><div class='media'><div class='media-body'><h4 class='text-white mb-0'>".to_string() + &good.title + &"</h4></div></div></div><div class='card-body spantshirt'></div><div class='card-footer'><p class='small mb-1 text-success'>".to_string() + &good.get_price() + &"</p></div></div>".to_string();
}

pub fn add_music(pk: i32, is_staff: bool, user_id: i32) -> String {
    use crate::schema::musics::dsl::musics;
    use crate::models::Music;
    let _connection = establish_connection();

    let music = musics
        .filter(schema::musics::id.eq(pk))
        .filter(schema::musics::types.eq("a"))
        .load::<Music>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let mut drops = "<span class='dropdown-item create_repost'>Добавить</span><span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();
    //if music.is_user_can_edit_delete_item(user) {
    //    drops = drops + &"<span class='dropdown-item track_edit'>Изменить</span><span class='dropdown-item track_remove'>Удалить</span>".to_string();
    //}
    if is_staff == true && user_id != pk {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }

    return "<div class='music' data-path='".to_string() + &music.file +
    &"' style='flex-basis: auto;width:100%;position: relative;'><div class='media'
    music-counter=''>".to_string() + &"'>".to_string() +
    &music.get_s_image() + &"<div class='media-body' style='display: flex;'><h6 class='music_list_post music_title'><a>".to_string() +
    &music.title + &"</a></h6><span class='span_btn' data-pk='".to_string() + &music.id.to_string() +
    &"'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
    fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
    </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='mus" + &music.id.to_string() + &"'>".to_string() +
    &drops + &"</div></span</span></div></div></div>".to_string();
}
pub fn add_edited_music(pk: i32) -> String {
    use crate::schema::musics::dsl::musics;
    use crate::models::Music;
    let _connection = establish_connection();

    let music = musics
        .filter(schema::musics::id.eq(pk))
        .filter(schema::musics::types.eq("a"))
        .load::<Music>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div style='display: flex; padding: 3px;'><span class='music_preview_delete'
    tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
    <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
    </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
    <input type='hidden' name='attach_items[]' value='mus".to_string() + &music.id.to_string() +
    &"'></span><span><svg width='30' height='30' viewBox='0 0 24 24' fill='none'
    stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'
    ><polygon points='5 3 19 12 5 21 5 3'></polygon></svg></span>
    <span style='margin-left: 10px; margin-right: 40px; overflow: hidden;'>
    <h6 class='music_list_item pointer music_title' style='padding-top: 4px;'>
    <a>" + &music.title + &"</a></h6></span></div>".to_string();
}

pub fn add_anon_music(pk: i32) -> String {
    use crate::schema::musics::dsl::musics;
    use crate::models::Music;
    let _connection = establish_connection();

    let music = musics
        .filter(schema::musics::id.eq(pk))
        .filter(schema::musics::types.eq("a"))
        .load::<Music>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let drops = "<span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();

    return "<div class='music' data-path='".to_string() + &music.file +
    &"' style='flex-basis: auto;width:100%;position: relative;'><div class='media'
    music-counter=''>".to_string() + &"'>".to_string() +
    &music.get_s_image() + &"<div class='media-body' style='display: flex;'><h6 class='music_list_post music_title'><a>".to_string() +
    &music.title + &"</a></h6><span class='span_btn' data-pk='".to_string() + &music.id.to_string() +
    &"'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
    fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
    </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='mus" + &music.id.to_string() + &"'>".to_string() +
    &drops + &"</div></span</span></div></div></div>".to_string();
}

pub fn add_anon_doc(pk: i32) -> String {
    use crate::schema::docs::dsl::docs;
    use crate::models::Doc;
    let _connection = establish_connection();

    let doc = docs
        .filter(schema::docs::id.eq(pk))
        .filter(schema::docs::types.eq("a"))
        .load::<Doc>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let drops = "<span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();

    return "<div class='doc' data-path='".to_string() + &doc.file +
    &"' style='flex-basis: auto;width:100%;position: relative;'><div class='media'>
    <svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
    <div class='media-body' style='display: flex;'><h6 class='doc_title'><a>".to_string() +
    &doc.title + &"</a></h6><span class='span_btn' data-pk='".to_string() + &doc.id.to_string() +
    &"'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
    fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
    </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='doc" + &doc.id.to_string() + &"'>".to_string() +
    &drops + &"</div></span</span></div></div></div>".to_string();
}
pub fn add_doc(pk: i32, is_staff: bool, user_id: i32) -> String {
    use crate::schema::docs::dsl::docs;
    use crate::models::Doc;
    let _connection = establish_connection();

    let doc = docs
        .filter(schema::docs::id.eq(pk))
        .filter(schema::docs::types.eq("a"))
        .load::<Doc>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let mut drops = "<span class='dropdown-item create_repost'>Добавить</span><span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();
    //if music.is_user_can_edit_delete_item(user) {
    //    drops = drops + &"<span class='dropdown-item track_edit'>Изменить</span><span class='dropdown-item track_remove'>Удалить</span>".to_string();
    //}
    if is_staff == true && user_id != pk {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }

    return "<div class='doc' data-path='".to_string() + &doc.file +
    &"' style='flex-basis: auto;width:100%;position: relative;'><div class='media'>
    <svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
    <div class='media-body' style='display: flex;'><h6 class='doc_title'><a>".to_string() +
    &doc.title + &"</a></h6><span class='span_btn' data-pk='".to_string() + &doc.id.to_string() +
    &"'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
    fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
    </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='doc" + &doc.id.to_string() + &"'>".to_string() +
    &drops + &"</div></span</span></div></div></div>".to_string();
}
pub fn add_edited_doc(pk: i32) -> String {
    use crate::schema::docs::dsl::docs;
    use crate::models::Doc;
    let _connection = establish_connection();

    let doc = docs
        .filter(schema::docs::id.eq(pk))
        .filter(schema::docs::types.eq("a"))
        .load::<Doc>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

        return "<div style='display: flex; padding: 3px;'><span class='doc_preview_delete'
        tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
        <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
        </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
        <input type='hidden' name='attach_items[]' value='mus".to_string() + &doc.id.to_string() +
        &"'></span><span><svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
        </span><span style='margin-left: 10px; margin-right: 40px; overflow: hidden;'>
        <h6 class='music_list_item pointer music_title' style='padding-top: 4px;'>
        <a href='".to_string() + &doc.file + &"' style='white-space: nowrap;' target='_blank' rel='nofollow'>" + &doc.title + &"</a></h6></span></div>".to_string();
}

pub fn add_user(pk: i32) -> String {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();

    let user = users
        .filter(schema::users::id.eq(pk))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div style='flex-basis: 100%;' class='card'><div class='card-body'
    style='padding: 5px'><div style='display:flex'><figure><a class='ajax'
    href='".to_string() + &user.link.to_string() + &"' >".to_string() +
    &user.get_bb_avatar() + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <a href='".to_string() + &user.link.to_string() +
    &"' class='my-0 mt-1 ajax'>".to_string() + &user.get_full_name() +
    &"</a><p>".to_string() + &user.get_online_status() + &"<br>Друзей: ".to_string() +
    &user.get_profile().friends.to_string() + &"</p></div></div></div></div>".to_string();
}
pub fn add_edited_user(pk: i32) -> String {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();

    let user = users
        .filter(schema::users::id.eq(pk))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div style='flex-basis: 100%;' class='card'><div class='card-body'
    style='padding: 5px'>
    <span class='doc_preview_delete'
    tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
    <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
    </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
    <input type='hidden' name='attach_items[]' value='use".to_string() + &user.id.to_string() +
    &"'></span>
    <div style='display:flex'><figure><a class='ajax'
    href='".to_string() + &user.link.to_string() + &"' >".to_string() +
    &user.get_bb_avatar() + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <a href='".to_string() + &user.link.to_string() +
    &"' class='my-0 mt-1 ajax'>".to_string() + &user.get_full_name() +
    &"</a><p>".to_string() + &user.get_online_status() + &"<br>Друзей: ".to_string() +
    &user.get_profile().friends.to_string() + &"</p></div></div></div></div>".to_string();
}

pub fn add_community(pk: i32) -> String {
    use crate::schema::communitys::dsl::communitys;
    use crate::models::Community;
    let _connection = establish_connection();

    let community = communitys
        .filter(schema::communitys::id.eq(pk))
        .filter(schema::communitys::types.lt(10))
        .load::<Community>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div style='flex-basis: 100%;' class='card'><div class='card-body'
    style='padding: 5px'><div style='display:flex'><figure><a class='ajax'
    href='".to_string() + &community.link.to_string() + &"' >".to_string() +
    &community.get_bb_avatar() + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <a href='".to_string() + &community.link.to_string() +
    &"' class='my-0 mt-1 ajax'>".to_string() + &community.name +
    &"</a><p>".to_string() + &community.description.as_ref().unwrap() + &"<br>Подписчиков: ".to_string() +
    &community.get_info_model().members.to_string() + &"</p></div></div></div></div>".to_string();
}
pub fn add_edited_community(pk: i32) -> String {
    use crate::schema::communitys::dsl::communitys;
    use crate::models::Community;
    let _connection = establish_connection();

    let community = communitys
        .filter(schema::communitys::id.eq(pk))
        .filter(schema::communitys::types.lt(10))
        .load::<Community>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    return "<div style='flex-basis: 100%;' class='card'><div class='card-body'
    style='padding: 5px'>
    <span class='doc_preview_delete'
    tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
    <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
    </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
    <input type='hidden' name='attach_items[]' value='com".to_string() + &community.id.to_string() +
    &"'></span>
    <div style='display:flex'><figure><a class='ajax'
    href='".to_string() + &community.link.to_string() + &"' >".to_string() +
    &community.get_bb_avatar() + &"</a></figure><div class='media-body' style='margin-left: 10px;'>
    <a href='".to_string() + &community.link.to_string() +
    &"' class='my-0 mt-1 ajax'>".to_string() + &community.name +
    &"</a><p>".to_string() + &community.description.as_ref().unwrap() + &"<br>Подписчиков: ".to_string() +
    &community.get_info_model().members.to_string() + &"</p></div></div></div></div>".to_string();
}

pub fn add_anon_survey(pk: i32) -> String {
    use crate::schema::surveys::dsl::surveys;
    use crate::models::Survey;
    let _connection = establish_connection();

    let survey = surveys
        .filter(schema::surveys::id.eq(pk))
        .filter(schema::surveys::types.eq("a"))
        .load::<Survey>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut answers = "".to_string();
    let mut info = "".to_string();
    let drops = "<span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();

    if survey.community_id.is_some() {
        let community = survey.get_community();
        name = community.name.clone();
        link = community.link.clone();
    }
    else {
        let creator = survey.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
    }

    if survey.is_have_votes() {
        if survey.is_anonymous == true {
            info = "Это анонимный опрос.".to_string();
        }
        else {
            info = "<a class='i_link survey_info pointer position-relative'>".to_string() + &survey.get_users_ru() + &"</a>".to_string() + &survey.get_6_users().to_string();
        }
    }
    else {
        info = "Пока никто не голосовал.".to_string();
    }

    for answer in survey.get_answers().iter() {
        answers = answers + &"<div data-pk='" + &answer.id.to_string() +
        &"' class='lite_color answer_style pointer survey_vote'>
        <div class='progress2' style='width:'" + &answer.get_procent().to_string() +
        &"%;'></div><span class='progress_span_r'>" + &answer.content +
        &" <span class='count text-muted small'>" + &answer.vote.to_string() +
        &"</span></span><span class='progress_span_l' style='margin-left: auto;'>
        <span class='vote_svg'></span><span class='procent'>".to_string() +
        &answer.get_procent().to_string() + &"%</span></span></div>".to_string();
    }

    return "<div data-pk='".to_string() + &survey.id.to_string() +
    &"' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
    <figure class='background-img'><img src='".to_string() + &survey.get_image() +
    &"alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
    </a><div class='dropdown-menu dropdown-menu-right' data-type='sur".to_string() + &pk.to_string() + &"' style='top:30px;right:-10px;'>".to_string() + &drops +
    &"</div></div><form><div class='container answers_container'> <br><h4 class='m-0'>".to_string() + &survey.title +
    &"</h4><p class='position-relative'><a href=".to_string() + &link + &"' class='underline ajax'>".to_string() +
    &name + &"</a></p>".to_string() + &survey.get_time_description() + &"<br>".to_string() +
    &answers + &info + &"</div>".to_string() + &"</form></div>".to_string();
}
pub fn add_survey(pk: i32, is_staff: bool, user_id: i32) -> String {
    use crate::schema::surveys::dsl::surveys;
    use crate::models::Survey;
    let _connection = establish_connection();

    let survey = surveys
        .filter(schema::surveys::id.eq(pk))
        .filter(schema::surveys::types.eq("a"))
        .load::<Survey>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut multiple_class = "".to_string();
    let mut answers = "".to_string();
    let mut vote_svg = "".to_string();
    let mut info = "".to_string();

    if survey.community_id.is_some() {
        let community = survey.get_community();
        name = community.name.clone();
        link = community.link.clone();
    }
    else {
        let creator = survey.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
    }

    if survey.is_multiple == false {
        multiple_class = "no_multiple".to_string();
    }
    if survey.is_have_votes() {
        if survey.is_anonymous == true {
            info = "Это анонимный опрос.".to_string();
        }
        else {
            info = "<a class='i_link survey_info pointer position-relative'>".to_string() + &survey.get_users_ru() + &"</a>".to_string() + &survey.get_6_users().to_string();
        }
    }
    else {
        info = "Пока никто не голосовал.".to_string();
    }

    let mut drops = "<span class='dropdown-item create_repost'>Добавить</span><span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();
    if is_staff == true && user_id != pk {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }
    for answer in survey.get_answers().iter() {
        if answer.is_user_voted(user_id) {
            vote_svg = "<svg fill='currentColor' style='width:15px;height:15px;' class='svg_default' viewBox='0 0 24 24'><path fill='none' d='M0 0h24v24H0z'></path><path d='M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z'></path></svg>".to_string()
        }
        answers = answers + &"<div data-pk='" + &answer.id.to_string() +
        &"' class='lite_color answer_style pointer survey_vote'>
        <div class='progress2' style='width:'" + &answer.get_procent().to_string() +
        &"%;'></div><span class='progress_span_r'>" + &answer.content +
        &" <span class='count text-muted small'>" + &answer.vote.to_string() +
        &"</span></span><span class='progress_span_l' style='margin-left: auto;'>
        <span class='vote_svg'>".to_string() + &vote_svg + &"</span><span class='procent'>".to_string() +
        &answer.get_procent().to_string() + &"%</span></span></div>".to_string();
    }

    return "<div data-pk='".to_string() + &survey.id.to_string() +
    &"' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
    <figure class='background-img'><img src='".to_string() + &survey.get_image() +
    &"alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
    </a><div class='dropdown-menu dropdown-menu-right' data-type='sur".to_string() + &pk.to_string() + &"' style='top:30px;right:-10px;'>
    <span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string() + &drops +
    &"</div></div><form><div class='container answers_container ".to_string() + &multiple_class +
    &"'> <br><h4 class='m-0'>".to_string() + &survey.title +
    &"</h4><p class='position-relative'><a href=".to_string() + &link + &"' class='underline ajax'>".to_string() +
    &name + &"</a></p>".to_string() + &survey.get_time_description() + &"<br>".to_string() +
    &answers + &info + &"</div><div class='card-footer position-relative'>
    <button type='button' class='btn hidden btn-sm float-left border votes_remove'>Отмена</button>
    <button id='add_vote_survey_btn' type='button' class='btn hidden btn-sm btn-success float-right'>
    Проголосовать</button></div>".to_string() + &"</form></div>".to_string();
}
pub fn add_edited_survey(pk: i32) -> String {
    use crate::schema::surveys::dsl::surveys;
    use crate::models::Survey;
    let _connection = establish_connection();

    let survey = surveys
        .filter(schema::surveys::id.eq(pk))
        .filter(schema::surveys::types.eq("a"))
        .load::<Survey>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();

    let mut name = "".to_string();
    let mut link = "".to_string();
    let mut answers = "".to_string();
    let mut info = "".to_string();
    let drops = "<span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();

    if survey.community_id.is_some() {
        let community = survey.get_community();
        name = community.name.clone();
        link = community.link.clone();
    }
    else {
        let creator = survey.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
    }

    if survey.is_have_votes() {
        if survey.is_anonymous == true {
            info = "Это анонимный опрос.".to_string();
        }
        else {
            info = "<a class='i_link survey_info pointer position-relative'>".to_string() + &survey.get_users_ru() + &"</a>".to_string() + &survey.get_6_users().to_string();
        }
    }
    else {
        info = "Пока никто не голосовал.".to_string();
    }

    for answer in survey.get_answers().iter() {
        answers = answers + &"<div data-pk='" + &answer.id.to_string() +
        &"' class='lite_color answer_style pointer survey_vote'>
        <div class='progress2' style='width:'" + &answer.get_procent().to_string() +
        &"%;'></div><span class='progress_span_r'>" + &answer.content +
        &" <span class='count text-muted small'>" + &answer.vote.to_string() +
        &"</span></span><span class='progress_span_l' style='margin-left: auto;'>
        <span class='vote_svg'></span><span class='procent'>".to_string() +
        &answer.get_procent().to_string() + &"%</span></span></div>".to_string();
    }

    return "<div data-pk='".to_string() + &survey.id.to_string() +
    &"' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
    <span class='survey_preview_delete'
    tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
    <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
    </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
    <input type='hidden' name='attach_items[]' value='sur".to_string() + &survey.id.to_string() +
    &"'></span>
    <figure class='background-img'><img src='".to_string() + &survey.get_image() +
    &"alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
    <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
    </a><div class='dropdown-menu dropdown-menu-right' data-type='sur".to_string() + &pk.to_string() + &"' style='top:30px;right:-10px;'>".to_string() + &drops +
    &"</div></div><form><div class='container answers_container'> <br><h4 class='m-0'>".to_string() + &survey.title +
    &"</h4><p class='position-relative'><a href=".to_string() + &link + &"' class='underline ajax'>".to_string() +
    &name + &"</a></p>".to_string() + &survey.get_time_description() + &"<br>".to_string() +
    &answers + &info + &"</div>".to_string() + &"</form></div>".to_string();
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
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "post_photo".to_string()),
                "vid" => add_video(pk, "post_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_music(pk, user.is_moderator(), user_id),
                "doc" => add_doc(pk, user.is_moderator(), user_id),
                "sur" => add_survey(pk, user.is_moderator(), user_id),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}

pub fn anon_post_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "post_photo".to_string()),
                "vid" => add_video(pk, "post_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_anon_music(pk),
                "doc" => add_anon_doc(pk),
                "sur" => add_anon_survey(pk),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
pub fn edit_post_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_edited_photo(pk, "post_photo".to_string()),
                "vid" => add_edited_video(pk, "post_video".to_string()),
                "goo" => add_edited_good(pk),
                "mus" => add_edited_music(pk),
                "doc" => add_edited_doc(pk),
                "sur" => add_edited_survey(pk),
                "use" => add_edited_user(pk),
                "com" => add_edited_community(pk),

                "lmu" => add_edited_music_list(pk),
                "ldo" => add_edited_doc_list(pk),
                "lpo" => add_edited_post_list(pk),
                "lvi" => add_edited_video_list(pk),
                "lph" => add_edited_photo_list(pk),
                "lgo" => add_edited_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        };
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}

pub fn comment_elements(attach: String, user_id: i32) -> String {
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
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "comment_photo".to_string()),
                "vid" => add_video(pk, "comment_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_music(pk, user.is_moderator(), user_id),
                "doc" => add_doc(pk, user.is_moderator(), user_id),
                "sur" => add_survey(pk, user.is_moderator(), user_id),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        };
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}

pub fn anon_comment_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "comment_photo".to_string()),
                "vid" => add_video(pk, "comment_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_anon_music(pk),
                "doc" => add_anon_doc(pk),
                "sur" => add_anon_survey(pk),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
pub fn edit_comment_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_edited_photo(pk, "comment_photo".to_string()),
                "vid" => add_edited_video(pk, "comment_video".to_string()),
                "goo" => add_edited_good(pk),
                "mus" => add_edited_music(pk),
                "doc" => add_edited_doc(pk),
                "sur" => add_edited_survey(pk),
                "use" => add_edited_user(pk),
                "com" => add_edited_community(pk),

                "lmu" => add_edited_music_list(pk),
                "ldo" => add_edited_doc_list(pk),
                "lpo" => add_edited_post_list(pk),
                "lvi" => add_edited_video_list(pk),
                "lph" => add_edited_photo_list(pk),
                "lgo" => add_edited_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}

pub fn message_elements(attach: String, user_id: i32) -> String {
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
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "message_photo".to_string()),
                "vid" => add_video(pk, "message_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_music(pk, user.is_moderator(), user_id),
                "doc" => add_doc(pk, user.is_moderator(), user_id),
                "sur" => add_survey(pk, user.is_moderator(), user_id),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}

pub fn anon_message_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_photo(pk, "message_photo".to_string()),
                "vid" => add_video(pk, "message_video".to_string()),
                "goo" => add_good(pk),
                "mus" => add_anon_music(pk),
                "doc" => add_anon_doc(pk),
                "sur" => add_anon_survey(pk),
                "use" => add_user(pk),
                "com" => add_community(pk),

                "lmu" => add_music_list(pk),
                "ldo" => add_doc_list(pk),
                "lpo" => add_post_list(pk),
                "lvi" => add_video_list(pk),
                "lph" => add_photo_list(pk),
                "lgo" => add_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
pub fn edit_message_elements(attach: String) -> String {
    let _connection = establish_connection();
    let v: Vec<&str> = attach.split(",").collect();
    let mut block = "".to_string();

    for item in v.iter() {
        if item.len() > 3 {
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3];

            let html = match code {
                "pho" => add_edited_photo(pk, "message_photo".to_string()),
                "vid" => add_edited_video(pk, "message_video".to_string()),
                "goo" => add_edited_good(pk),
                "mus" => add_edited_music(pk),
                "doc" => add_edited_doc(pk),
                "sur" => add_edited_survey(pk),
                "use" => add_edited_user(pk),
                "com" => add_edited_community(pk),

                "lmu" => add_edited_music_list(pk),
                "ldo" => add_edited_doc_list(pk),
                "lpo" => add_edited_post_list(pk),
                "lvi" => add_edited_video_list(pk),
                "lph" => add_edited_photo_list(pk),
                "lgo" => add_edited_good_list(pk),
                _ => "".to_string(),
            };
            block = block + &html;
        }
    }
    return "<div class='attach_container'>".to_owned() + &block + &"</div>".to_string();
}
