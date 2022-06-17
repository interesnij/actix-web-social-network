use crate::utils::establish_connection;
use crate::schema;
use diesel::prelude::*;
use crate::models::User;


pub fn add_post(pk: i32, user_id: i32, is_staff: bool) -> String {
    use crate::utils::get_post;

    let name : String;
    let link : String;
    let image : String;
    let react_container : String;

    let post = get_post(pk);
    let post_list = post.get_list();
    let reactions_list = post_list.get_reactions_list();
    if post.reactions == 0 {
        react_container = "<span class='react_items' data-type='pos".to_owned() + &post.id.to_string() + &"' style='display: inline-flex'></span>".to_string();
    }
    else {
        let object_reactions_count = post.get_or_create_react_model();
        let mut user_reaction = 0;
        if post.is_have_user_reaction(user_id) {
            user_reaction = post.get_user_reaction(user_id);
        }
        let mut reacts = "".to_string();
        for reaction in reactions_list.iter() {
            let count = object_reactions_count.count_reactions_of_types(*reaction);
            let mut border_radius = "".to_string();
            if count != 0 {
                let count_str = count.to_string();

                let users = post.get_6_reactions_users_of_types(*reaction);
                if &user_reaction == reaction {
                    border_radius = "border_radius".to_string();
                }
                let reaction_str = reaction.to_string();
                let mut users_html = "".to_string();
                for user in users.iter() {
                    users_html = concat_string!(
                        "<a href='", user.link, "' target='_blank' tooltip='",
                        user.get_full_name(),
                        "' flow='up' style='padding-right:10px' data-pk='",
                        user.id.to_string(), "'><figure style='margin: 0;'>",
                        user.get_s_avatar(), "</figure></a>"
                    );
                }
                reacts = concat_string!(
                    reacts, "<span class='react' data-react='", reaction_str,
                    "'><span class='like send_react ", border_radius,
                    "<img style='width:17px' src='/static/images/reactions/",
                    reaction_str, ".png' alt='img' /><span class='reactions_count'>",
                    count_str, "</span></span><span class='like_window'><div class='like_pop'>
                    <span class='item_reactions pointer'>Отреагировали: <span data-count='like'>",
                    count_str, "<span class='like_list' style='display:flex;flex-wrap:wrap;margin-top:10px;'>",
                    users_html, "</span></div></span></span>"
                );
            }
        }
        react_container = concat_string!(
            "<span class='react_items' data-type='pos",
            post.id.to_string(),
            "' style='display: inline-flex'>", reacts, "</span>");
    }

    if post.community_id.is_some() {
        let community = post.get_community();
        name = community.name.clone();
        link = community.link.clone();
        image = community.get_bb_avatar();
    }
    else {
        let creator = post.get_creator();
        name = creator.get_full_name().clone();
        link = creator.link.clone();
        image = creator.get_bb_avatar();
    }

    let mut comment_enabled = "".to_string();
    if !post.comment_enabled {
        comment_enabled = "style='display:none'".to_string();
    }


    let mut drops = "<span class='dropdown-item create_repost'>Добавить</span><span class='dropdown-item copy_link'>Копировать ссылку</span>".to_string();
    if post.is_user_can_edit_delete_item(user_id) {
        drops = drops + &"<span class='dropdown-item post_edit'>Изменить</span><span class='dropdown-item post_remove'>Удалить</span>".to_string();
    }
    else if is_staff == true {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }

    return concat_string!(
        "<div class='pag card mb-3' data-type='user_post' data-pk='",
        post.id.to_string(),
        "'><div class='card-header'><div class='media'><a href='",
        link, "' class='ajax'><figure>",
        image, "</figure></a><div class='media-body'><h6 class='mb-0'><a href='",
        link, "' class='ajax'>", name,
        "</a></h6><p class='mb-0 fullscreen_2 pointer'>",
        post.created.format("%d-%m-%Y в %H:%M").to_string(),
        "</p></div><div class='dropdown'><a class='icon-circle icon-30 btn_default drop pointer'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
        </a><div class='dropdown-menu dropdown-menu-right'><span><span class='dropdown-item item_likes pointer'>Оценили</span>
        <span class='dropdown-item item_dislikes pointer'>Не оценили</span></span>",
        drops, "</div></div></div></div><div class='fullscreen text_support pointer'>",
        post.get_format_text(), "</div>", post.get_attach(user_id),
        "<div class='card-footer border-top py-2'><div class='row'>
        <div class='col interaction' data-type='pos'",
        post.id.to_string(), "'>",
        react_container,
        "<span title='Комментарий' class='pointer load_comments_list btn_default'
        style='margin-right: 5px;",
        comment_enabled, "'>
        <svg viewBox='0 0 24 24' class='svg_info' fill='currentColor'>
        <path d='M0 0h24v24H0V0z' fill='none'></path><path d='M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H6l-2 2V4h16v12z'></path></svg><span class='comment-count'>",
        post.count_comments().to_string(),
        "</span></span> <span title='Поделиться' class='create_repost btn_default pointer'><svg class='svg_info repost_style_btn' viewBox='0 0 24 24' fill='currentColor'><path d='m0 0h24v24h-24z' fill='none'></path><path fill='currentColor' d='m12.1 7.87v-3.47a1.32 1.32 0 0 1 2.17-1l8.94 7.6a1.32 1.32 0 0 1 .15 1.86l-.15.15-8.94 7.6a1.32 1.32 0 0 1 -2.17-1v-3.45c-4.68.11-8 1.09-9.89 2.87a1.15 1.15 0 0 1 -1.9-1.11c1.53-6.36 5.51-9.76 11.79-10.05zm1.8-2.42v4.2h-.9c-5.3 0-8.72 2.25-10.39 6.86 2.45-1.45 5.92-2.16 10.39-2.16h.9v4.2l7.71-6.55z'></path></svg><span class='repost_count'>",
        post.count_reposts().to_string(),
        "</span></span></div><span class='small' style='float: right;' title='Просмотры'>
        <svg fill='currentColor' class='svg_info svg_default' style='width:17px;'
        viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z' /></svg>
        </span></div><div class='load_comments'></div></div></div>"
    );
}

pub fn add_post_list(pk: i32) -> String {
    use crate::schema::post_lists::dsl::post_lists;
    use crate::models::PostList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let image : String;
    let owner : String;

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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body' owner-pk='",
        owner, "' postlist-pk='", list.id.to_string(),
        "' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'><figure><a class='load_post_list btn_default pointer'>",
        image,
        "</a></figure><div class='media-body' style='margin-left: 10px;'><h6 class='my-0 mt-1 load_post_list pointer'>",
        list.name,
        "</h6><p>Список записей: <a style='vertical-align: baseline;'class='ajax underline' href='",
        link, "'>", name, "</a><br>Записей: ",
        list.count.to_string(), "</p></div></div></div></div>");
}

pub fn add_edited_post_list(pk: i32) -> String {
    use crate::schema::post_lists::dsl::post_lists;
    use crate::models::PostList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' postlist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='lpo",
        list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_postlist pointer'><svg width='50' height='50' viewBox='0 0 24 24'
        fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round'
        stroke-linejoin='round'><polygon points='5 3 19 12 5 21 5 3'></polygon></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_postlist pointer'>
        <a class='nowrap'>", list.name, " (",
        list.count_items(), ")</a></p></div><small class='file-accessed pointer
        post_attach_list_remove underline'>Открепить</small></div></div>");
}

pub fn add_music_list(pk: i32) -> String {
    use crate::schema::music_lists::dsl::music_lists;
    use crate::models::MusicList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let owner : String;

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

    let image : String;
    if list.image.is_some() {
        image = "<img src='".to_owned() + &list.image.as_ref().unwrap() + &"' style='width:120px;height:120px;' alt='image'>".to_string();
    }
    else {
        image = "<svg fill='currentColor' class='svg_default border' style='width:120px;height:120px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M15 6H3v2h12V6zm0 4H3v2h12v-2zM3 16h8v-2H3v2zM17 6v8.18c-.31-.11-.65-.18-1-.18-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3V8h3V6h-5z'/></svg>".to_string();
    }
    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body' owner-pk='",
        owner, "' playlist-pk='", list.id.to_string(),
        "' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
        <figure><a class='load_music_list btn_default pointer'>",
        image, "</a></figure><div class='media-body' style='margin-left: 10px;'>
        <h6 class='my-0 mt-1 load_music_list pointer'>",
        list.name, "</h6><p>Плейлист: <a style='vertical-align: baseline;'class='ajax underline' href='",
        link, "'>", name, "</a><br>Треков: ",
        list.count.to_string(), "</p></div><span class='playlist_share'></span></div></div></div>");
}
pub fn add_edited_music_list(pk: i32) -> String {
    use crate::schema::music_lists::dsl::music_lists;
    use crate::models::MusicList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' playlist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='lmu", list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_playlist pointer'><svg width='50' height='50' viewBox='0 0 24 24'
        fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round'
        stroke-linejoin='round'><polygon points='5 3 19 12 5 21 5 3'></polygon></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_playlist pointer'>
        <a class='nowrap'>", list.name, " (",
        list.count_items(), ")</a></p></div><small class='file-accessed pointer
        music_attach_list_remove underline'>Открепить</small></div></div>");
}

pub fn add_doc_list(pk: i32) -> String {
    use crate::schema::doc_lists::dsl::doc_lists;
    use crate::models::DocList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let owner : String;

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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body' owner-pk='",
        owner, "' doclist-pk='", list.id.to_string(),
        "' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
        <figure><a class='load_doc_list btn_default pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:100%;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
        <h6 class='my-0 mt-1 load_doc_list pointer'>",
        list.name, "</h6><p><a style='vertical-align: baseline;'
        class='ajax underline' href='",
        link, "'>", name, "</a> список документов<br>Документов: ",
        list.count.to_string(), "</p></div></div></div></div>");
}
pub fn add_edited_doc_list(pk: i32) -> String {
    use crate::schema::doc_lists::dsl::doc_lists;
    use crate::models::DocList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' doclist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='ldo", list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_doc_list pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13'/></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_doc_list pointer'>
        <a class='nowrap'>", list.name, " (",
        list.count_items(), ")</a></p></div><small class='file-accessed pointer
        doc_attach_list_remove underline'>Открепить</small></div></div>");
}

pub fn add_video_list(pk: i32) -> String {
    use crate::schema::video_lists::dsl::video_lists;
    use crate::models::VideoList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let owner : String;

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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body' owner-pk='",
        owner, "' videolist-pk='", list.id.to_string(),
        "' style='padding: 8px;padding-bottom: 0;'><div style='display:flex'>
        <figure><a class='load_video_list btn_default pointer'><svg fill='currentColor' class='svg_default border' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg></a></figure><div class='media-body' style='margin-left: 10px;'>
        <h6 class='my-0 mt-1 load_video_list pointer'>",
        list.name, "</h6><p>Список видеозаписей: <a style='vertical-align: baseline;'
        class='ajax underline' href='", link,
        "'>", name, "</a><br>Видеозаписей: ",
        list.count.to_string(), "</p></div></div></div></div>");
}
pub fn add_edited_video_list(pk: i32) -> String {
    use crate::schema::video_lists::dsl::video_lists;
    use crate::models::VideoList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' videolist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='lvi", list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_videolist pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_videolist pointer'>
        <a class='nowrap'>", list.name, " (",
        list.count_items(), ")</a></p></div><small class='file-accessed pointer
        video_attach_list_remove underline'>Открепить</small></div></div>");
}

pub fn add_photo_list(pk: i32) -> String {
    use crate::schema::photo_lists::dsl::photo_lists;
    use crate::models::PhotoList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let owner : String;

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

    return concat_string!(
        "<div class='custom_color mb-1 text-center has-background-img
        position-relative box-shadow' owner-pk='", owner,
        "' photolist-pk='", list.id.to_string(),
        "' style='width: 100%;flex-basis: 100%;'>
        <figure class='background-img'><img src='",
        list.get_cover_photo(), "' </figure><div class='container'><br>
        <h4 class='load_photo_list pointer'><a>", list.name,
        "</a></h4><p class='lead'><a class='ajax underline' href='",
        link, "'>", name, "</a></p>
        <hr class='my-3'><a class='load_photo_list pointer'>",
        list.count_items_ru(), "</a><div class='row'></div></div></div>");
}
pub fn add_edited_photo_list(pk: i32) -> String {
    use crate::schema::photo_lists::dsl::photo_lists;
    use crate::models::PhotoList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' photolist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='lph", list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_photolist pointer'><svg fill='currentColor' class='svg_default' style='width:60px;height:88px;' viewBox='0 0 24 24'><path d='M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z'></path></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_photolist pointer'>
        <a class='nowrap'>", list.name, " (",
        &list.count_items(), ")</a></p></div><small class='file-accessed pointer
        photo_attach_list_remove underline'>Открепить</small></div></div>");
}

pub fn add_good_list(pk: i32) -> String {
    use crate::schema::good_lists::dsl::good_lists;
    use crate::models::GoodList;
    let _connection = establish_connection();

    let name : String;
    let link : String;
    let image : String;

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

    return concat_string!(
        "<div goodlist-pk='", list.id.to_string(),
        "' style='padding: 7px;width: 100%;flex-basis: 100%'><div class='media mb-2'>
        <div class='media-body'><h6 class='content-color-primary mb-0 load_good_list pointer'><a>",
        list.name, "</a></h6></div><span class='small'></span></div>
        <div class='centered no-gutters'><figure class='mx-auto mb-3' style='width:120px'>
        <img class='load_good_list pointer image_fit_small' src='",
        image, "' style='border-radius:50%' /></figure></div>
        <h5 class='mb-2 header-color-primary text-center'><a href='", link,
        "' class='ajax underline'>", name, "</a></h5>
        <h6 class='card-subtitle header-color-secondary text-center'>",
        list.count_items_ru(), "</h6></div>");
}
pub fn add_edited_good_list(pk: i32) -> String {
    use crate::schema::good_lists::dsl::good_lists;
    use crate::models::GoodList;
    let _connection = establish_connection();

    let owner : String;

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

    return concat_string!(
        "<div class='folder' owner-pk='", owner,
        "' goodlist-pk='", list.id.to_string(),
        "' style='text-align: center;padding: 3px;'><span><input type='hidden' name='attach_items'
        value='lgo", list.id.to_string(),
        "'></span><div class='card-img-top file-logo-wrapper' style='padding: 2rem;'>
        <a class='nowrap'><div class='d-flex align-items-center justify-content-center
        w-100 load_goodlist pointer'><svg fill='currentColor' class='svg_default' viewBox='0 0 24 24'><g><rect fill='none' /><path d='M18,6h-2c0-2.21-1.79-4-4-4S8,3.79,8,6H6C4.9,6,4,6.9,4,8v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8C20,6.9,19.1,6,18,6z M12,4c1.1,0,2,0.9,2,2h-4C10,4.9,10.9,4,12,4z M18,20H6V8h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8h4v2c0,0.55,0.45,1,1,1s1-0.45,1-1V8 h2V20z'/></g></svg>
        </div></a></div><div class='card-body pt-0'><div class='content-wrapper'
        style='display: flex;'><p class='card-text file-name mb-0 load_goodlist pointer'>
        <a class='nowrap'>", list.name, " (",
        list.count_items(), ")</a></p></div><small class='file-accessed pointer
        good_attach_list_remove underline'>Открепить</small></div></div>");
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

    return concat_string!(
        "<div class='photo'><span class='photo_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items' value='pho",
        photo.id.to_string(),
        "'></span><div class='progressive replace ",
        case, " pointer' data-href='", photo.file, "' photo-pk='",
        photo.file, "'><img class='preview image_fit' width='20' height='15' loading='lazy' src='",
        photo.preview, "' alt='img'></div></div>");
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

     return concat_string!(
         "<div class='video'><img class='image_fit' src='", video.get_image(),
         "' alt='img'><div class='video_icon_play_v2 ", case,
         "' video-pk='", pk.to_string(), "' video-counter=''></div></div>");
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

    return concat_string!(
        "<div class='video'><span class='video_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items' value='vid",
        video.id.to_string(), "'></span><img class='image_fit' src='",
        video.get_image(), "' alt='img'><div class='video_icon_play_v2 ",
        case, "' video-pk='", pk.to_string(), "' video-counter=''></div></div>");
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

    return concat_string!(
        "<div class='card has-background-img good_detail mb-3 pointer' good-pk='",
        good.id.to_string(),
        "' style='flex-basis: 100%;'><figure class='background-img shadow-dark'>",
        good.get_image(), "</figure><div class='card-header'><div class='media'><div class='media-body'><h4 class='text-white mb-0'>",
        good.title, "</h4></div></div></div><div class='card-body spantshirt'></div><div class='card-footer'><p class='small mb-1 text-success'>",
        good.get_price(), "</p></div></div>");
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

        return concat_string!(
            "<div class='card has-background-img good_detail mb-3 pointer' good-pk='",
            good.id.to_string(), "' style='flex-basis: 100%;'>
            <span class='good_preview_delete' tooltip='Не прикреплять' flow='up'><span><input type='hidden' name='attach_items' value='goo",
            good.id.to_string(), "'></span>
            <figure class='background-img shadow-dark'>", good.get_image(),
            "</figure><div class='card-header'><div class='media'><div class='media-body'><h4 class='text-white mb-0'>",
            good.title, "</h4></div></div></div><div class='card-body spantshirt'></div><div class='card-footer'><p class='small mb-1 text-success'>",
            good.get_price(), "</p></div></div>");
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
    if music.is_user_can_edit_delete_item(user_id) {
        drops = drops + &"<span class='dropdown-item track_edit'>Изменить</span><span class='dropdown-item track_remove'>Удалить</span>".to_string();
    }
    else if is_staff {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }

    return concat_string!(
        "<div class='music' data-path='", music.file,
        "' style='flex-basis: auto;width:100%;position: relative;'><div class='media'
        music-counter=''>", music.get_s_image(),
        "<div class='media-body' style='display: flex;'><h6 class='music_list_post music_title'><a>",
        music.title, "</a></h6><span class='span_btn' data-pk='", music.id.to_string(),
        "'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
        fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
        </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='mus",
        music.id.to_string(), "'>",
        drops, "</div></span</span></div></div></div>");
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

    return concat_string!(
        "<div style='display: flex; padding: 3px;'><span class='music_preview_delete'
        tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
        <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
        </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
        <input type='hidden' name='attach_items' value='mus", music.id.to_string(),
        "'></span><span><svg width='30' height='30' viewBox='0 0 24 24' fill='none'
        stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'
        ><polygon points='5 3 19 12 5 21 5 3'></polygon></svg></span>
        <span style='margin-left: 10px; margin-right: 40px; overflow: hidden;'>
        <h6 class='music_list_item pointer music_title' style='padding-top: 4px;'>
        <a>", music.title, "</a></h6></span></div>");
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

    return concat_string!(
        "<div class='music' data-path='", music.file,
        "' style='flex-basis: auto;width:100%;position: relative;'><div class='media'
        music-counter=''>", music.get_s_image(),
        "<div class='media-body' style='display: flex;'><h6 class='music_list_post music_title'><a>",
        music.title, "</a></h6><span class='span_btn' data-pk='", music.id.to_string(),
        "'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
        fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
        </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='mus",
        music.id.to_string(), "'>",
        drops, "</div></span</span></div></div></div>");
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

    return concat_string!(
        "<div class='doc' data-path='", doc.file,
        "' style='flex-basis: auto;width:100%;position: relative;'><div class='media'>
        <svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
        <div class='media-body' style='display: flex;'><h6 class='doc_title'><a>",
        doc.title, "</a></h6><span class='span_btn' data-pk='", doc.id.to_string(),
        &"'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
        fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
        </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='doc",
        doc.id.to_string(), "'>",
        drops, "</div></span</span></div></div></div>");
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
    if doc.is_user_can_edit_delete_item(user_id) {
        drops = drops + &"<span class='dropdown-item track_edit'>Изменить</span><span class='dropdown-item track_remove'>Удалить</span>".to_string();
    }
    else if is_staff == true {
        drops = drops + &"<span class='dropdown-item create_close'>Закрыть</span>".to_string();
    }
    else {
        drops = drops + &"<span class='dropdown-item create_claim'>Пожаловаться</span>".to_string();
    }

    return concat_string!(
        "<div class='doc' data-path='", doc.file,
        "' style='flex-basis: auto;width:100%;position: relative;'><div class='media'>
        <svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
        <div class='media-body' style='display: flex;'><h6 class='doc_title'><a>",
        doc.title, "</a></h6><span class='span_btn' data-pk='", doc.id.to_string(),
        "'><span class='dropdown' style='position: inherit;'><a class='btn_default drop pointer'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z'
        fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' />
        </svg></a><div class='dropdown-menu dropdown-menu-right' style='top: 25px;' data-type='doc",
        doc.id.to_string(), "'>",
        drops, "</div></span</span></div></div></div>");
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

        return concat_string!(
            "<div style='display: flex; padding: 3px;'><span class='doc_preview_delete'
            tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
            <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
            </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
            <input type='hidden' name='attach_items' value='mus", doc.id.to_string(),
            "'></span><span><svg fill='currentColor' class='svg_default' style='width:45px;margin: 0;' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none'/><path d='M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z'/></svg>
            </span><span style='margin-left: 10px; margin-right: 40px; overflow: hidden;'>
            <h6 class='music_list_item pointer music_title' style='padding-top: 4px;'>
            <a href='", doc.file, "' style='white-space: nowrap;' target='_blank' rel='nofollow'>",
            doc.title, "</a></h6></span></div>");
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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body'
        style='padding: 5px'><div style='display:flex'><figure><a class='ajax' href='",
        user.link, "' >",
        user.get_bb_avatar(), "</a></figure><div class='media-body' style='margin-left: 10px;'>
        <a href='", user.link, "' class='my-0 mt-1 ajax'>",
        user.get_full_name(), "</a><p>",
        user.get_online_status(), "<br>Друзей: ",
        user.get_profile().friends.to_string(),
        "</p></div></div></div></div>");
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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body'
        style='padding: 5px'> <span class='doc_preview_delete'
        tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
        <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
        </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
        <input type='hidden' name='attach_items' value='use", user.id.to_string(),
        "'></span> <div style='display:flex'><figure><a class='ajax'
        href='", user.link.to_string(), "' >",
        user.get_bb_avatar(), "</a></figure><div class='media-body' style='margin-left: 10px;'>
        <a href='", user.link.to_string(),
        "' class='my-0 mt-1 ajax'>", user.get_full_name(),
        "</a><p>", user.get_online_status(), "<br>Друзей: ",
        user.get_profile().friends.to_string(), "</p></div></div></div></div>");
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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body'
        style='padding: 5px'><div style='display:flex'><figure><a class='ajax'
        href='", community.link, "' >", community.get_bb_avatar(), "</a></figure><div class='media-body' style='margin-left: 10px;'>
        <a href='", community.link, "' class='my-0 mt-1 ajax'>", community.name,
        "</a><p>", community.description.as_ref().unwrap(), "<br>Подписчиков: ",
        community.count_members().to_string(), "</p></div></div></div></div>");
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

    return concat_string!(
        "<div style='flex-basis: 100%;' class='card'><div class='card-body' style='padding: 5px'>
        <span class='doc_preview_delete' tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
        <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
        </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
        <input type='hidden' name='attach_items' value='com",
        community.id.to_string(),
        "'></span><div style='display:flex'><figure><a class='ajax' href='",
        community.link, "' >", community.get_bb_avatar(),
        "</a></figure><div class='media-body' style='margin-left: 10px;'>
        <a href='",
        community.link,
        "' class='my-0 mt-1 ajax'>",
        community.name,
        "</a><p>",
        community.description.as_ref().unwrap(),
        "<br>Подписчиков: ",
        community.count_members().to_string(),
        "</p></div></div></div></div>");
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

    let name : String;
    let link : String;
    let mut answers = "".to_string();
    let info : String;
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
        answers = concat_string!(
            answers, "<div data-pk='", answer.id.to_string(),
            "' class='lite_color answer_style pointer survey_vote'>
            <div class='progress2' style='width:'",
            answer.get_procent().to_string(),
            "%;'></div><span class='progress_span_r'>", answer.content,
            " <span class='count text-muted small'>",answer.vote.to_string(),
            "</span></span><span class='progress_span_l' style='margin-left: auto;'>
            <span class='vote_svg'></span><span class='procent'>",
            answer.get_procent().to_string(), "%</span></span></div>");
    }

    return concat_string!(
        "<div data-pk='", survey.id.to_string(),
        "' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
        <figure class='background-img'><img src='", survey.get_image(),
        "alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
        </a><div class='dropdown-menu dropdown-menu-right' data-type='sur",
        pk.to_string(), "' style='top:30px;right:-10px;'>", drops,
        "</div></div><form><div class='container answers_container'> <br><h4 class='m-0'>",
        survey.title, "</h4><p class='position-relative'><a href=",
        link, "' class='underline ajax'>", name, "</a></p>",
        survey.get_time_description(), "<br>",
        answers, info, "</div>", "</form></div>");
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

    let name : String;
    let link : String;
    let mut multiple_class = "".to_string();
    let mut answers = "".to_string();
    let mut vote_svg = "".to_string();
    let info : String;

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
        answers = concat_string!(
            answers, "<div data-pk='", answer.id.to_string(),
            "' class='lite_color answer_style pointer survey_vote'>
            <div class='progress2' style='width:'",
            answer.get_procent().to_string(), "%;'></div><span class='progress_span_r'>",
            answer.content, " <span class='count text-muted small'>", answer.vote.to_string(),
            "</span></span><span class='progress_span_l' style='margin-left: auto;'>
            <span class='vote_svg'>", vote_svg, "</span><span class='procent'>",
            answer.get_procent().to_string(), "%</span></span></div>");
    }

    return concat_string!(
        "<div data-pk='", survey.id.to_string(),
        "' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
        <figure class='background-img'><img src='", survey.get_image(),
        "alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
        </a><div class='dropdown-menu dropdown-menu-right' data-type='sur",
        pk.to_string(), "' style='top:30px;right:-10px;'>
        <span class='dropdown-item copy_link'>Копировать ссылку</span>", drops,
        "</div></div><form><div class='container answers_container ", multiple_class,
        "'> <br><h4 class='m-0'>", survey.title,
        "</h4><p class='position-relative'><a href=", link, "' class='underline ajax'>",
        name, "</a></p>", survey.get_time_description(), "<br>",
        answers, info, "</div><div class='card-footer position-relative'>
        <button type='button' class='btn hidden btn-sm float-left border votes_remove'>Отмена</button>
        <button id='add_vote_survey_btn' type='button' class='btn hidden btn-sm btn-success float-right'>
        Проголосовать</button></div>", "</form></div>");
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

    let name : String;
    let link : String;
    let mut answers = "".to_string();
    let info : String;
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
        answers = concat_string!(
            answers,
            "<div data-pk='",
            answer.id.to_string(),
            "' class='lite_color answer_style pointer survey_vote'>
            <div class='progress2' style='width:'",
            answer.get_procent().to_string(),
            "%;'></div><span class='progress_span_r'>",
            answer.content,
            " <span class='count text-muted small'>",
            answer.vote.to_string(),
            "</span></span><span class='progress_span_l' style='margin-left: auto;'>
            <span class='vote_svg'></span><span class='procent'>",
            answer.get_procent().to_string(),
            "%</span></span></div>");
    }

    return concat_string!(
        "<div data-pk='",
        survey.id.to_string(),
        "' class='card p-1 border text-center position-relative box-shadow' style='flex-basis: 100%;'>
        <span class='survey_preview_delete'
        tooltip='Не прикреплять' flow='up'><svg fill='#FF0000' viewBox='0 0 24 24'>
        <path d='M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z'>
        </path><path d='M0 0h24v24H0z' fill='none'></path></svg></span><span>
        <input type='hidden' name='attach_items' value='sur",
        survey.id.to_string(),
        "'></span><figure class='background-img'><img src='",
        survey.get_image(),
        "alt='img' ></figure><div class='dropdown'><a class='btn_default drop pointer' style='position:absolute;right:5px;top:5px;'>
        <svg class='svg_info' fill='currentColor' viewBox='0 0 24 24'><path d='M0 0h24v24H0z' fill='none' /><path d='M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z' /></svg>
        </a><div class='dropdown-menu dropdown-menu-right' data-type='sur",
        pk.to_string(),
        "' style='top:30px;right:-10px;'>",
        drops,
        "</div></div><form><div class='container answers_container'> <br><h4 class='m-0'>",
        survey.title,
        "</h4><p class='position-relative'><a href='",
        link,
        "' class='underline ajax'>",
        name,
        "</a></p>",
        survey.get_time_description(),
        "<br>",
        answers,
        info,
        "</div>",
        "</form></div>");
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
