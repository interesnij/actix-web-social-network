use crate::schema::{
    phone_codes,
    custom_links,
    sticker_categories,
    stickers,
    smile_categories,
    smiles,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::User;


/////// PhoneCode //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:     i32,
    pub phone:  i64,
    pub code:   i32,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode {
    pub phone:  i64,
    pub code:   i32,
}

/////// CustomLink //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CustomLink {
    pub id:   i32,
    pub link: String,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="custom_links"]
pub struct NewCustomLink {
    pub link: String,
}

/////// StickerCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StickerCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i32,
    pub creator_id:  Option<i32>,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="sticker_categories"]
pub struct NewStickerCategorie {
    pub name:        String,
    pub position:    i32,
    pub creator_id:  Option<i32>,
    pub description: Option<String>,
}

/////// Stickers //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StickerCategorie)]
pub struct Sticker {
    pub id:                    i32,
    pub name:                  String,
    pub position:              i32,
    pub sticker_categorie_id: i32,
    pub image:                 String,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stickers"]
pub struct NewSticker {
    pub name:        String,
    pub position:    i32,
    pub sticker_categorie_id: i32,
    pub image:       String,
}

/////// SmileCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SmileCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i32,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="smile_categories"]
pub struct NewSmileCategorie {
    pub name:        String,
    pub position:    i32,
    pub description: Option<String>,
}

/////// Smiles //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(SmileCategorie)]
pub struct Smile {
    pub id:               i32,
    pub name:             String,
    pub position:         i32,
    pub smile_categorie_id: i32,
    pub image:            String,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="smiles"]
pub struct NewSmile {
    pub name:        String,
    pub position:    i32,
    pub smile_categorie_id: i32,
    pub image:       String,
}
