use actix_web::{
    HttpResponse,
    HttpRequest,
    web,
    //web::Json,
    //error::InternalError,
    //http::StatusCode,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    //get_community,
    //get_community_permission,
    //get_user_permission,
};
use actix_session::Session;
//use sailfish::TemplateOnce;
use crate::models::{
    Community,
    //User
};
//use crate::diesel::RunQueryDsl;
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use std::{
    str,
    //io::Write,
    //borrow::BorrowMut,
};
use serde::{Serialize, Deserialize};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/communities/create_community/", web::post().to(create_community));
}

pub async fn create_community(session: Session, req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        use crate::views::community_page;

        #[derive(Deserialize, Serialize, Debug)]
        pub struct CommunityForm {
            pub name:        String,
            pub category_id: i32,
            pub types:       i16,
        }

        let mut form: CommunityForm = CommunityForm {
            name:        "".to_string(),
            category_id: 0,
            types:       0,
        };

        while let Some(item) = payload.next().await {
            let mut field: Field = item.expect("split_payload err");
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "category_id" {
                        let _int: i32 = data_string.parse().unwrap();
                        form.category_id = _int;
                    }
                    else if field.name() == "types" {
                        let _int: i16 = data_string.parse().unwrap();
                        form.types = _int;
                    }
                    else if field.name() == "name" {
                        form.name = data_string;
                    }
                }
            }
        }

        let _request_user = get_request_user_data(&session);
        let new_community = Community::create_community (
            form.name,
            form.category_id,
            &_request_user,
            form.types,
        );
        return community_page(session, req, "/public".to_owned() + &new_community.to_string()).await;

    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
