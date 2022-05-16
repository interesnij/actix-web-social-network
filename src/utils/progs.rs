use crate::utils::establish_connection;

use crate::schema;
use diesel::prelude::*;
use crate::models::{
    User, Community,
    PostList, Post, PostComment,
    PhotoList, Photo, PhotoComment,
    Chat, Message, Doc, DocList, Music, MusicList,
    GoodList, Good, GoodComment,
    Survey, SurveyList,
    VideoList, Video, VideoComment,
};


pub fn get_user(pk: i32) -> User {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq(pk))
        .load::<User>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_community(pk: i32) -> Community {
    use crate::schema::communitys::dsl::communitys;
    let _connection = establish_connection();
    return communitys
        .filter(schema::communitys::id.eq(pk))
        .load::<Community>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_post_list(pk: i32) -> PostList {
    use crate::schema::post_lists::dsl::post_lists;
    let _connection = establish_connection();
    return post_lists
        .filter(schema::post_lists::id.eq(pk))
        .load::<PostList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_post(pk: i32) -> Post {
    use crate::schema::posts::dsl::posts;
    let _connection = establish_connection();
    return posts
        .filter(schema::posts::id.eq(pk))
        .load::<Post>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_post_comment(pk: i32) -> PostComment {
    use crate::schema::post_comments::dsl::post_comments;
    let _connection = establish_connection();
    return post_comments
        .filter(schema::post_comments::id.eq(pk))
        .load::<PostComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_photo_list(pk: i32) -> PhotoList {
    use crate::schema::photo_lists::dsl::photo_lists;
    let _connection = establish_connection();
    return photo_lists
        .filter(schema::photo_lists::id.eq(pk))
        .load::<PhotoList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_photo(pk: i32) -> Photo {
    use crate::schema::photos::dsl::photos;
    let _connection = establish_connection();
    return photos
        .filter(schema::photos::id.eq(pk))
        .load::<Photo>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_photo_comment(pk: i32) -> PhotoComment {
    use crate::schema::photo_comments::dsl::photo_comments;
    let _connection = establish_connection();
    return photo_comments
        .filter(schema::photo_comments::id.eq(pk))
        .load::<PhotoComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_good_list(pk: i32) -> GoodList {
    use crate::schema::good_lists::dsl::good_lists;
    let _connection = establish_connection();
    return good_lists
        .filter(schema::good_lists::id.eq(pk))
        .load::<GoodList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_good(pk: i32) -> Good {
    use crate::schema::goods::dsl::goods;
    let _connection = establish_connection();
    return goods
        .filter(schema::goods::id.eq(pk))
        .load::<Good>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_good_comment(pk: i32) -> GoodComment {
    use crate::schema::good_comments::dsl::good_comments;
    let _connection = establish_connection();
    return good_comments
        .filter(schema::good_comments::id.eq(pk))
        .load::<GoodComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_video_list(pk: i32) -> VideoList {
    use crate::schema::video_lists::dsl::video_lists;
    let _connection = establish_connection();
    return video_lists
        .filter(schema::video_lists::id.eq(pk))
        .load::<VideoList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_video(pk: i32) -> Video {
    use crate::schema::videos::dsl::videos;
    let _connection = establish_connection();
    return videos
        .filter(schema::videos::id.eq(pk))
        .load::<Video>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_video_comment(pk: i32) -> VideoComment {
    use crate::schema::video_comments::dsl::video_comments;
    let _connection = establish_connection();
    return video_comments
        .filter(schema::video_comments::id.eq(pk))
        .load::<VideoComment>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_doc_list(pk: i32) -> DocList {
    use crate::schema::doc_lists::dsl::doc_lists;
    let _connection = establish_connection();
    return doc_lists
        .filter(schema::doc_lists::id.eq(pk))
        .load::<DocList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_doc(pk: i32) -> Doc {
    use crate::schema::docs::dsl::docs;
    let _connection = establish_connection();
    return docs
        .filter(schema::docs::id.eq(pk))
        .load::<Doc>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_music_list(pk: i32) -> MusicList {
    use crate::schema::music_lists::dsl::music_lists;
    let _connection = establish_connection();
    return music_lists
        .filter(schema::music_lists::id.eq(pk))
        .load::<MusicList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_music(pk: i32) -> Music {
    use crate::schema::musics::dsl::musics;
    let _connection = establish_connection();
    return musics
        .filter(schema::musics::id.eq(pk))
        .load::<Music>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_survey_list(pk: i32) -> SurveyList {
    use crate::schema::survey_lists::dsl::survey_lists;
    let _connection = establish_connection();
    return survey_lists
        .filter(schema::survey_lists::id.eq(pk))
        .load::<SurveyList>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_survey(pk: i32) -> Survey {
    use crate::schema::surveys::dsl::surveys;
    let _connection = establish_connection();
    return surveys
        .filter(schema::surveys::id.eq(pk))
        .load::<Survey>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_chat(pk: i32) -> Chat {
    use crate::schema::chats::dsl::chats;
    let _connection = establish_connection();
    return chats
        .filter(schema::chats::id.eq(pk))
        .load::<Chat>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_message(pk: i32) -> Message {
    use crate::schema::messages::dsl::messages;
    let _connection = establish_connection();
    return messages
        .filter(schema::messages::id.eq(pk))
        .load::<Message>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

static BAD_TOP: &str = "<div class='card mt-3 centered'><div class='card-body' style='margin-top: 10%;'><svg fill='currentColor' class='thumb_big svg_default' viewBox='0 0 24 24'>";
static BAD_MEDIUM: &str = "</svg></div><h6 style='text-align: center;margin-bottom: 20px;' class='text-center'>";
static BAD_BOTTOM: &str = "</h6></div>";

pub fn get_user_permission(user: &User, request_user: &User)
    -> (bool, String) {

    if request_user.types > 10 {
        let chat_pk = request_user.get_or_create_support_chat_pk().to_string();
        if request_user.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Ваша страница заблокирована за нарушение правил сайта. Если Вы не согласны с примененными к Вашей странице санкциями, напишите в <a href='/chat/",
                    chat_pk,
                    "/' class='ajax underline'> техподдержку </a>",
                    BAD_BOTTOM
                )
            )
        }
        else if request_user.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Страница удалена. <br>Вы можете её <a class='pointer underline user_profile_restore'>восстановить</a>.",
                    BAD_BOTTOM
                )
            )
        }
        else if request_user.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Ваша страница будет разморожена",
                    request_user.get_longest_penalties(),
                    "Если Вы не согласны с примененными к Вашей странице санкциями, напишите в <a href='/chat/",
                    chat_pk,
                    "/' class='ajax underline'> техподдержку </a>",
                    BAD_BOTTOM
                )
            )
        }
        else { return (true, "".to_string());}
    }

    else if user.types > 10 {
        if user.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    user.link,
                    "'>",
                    user.get_full_name(),
                    "</a> заблокирован",
                    user.get_gender_a(),
                    " за нарушение правил сайта.",
                    BAD_BOTTOM
                )
            )
        }
        else if user.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    user.link,
                    "'>",
                    user.get_full_name(),
                    "</a> удалил страницу.",
                    BAD_BOTTOM
                )
            )
        }
        else if user.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Страница будет разморожена ",
                    user.get_longest_penalties(),
                    BAD_BOTTOM
                )
            )
        }
        else { return (true, "".to_string());}
    }
    else if user.is_user_in_block(request_user.id) {
        return (
            false,
            concat_string!(
                BAD_TOP,
                "<path d='M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z' fill='none' /><path d='M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z'/>",
                BAD_MEDIUM,
                "<a href='",
                user.link,
                "' class='ajax underline'>",
                user.get_full_name(),
                "</a> ограничил",
                user.get_gender_a(),
                " доступ к своей странице.",
                BAD_BOTTOM
            )
        )
    }
    else if !user.is_user_can_see_all(request_user.id) {
        return (
            false,
            concat_string!(
                BAD_TOP,
                "<path d='M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z' fill='none' /><path d='M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z'/>",
                BAD_MEDIUM,
                "<a href='",
                user.link,
                "' class='ajax underline'>",
                user.get_full_name(),
                "</a> закрыл",
                user.get_gender_a(),
                " доступ к своим данным.",
                BAD_BOTTOM
            )
        )
    }
    else {
        return (true, "".to_string())
    }
}

pub fn get_anon_user_permission(user: &User) -> (bool, String) {
    if user.types > 10 {
        if user.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    user.link,
                    "'>",
                    user.get_full_name(),
                    "</a> заблокирован",
                    user.get_gender_a(),
                    " за нарушение правил сайта.",
                    BAD_BOTTOM
                )
            )
        }
        else if user.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    user.link,
                    "'>",
                    user.get_full_name(),
                    "</a> удалил страницу.",
                    BAD_BOTTOM
                )
            )
        }
        else if user.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Страница будет разморожена ",
                    user.get_longest_penalties(),
                    BAD_BOTTOM
                )
            )
        }
        else { return (true, "".to_string());}
    }
    else if !user.is_anon_user_can_see_all() {
        return (
            false,
            concat_string!(
                BAD_TOP,
                "<path d='M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z' fill='none' /><path d='M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z'/>",
                BAD_MEDIUM,
                "<a href='",
                user.link,
                "' class='ajax underline'>",
                user.get_full_name(),
                "</a> закрыл",
                user.get_gender_a(),
                " доступ к своим данным.",
                BAD_BOTTOM
            )
        )
    }
    else {
        return (true, "".to_string())
    }
}


pub fn get_community_permission(community: &Community, request_user: &User)
    -> (bool, String) {

    if request_user.types > 10 {
        let chat_pk = request_user.get_or_create_support_chat_pk().to_string();
        if request_user.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Ваша страница заблокирована за нарушение правил сайта. Если Вы не согласны с примененными к Вашей странице санкциями, напишите в <a href='/chat/",
                    chat_pk,
                    "/' class='ajax underline'> техподдержку </a>",
                    BAD_BOTTOM
                )
            )
        }
        else if request_user.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Страница удалена. <br>Вы можете её <a class='pointer underline user_profile_restore'>восстановить</a>.",
                    BAD_BOTTOM
                )
            )
        }
        else if request_user.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Ваша страница будет разморожена",
                    request_user.get_longest_penalties(),
                    "Если Вы не согласны с примененными к Вашей странице санкциями, напишите в <a href='/chat/",
                    chat_pk,
                    "/' class='ajax underline'> техподдержку </a>",
                    BAD_BOTTOM
                )
            )
        }
        else { return (true, "".to_string());}
    }

    else if community.types > 10 {
        if community.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    community.link,
                    "'>",
                    community.name,
                    "</a> заблокировано за нарушение правил сайта.",
                    BAD_BOTTOM
                )
            )
        }
        else if community.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Сщщбщество <a href='",
                    community.link,
                    "'>",
                    community.name,
                    "</a> удалено.",
                    BAD_BOTTOM
                )
            )
        }
        else if community.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Сообщество будет разморожено ",
                    community.get_longest_penalties(),
                    BAD_BOTTOM
                )
            )
        }
        else { return (true, "".to_string());}
    }
    else if request_user.is_banned_from_community(community.id) {
        return (
            false,
            concat_string!(
                BAD_TOP,
                "<path d='M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z' fill='none' /><path d='M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z'/>",
                BAD_MEDIUM,
                "<a href='",
                community.link,
                "' class='ajax underline'>",
                community.name,
                "</a> добавило Вас в чёрный список",
                BAD_BOTTOM
            )
        )
    }
    else {
        return (true, "".to_string())
    }
}

pub fn get_anon_community_permission(community: &Community) -> (bool, String) {
    if community.types > 10 {
        if community.is_closed() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "<a href='",
                    community.link,
                    "'>",
                    community.name,
                    "</a> заблокировано за нарушение правил сайта.",
                    BAD_BOTTOM
                )
            )
        }
        else if community.is_deleted() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/></g><g><path d='M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z'/></g>",
                    BAD_MEDIUM,
                    "Сщщбщество <a href='",
                    community.link,
                    "'>",
                    community.name,
                    "</a> удалено.",
                    BAD_BOTTOM
                )
            )
        }
        else if community.is_suspended() {
            return (
                false,
                concat_string!(
                    BAD_TOP,
                    "<g><rect fill='none' height='24' width='24'/><path d='M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z'/></g>",
                    BAD_MEDIUM,
                    "Сообщество будет разморожено ",
                    community.get_longest_penalties(),
                    BAD_BOTTOM
                )
            )
        }
        else {
            return (true, "".to_string())
        }
    }
    else {
        return (true, "".to_string())
    }
}

pub fn custom_link_check(value: &str) -> (bool, String) {
    let exclude_chars = [
        ",", ":", ";", ">", "@", "$",
        "*", "/", "!", "?", "-", "+",
        "{", "}", "(", ")", "%", "&",
        "¤", "#", "^", "~", "[", "]",
        "<", ">",
    ];
    let words_list = [
        "chat", "chats_list", "chat_list",
        "community", "communities",
        "doc", "docs", "doc_lists", "docs_list",
        "friends", "friend", "follows", "follow", "followings",
        "good", "goods", "goods_list", "good_comments", "good_comment",
        "music", "track", "playlist", "playlists", "music_list", "music_lists", "music_album", "music_albums", "genre",
        "notify", "notification", "notifications_list", "notifications",
        "manager", "managers", "managers_list", "penalties_list", "penaltie_list", "penalties", "penaltie", "claims", "claims_list", "claim",
        "photo", "photos", "photo_list", "photo_comments", "photo_comment",
        "post", "posts", "post_list", "post_comments", "post_comment",
        "survey", "surveys", "survey_list",
        "video", "videos", "video_list", "video_comments", "video_comment",
        "user", "users", "user_list", "users_list",
        "admin", "staff", "staffed", "static", "media",
    ];
    if &value.len() < &5 {
        return (false, "Слишком короткая ссылка".to_string());
    }
    else if &value.len() > &32 {
        return (false, "Слишком длинная ссылка".to_string());
    }

    else if &value[..2] == "id".to_string()
        || &value[..6] == "public".to_string()
        || words_list.iter().any(|&i| i==value) {
            return (false, "Адрес занят".to_string());
        }

    for i in exclude_chars.iter() {
        if value.contains(i) {
            return (false, "Недопустимый формат".to_string());
        }
    }

    use crate::schema::custom_links::dsl::custom_links;
    use crate::models::CustomLink;

    let _connection = establish_connection();
    let _links = custom_links
        .filter(schema::custom_links::link.eq(value))
        .load::<CustomLink>(&_connection)
        .expect("E.");

    if _links.len() > 0 {
        return (false, "Адрес занят".to_string());
    }
    else {
        return (true, "Занять адрес".to_owned() + value);
    }
}
