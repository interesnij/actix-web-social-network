use actix_web::{
    HttpRequest,
    web,
    web::Json,
};
use std::borrow::BorrowMut;
use actix_multipart::Multipart;
use serde::{Serialize, Deserialize};
use crate::utils::{
    is_signed_in,
    establish_connection,
    get_request_user_data,
    get_type,
    JsonItemReactions,
};
use actix_session::Session;
use crate::diesel::RunQueryDsl;

pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/users/progs/send_reaction/", web::get().to(send_reaction));
    config.route("/users/progs/delete_comment/", web::get().to(delete_comment));
    config.route("/users/progs/recover_comment/", web::get().to(recover_comment));
    config.route("/users/progs/edit_comment/", web::post().to(edit_comment));
}

#[derive(Deserialize, Serialize)]
pub struct JsonCommentResponse {
    pub content: Option<String>,
    pub attach:  Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
    pub info: String,
}

pub async fn edit_comment(session: Session, req: HttpRequest, mut payload: Multipart) -> web::Json<JsonCommentResponse> {

    if is_signed_in(&session) {
        use crate::utils::comment_form;
        let _connection = establish_connection();

        let _request_user = get_request_user_data(&session);
        let form = comment_form(payload.borrow_mut()).await;

        let (type_exists, comment_id, types) = get_type(&req);
        if type_exists == false {
            return Json(JsonCommentResponse {
                content: None,
                attach:  None,
            })
        }
        else {
            if types == "cpo".to_string() {
                use crate::utils::get_post_comment;
                use crate::models::{PostComment, EditPostComment};

                let edited_comment = EditPostComment {
                    content: form.content,
                    attach:  form.attach,
                };
                let item = get_post_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) {
                    diesel::update(&item)
                        .set(&edited_comment)
                        .get_result::<PostComment>(&_connection)
                        .expect("Error.");
                }
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                })
            }
            else if types == "cgo".to_string() {
                use crate::utils::get_good_comment;
                use crate::models::{GoodComment, EditGoodComment};

                let item = get_good_comment(comment_id);

                let edited_comment = EditGoodComment {
                    content: form.content,
                    attach:  form.attach,
                };
                if item.get_list().is_user_can_create_comment(_request_user.id) {
                    diesel::update(&item)
                        .set(&edited_comment)
                        .get_result::<GoodComment>(&_connection)
                        .expect("Error.");
                }
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                })
            }
            else if types == "cph".to_string() {
                use crate::utils::get_photo_comment;
                use crate::models::{PhotoComment, EditPhotoComment};

                let item = get_photo_comment(comment_id);

                let edited_comment = EditPhotoComment {
                    content: form.content,
                    attach:  form.attach,
                };
                if item.get_list().is_user_can_create_comment(_request_user.id) {
                    diesel::update(&item)
                        .set(&edited_comment)
                        .get_result::<PhotoComment>(&_connection)
                        .expect("Error.");
                }
                return Json(JsonCommentResponse {
                    content:  edited_comment.content,
                    attach:   edited_comment.attach,
                })
            }
            else if types == "cvi".to_string() {
                use crate::utils::get_video_comment;
                use crate::models::{VideoComment, EditVideoComment};

                let item = get_video_comment(comment_id);

                let edited_comment = EditVideoComment {
                    content: form.content,
                    attach:  form.attach,
                };

                if item.get_list().is_user_can_create_comment(_request_user.id) {
                    diesel::update(&item)
                        .set(&edited_comment)
                        .get_result::<VideoComment>(&_connection)
                        .expect("Error.");
                }
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                })
            }
            else {
                return Json(JsonCommentResponse {
                    content: None,
                    attach:  None,
                })
            }
        }
    } else {
        return Json(JsonCommentResponse {
            content: None,
            attach:  None,
        })
    }
}

pub async fn delete_comment(session: Session, req: HttpRequest) -> web::Json<JsonResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (type_exists, comment_id, types) = get_type(&req);
        if type_exists == false {
            return Json(JsonResponse {info: "Ошибка доступа".to_string()})
        }
        else {
            if types == "cpo".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cgo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cph".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cvi".to_string() {
                use crate::utils::get_video_comment;

                let item = get_video_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else {
                return Json(JsonResponse {info: "Ошибка доступа".to_string()})
            }
        }
    } else {
        return Json(JsonResponse {info: "Ошибка доступа".to_string()})
    }
}

pub async fn recover_comment(session: Session, req: HttpRequest) -> web::Json<JsonResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (type_exists, comment_id, types) = get_type(&req);
        if type_exists == false {
            return Json(JsonResponse {info: "Ошибка доступа".to_string()})
        }
        else {
            if types == "cpo".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cgo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cph".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "cvi".to_string() {
                use crate::utils::get_video_comment;

                let item = get_video_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else {
                return Json(JsonResponse {info: "Ошибка доступа".to_string()})
            }
        }
    } else {
        return Json(JsonResponse {info: "Ошибка доступа".to_string()})
    }
}

pub async fn send_reaction(session: Session, req: HttpRequest) -> web::Json<JsonItemReactions> {
    if is_signed_in(&session) {
        #[derive(Debug, Deserialize)]
        struct TypesParams {
            pub types:    Option<String>,
            pub reaction: Option<i16>,
        }
        let mut item_id: i32 = 0;
        let mut code = "".to_string();
        let mut reaction: i16 = 0;

        let params_some = web::Query::<TypesParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.types.is_some() {
                let item = params.types.as_deref().unwrap();
                item_id = item[3..].parse().unwrap();
                code = item[..3].to_string();
            }
            if params.reaction.is_some() {
                reaction = params.reaction.unwrap();
            }
        }

        let _request_user = get_request_user_data(&session);

        if code == "cpo".to_string() {
            use crate::utils::get_post_comment;

            let item = get_post_comment(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "cgo".to_string() {
            use crate::utils::get_good_comment;

            let item = get_good_comment(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "cph".to_string() {
            use crate::utils::get_photo_comment;

            let item = get_photo_comment(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "cvi".to_string() {
            use crate::utils::get_video_comment;

            let item = get_video_comment(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "pos".to_string() {
            use crate::utils::get_post;

            let item = get_post(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "goo".to_string() {
            use crate::utils::get_good;

            let item = get_good(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "pho".to_string() {
            use crate::utils::get_photo;

            let item = get_photo(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else if code == "vid".to_string() {
            use crate::utils::get_video;

            let item = get_video(item_id);
            item.send_reaction(_request_user.id, reaction)
        }
        else {
            let data: Vec<i32> = Vec::new();
            return Json(JsonItemReactions {data});
        }
    }
    else {
        let data: Vec<i32> = Vec::new();
        return Json(JsonItemReactions {data});
    }
}
