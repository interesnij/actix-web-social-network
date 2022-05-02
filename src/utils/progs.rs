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

pub fn get_post_user_perm(user: User, request_user: User) -> (bool, String) {
    if user.is_user_in_block(request_user.id) {
        return (
            false,
            concat_string!(
                BAD_TOP,
                "<path d='M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z' fill='none' /><path d='M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z'/>",
                BAD_MEDIUM,
                "<a href='", user.get_link(),
                "' class='ajax underline'>", user.get_full_name(),
                "</a> ограничил", user.get_gender_a(),
                " доступ к своей странице.",
                BAD_BOTTOM
            )
        )
    }
    else { return (true, "".to_string());}
}

pub fn get_user_permission(user: User, request_user: User, part: &str)
    -> (bool, String) {

    if request_user.types > 10 {
        if request_user.is_closed() {
            let chat_pk = request_user.get_or_create_support_chat_pk().to_string();
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
        else { return (true, "".to_string());}
    }
    else {
        return match part {
            &"post" => get_post_user_perm(user, request_user),
            //"photo" => get_photo_user_perm(user, request_user),
            //"doc" => get_doc_user_perm(user, request_user),
            //"good" => get_good_user_perm(user, request_user),
            //"music" => get_music_user_perm(user, request_user),
            //"survey" => get_survey_user_perm(user, request_user),
            //"video" => get_video_user_perm(user, request_user),
            //"default" => get_default_user_perm(user, request_user),
            //"planner" => get_default_user_perm(user, request_user),
            //"forum" => get_forum_user_perm(user, request_user),
        };
    }
}
