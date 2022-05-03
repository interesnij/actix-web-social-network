use actix_web::{
    error::InternalError,
    http::StatusCode,
    HttpResponse,
};
use sailfish::TemplateOnce;
use crate::models::{User, PostList};


pub fn close_item(text: String) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "base_block/close/close_item.stpl")]
    struct Template {
        text: String,
    }
    let body = Template {
        text:  text,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}
