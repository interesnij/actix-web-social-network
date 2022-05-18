use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    post,
    get,
    web::Json,
};
use std::borrow::BorrowMut;
use actix_multipart::Multipart;
use serde::{Serialize, Deserialize};
use crate::utils::{
    is_signed_in,
    establish_connection,
    get_request_user_data,
    JsonReactions,
};
use actix_session::Session;
use crate::diesel::RunQueryDsl;

pub fn progs_routes(config: &mut web::ServiceConfig) {
    //config.route("/edit_comment/", web::get().to(edit_comment));
}

pub fn get_type(req: HttpRequest) -> (bool, i32, String) {
    #[derive(Debug, Deserialize)]
    pub struct TypesParams {
        pub types: Option<String>,
    }
    let params_some = web::Query::<TypesParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.types.is_some() {
            let item = params.types.as_deref().unwrap();
            let pk: i32 = item[3..].parse().unwrap();
            let code = &item[..3].to_string();

            return (true, pk, code.to_string());
        }
        else {
            return (false, 0, "".to_string());
        }
    }
    else {
        return (false, 0, "".to_string());
    }
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

#[post("/users/progs/edit_comment/")]
pub async fn edit_comment(session: Session, req: HttpRequest, mut payload: Multipart) -> web::Json<JsonCommentResponse> {

    if is_signed_in(&session) {
        use crate::utils::comment_form;
        use crate::models::EditGoodComment;
        let _connection = establish_connection();

        let _request_user = get_request_user_data(session);
        let form = comment_form(payload.borrow_mut()).await;

        let (type_exists, comment_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonCommentResponse {
                content: None,
                attach:  None,
            })
        }
        else {
            if types == "pos".to_string() {
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
            else if types == "goo".to_string() {
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
            else if types == "pho".to_string() {
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
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                })
            }
            else if types == "vid".to_string() {
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

#[get("/users/progs/delete_comment/")]
pub async fn delete_comment(session: Session, req: HttpRequest) -> web::Json<JsonResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, comment_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonResponse {info: "Ошибка доступа".to_string()})
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                if item.get_list().is_user_can_create_comment(_request_user.id) && item.delete_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "vid".to_string() {
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

#[get("/users/progs/recover_comment/")]
pub async fn recover_comment(session: Session, req: HttpRequest) -> web::Json<JsonResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, comment_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonResponse {info: "Ошибка доступа".to_string()})
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                if item.restore_item() {
                    return Json(JsonResponse {info: "ок".to_string()})
                }
                else {
                    return Json(JsonResponse {info: "Ошибка доступа".to_string()})
                }
            }
            else if types == "vid".to_string() {
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

#[get("/users/progs/like_comment/")]
pub async fn like_comment(session: Session, req: HttpRequest) -> web::Json<JsonReactions> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, comment_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonReactions {
                like_count: 0,
                dislike_count: 0,
            })
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                item.send_like(_request_user.id)
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                item.send_like(_request_user.id)
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                item.send_like(_request_user.id)
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video_comment;

                let item = get_video_comment(comment_id);
                item.send_like(_request_user.id)
            }
            else {
                return Json(JsonReactions {
                    like_count: 0,
                    dislike_count: 0,
                })
            }
        }
    } else {
        return Json(JsonReactions {
            like_count: 0,
            dislike_count: 0,
        })
    }
}

#[get("/users/progs/dislike_comment/")]
pub async fn dislike_comment(session: Session, req: HttpRequest) -> web::Json<JsonReactions> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, comment_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonReactions {
                like_count: 0,
                dislike_count: 0,
            })
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post_comment;

                let item = get_post_comment(comment_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good_comment;

                let item = get_good_comment(comment_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo_comment;

                let item = get_photo_comment(comment_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video_comment;

                let item = get_video_comment(comment_id);
                item.send_dislike(_request_user.id)
            }
            else {
                return Json(JsonReactions {
                    like_count: 0,
                    dislike_count: 0,
                })
            }
        }
    } else {
        return Json(JsonReactions {
            like_count: 0,
            dislike_count: 0,
        })
    }
}


#[get("/users/progs/like_item/")]
pub async fn like_item(session: Session, req: HttpRequest) -> web::Json<JsonReactions> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, item_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonReactions {
                like_count: 0,
                dislike_count: 0,
            })
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post;

                let item = get_post(item_id);
                item.send_like(_request_user.id)
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good;

                let item = get_good(item_id);
                item.send_like(_request_user.id)
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo;

                let item = get_photo(item_id);
                item.send_like(_request_user.id)
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video;

                let item = get_video(item_id);
                item.send_like(_request_user.id)
            }
            else {
                return Json(JsonReactions {
                    like_count: 0,
                    dislike_count: 0,
                })
            }
        }
    } else {
        return Json(JsonReactions {
            like_count: 0,
            dislike_count: 0,
        })
    }
}

#[get("/users/progs/dislike_item/")]
pub async fn dislike_item(session: Session, req: HttpRequest) -> web::Json<JsonReactions> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(session);
        let (type_exists, item_id, types) = get_type(req);
        if type_exists == false {
            return Json(JsonReactions {
                like_count: 0,
                dislike_count: 0,
            })
        }
        else {
            if types == "pos".to_string() {
                use crate::utils::get_post;

                let item = get_post(item_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good;

                let item = get_good(item_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo;

                let item = get_photo(item_id);
                item.send_dislike(_request_user.id)
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video;

                let item = get_video(item_id);
                item.send_dislike(_request_user.id)
            }
            else {
                return Json(JsonReactions {
                    like_count: 0,
                    dislike_count: 0,
                })
            }
        }
    } else {
        return Json(JsonReactions {
            like_count: 0,
            dislike_count: 0,
        })
    }
}
