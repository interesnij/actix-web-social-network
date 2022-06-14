use actix_web::{
    HttpResponse,
    web,
    //web::Json,
    error::InternalError,
    http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    //get_community_permission,
    //get_user_permission,
    get_chat,
    //get_message,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::{User, Chat, Message};
use serde::{Deserialize, Serialize};

use std::str;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{borrow::BorrowMut, io::Write};
//use crate::diesel::RunQueryDsl;


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/chats/create_chat/", web::post().to(create_chat));
    config.route("/chats/edit_chat/{id}/", web::post().to(edit_chat));
    config.route("/chats/delete_chat/{id}/", web::get().to(delete_chat));
    config.route("/chats/restore_chat/{id}/", web::get().to(restore_chat));

    //config.route("/chats/send_page_message/{id}/", web::post().to(send_page_message));
    //config.route("/chats/send_message/{id}/", web::post().to(send_message));
    //config.route("/chats/send_voice_message/{id}/", web::post().to(send_voice_message));
    //config.route("/chats/save_draft_message/{id}/", web::post().to(save_draft_message));
    //config.route("/chats/edit_message/{id}/", web::post().to(edit_message));
    //config.route("/chats/delete_message/{id}/", web::get().to(delete_message));
    //config.route("/chats/restore_message/{id}/", web::get().to(restore_message));
    //config.route("/chats/favorite_messages/", web::get().to(favorite_messages));
    //config.route("/chats/unfavorite_messages/", web::get().to(unfavorite_messages));

    //config.route("/chats/add_attach_photo/", web::post().to(add_attach_photo));
    //config.route("/chats/{chat_id}/add_admin/{user_id}/", web::get().to(add_admin));
    //config.route("/chats/{chat_id}/remove_admin/{user_id}/", web::get().to(remove_admin));
    //config.route("/chats/{chat_id}/remove_member/{user_id}/", web::get().to(remove_member));

    //config.route("/chats/add_attach_photo/", web::post().to(add_attach_photo));
    //config.route("/chats/beep_off/{id}/", web::get().to(beep_off));
    //config.route("/chats/beep_on/{id}/", web::get().to(beep_on));
    //config.route("/chats/exit_user_from_user_chat/{id}/", web::get().to(exit_user_from_user_chat));
    //config.route("/chats/delete_support_chat/{id}/", web::get().to(delete_support_chat));
    //config.route("/chats/refresh_support_chat/{id}/", web::get().to(refresh_support_chat));

    //config.route("/chats/invite_members/{id}/", web::post().to(invite_members));
    //config.route("/chats/clean_messages/{id}/", web::get().to(clean_messages));

    //config.route("/chats/private/{id}/", web::post().to(private));
    //config.route("/chats/exclude_users/{id}/", web::post().to(chat_exclude_users));
    //config.route("/chats/include_users/{id}/", web::post().to(chat_include_users));

    //config.route("/chats/like_manager/{id}/", web::get().to(like_manager));
    //config.route("/chats/dislike_manager/{id}/", web::get().to(dislike_manager));
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatForm {
    pub name:             Option<String>,
    pub image:            Option<String>,
    pub description:      Option<String>,
    pub can_add_members:  String,
    pub can_fix_item:     String,
    pub can_mention:      String,
    pub can_add_admin:    String,
    pub can_add_design:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub users:            Option<String>,
    pub types:            i16,
    pub reactions:        Option<String>,
}

pub async fn chat_form (
    payload: &mut Multipart,
    owner_path: String,
    owner_id: String
) -> ChatForm {
    use crate::utils::UploadedFiles;

    let mut form: ChatForm = ChatForm {
        name:             None,
        image:            None,
        description:      None,
        can_add_members:  "".to_string(),
        can_fix_item:     "".to_string(),
        can_mention:      "".to_string(),
        can_add_admin:    "".to_string(),
        can_add_design:   "".to_string(),
        can_see_settings: "".to_string(),
        can_see_log:      "".to_string(),
        users:            None,
        types:            0,
        reactions:        None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            let file = UploadedFiles::new (
                owner_path.clone(),
                owner_id.to_string(),
                "chats".to_string(),
                _new_path.to_string(),
            );
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            if field.content_type().to_string() == "image/jpeg".to_string() {
                form.image = Some(file.path.clone().replace("./","/"));
            };
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = Some(data_string);
                    }
                    else if field.name() == "image" {
                        form.image = Some(data_string);
                    }
                    else if field.name() == "description" {
                        form.description = Some(data_string);
                    }
                    else if field.name() == "can_add_members" {
                        form.can_add_members = data_string;
                    }
                    else if field.name() == "can_fix_item" {
                        form.can_fix_item = data_string;
                    }
                    else if field.name() == "can_mention" {
                        form.can_mention = data_string;
                    }
                    else if field.name() == "can_add_admin" {
                        form.can_add_admin = data_string;
                    }
                    else if field.name() == "can_add_design" {
                        form.can_add_design = data_string;
                    }
                    else if field.name() == "can_see_settings" {
                        form.can_see_settings = data_string;
                    }
                    else if field.name() == "can_see_log" {
                        form.can_see_log = data_string;
                    }
                    else if field.name() == "users" {
                        form.users = Some(data_string);
                    }
                    else if field.name() == "types" {
                        let _int: i16 = data_string.parse().unwrap();
                        form.types = _int;
                    }
                    else if field.name() == "reactions" {
                        form.reactions = Some(data_string);
                    }
                }
            }
        }
    }
    form
}

pub async fn create_chat(session: Session, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = chat_form (
            payload.borrow_mut(),
            "users".to_string(),
            _request_user.id.to_string()
        ).await;
        let new_chat = Chat::create_group_chat (
            &_request_user,
            form.name,
            None,
            form.types,
            form.users,
        );

        let object_list: Vec<Message> = Vec::new();
        #[derive(TemplateOnce)]
        #[template(path = "desctop/chats/chat/detail/chat.stpl")]
        struct Template {
            chat:             Chat,
            request_user:     User,
            object_list:      Vec<Message>,
            next_page_number: i32,
            count:            i32,
            is_ajax:          bool,
        }
        let body = Template {
            chat:             new_chat,
            request_user:     _request_user,
            object_list:      object_list,
            next_page_number: 0,
            count:            0,
            is_ajax:          true,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn edit_chat(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let chat = get_chat(*_id);
        let form = chat_form (
            payload.borrow_mut(),
            "users".to_string(),
            _request_user.id.to_string()
        ).await;
        chat.edit_chat (
            form.name,
            form.image,
            form.description,
            form.reactions,
        );
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ок"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn delete_chat(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let chat = get_chat(*_id);
        let _request_user = get_request_user_data(&session);
        if _request_user.is_administrator_of_chat(chat.id) {
            chat.delete_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn restore_chat(session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {

        let chat = get_chat(*_id);
        let _request_user = get_request_user_data(&session);
        if _request_user.is_administrator_of_chat(chat.id) {
            chat.restore_item();
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
