use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{
    is_signed_in,
    establish_connection,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use crate::models::User;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/edit_comment/", web::get().to(edit_comment));
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
            let item = params.types.unwrap();
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

#[derive(Deserialize)]
pub struct JsonCommentResponse {
    pub content: Option<String>,
    pub attach:  Option<String>,
}
pub async fn edit_comment(session: Session, req: HttpRequest, mut payload: Multipart) -> Json<JsonCommentResponse> {
    use actix_web::web::Json;

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
            });
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

                diesel::update(&item)
                    .set(edited_comment)
                    .get_result::<PostComment>(&_connection)
                    .expect("Error.");
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                });
            }
            else if types == "goo".to_string() {
                use crate::utils::get_good_comment;
                use crate::models::{GoodComment, EditGoodComment};

                let item = get_good_comment(comment_id);

                let edited_comment = EditGoodComment {
                    content: form.content,
                    attach:  form.attach,
                };

                diesel::update(&item)
                    .set(edited_comment)
                    .get_result::<GoodComment>(&_connection)
                    .expect("Error.");
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                });
            }
            else if types == "pho".to_string() {
                use crate::utils::get_photo_comment;
                use crate::models::{PhotoComment, EditPhotoComment};

                let item = get_photo_comment(comment_id);

                let edited_comment = EditPhotoComment {
                    content: form.content,
                    attach:  form.attach,
                };

                diesel::update(&item)
                    .set(edited_comment)
                    .get_result::<PhotoComment>(&_connection)
                    .expect("Error.");
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                });
            }
            else if types == "vid".to_string() {
                use crate::utils::get_video_comment;
                use crate::models::{VideoComment, EditVideoComment};

                let item = get_video_comment(comment_id);

                let edited_comment = EditVideoComment {
                    content: form.content,
                    attach:  form.attach,
                };

                diesel::update(&item)
                    .set(edited_comment)
                    .get_result::<VideoComment>(&_connection)
                    .expect("Error.");
                return Json(JsonCommentResponse {
                    content: edited_comment.content,
                    attach:  edited_comment.attach,
                });
            }
            else {
                return Json(JsonCommentResponse {
                    content: None,
                    attach:  None,
                });
            }
        }
    } else {
        return Json(JsonCommentResponse {
            content: None,
            attach:  None,
        });
    }
}
