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
