pub mod pages;

use actix_web::web::ServiceConfig;

pub use self::{
    pages::*,
};

pub fn survey_routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(surveys_urls)
    ;
}
