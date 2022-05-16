use serde::{Deserialize, Serialize};
use std::str;
use std::{
    str,
    io::Write,
    fs::create_dir_all,
};

use actix_multipart::{Field, Multipart};
use futures::StreamExt;


#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    pub fn new (
        owner_path: String, // "users"
        owner_id:   String,    // user_id / community_id
        folder:     String, // "photos"
        filename:   String  // uuid
    ) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();

        let format_path = format!(
            "./media/{}/{}/{}/{}/{}/{}/{}/",
            owner_path.to_string(),
            owner_id.to_string(),
            folder.to_string(),
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
            filename.to_string(),
        );
        create_dir_all(format_path.clone()).unwrap();
        UploadedFiles {
            name: filename.to_string(),
            path: format_path,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostListForm {
    pub name: String,
    pub description: Option<String>,
    pub can_see_el: String,
    pub can_see_comment: String,
    pub create_el: String,
    pub create_comment: String,
    pub copy_el: String,
    pub can_see_el_users: Vec<i32>,
    pub can_see_comment_users: Vec<i32>,
    pub create_el_users: Vec<i32>,
    pub create_comment_users: Vec<i32>,
    pub copy_el_users: Vec<i32>,
}

pub async fn post_list_form(payload: &mut Multipart) -> PostListForm {
    let mut form: PostListForm = PostListForm {
        name: "".to_string(),
        description: None,
        can_see_el: "".to_string(),
        can_see_comment: "".to_string(),
        create_el: "".to_string(),
        create_comment: "".to_string(),
        copy_el: "".to_string(),
        can_see_el_users: Vec::new(),
        can_see_comment_users: Vec::new(),
        create_el_users: Vec::new(),
        create_comment_users: Vec::new(),
        copy_el_users: Vec::new(),
    };
    let _list = [
        "can_see_el_users",
        "can_see_comment_users",
        "create_el_users",
        "create_comment_users",
        "copy_el_users",
    ];

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if _list.contains(&field.name()) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    if field.name() == "can_see_el_users" {
                        form.can_see_el_users.push(_int);
                    }
                    else if field.name() == "can_see_comment_users" {
                        form.can_see_comment_users.push(_int);
                    }
                    else if field.name() == "create_el_users" {
                        form.create_el_users.push(_int);
                    }
                    else if field.name() == "create_comment_users" {
                        form.create_comment_users.push(_int);
                    }
                    else if field.name() == "copy_el_users" {
                        form.copy_el_users.push(_int);
                    }
                }
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "description" {
                        form.description = Some(data_string)
                    }
                    else if field.name() == "can_see_el" {
                        form.can_see_el = data_string
                    }
                    else if field.name() == "can_see_comment" {
                        form.can_see_comment = data_string
                    }
                    else if field.name() == "create_el" {
                        form.create_el = data_string
                    }
                    else if field.name() == "create_comment" {
                        form.create_comment = data_string
                    }
                    else if field.name() == "copy_el" {
                        form.copy_el = data_string
                    }
                }
            }
        }
    }
    form
}
