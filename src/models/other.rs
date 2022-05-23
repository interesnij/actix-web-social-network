use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    phone_codes,
    custom_links,
    sticker_categories,
    stickers,
    smile_categories,
    smiles,
    establish_connection,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};


/////// PhoneCode //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:     i32,
    pub phone:  String,
    pub code:   i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode {
    pub phone:  String,
    pub code:   i32,
}

/////// CustomLink //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CustomLink {
    pub id:    i32,
    pub link:  String,
    pub owner: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="custom_links"]
pub struct NewCustomLink {
    pub link:  String,
    pub owner: i16,
}

/////// StickerCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StickerCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub user_id:     Option<i32>,
    pub description: Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="sticker_categories"]
pub struct NewStickerCategorie {
    pub name:        String,
    pub position:    i16,
    pub user_id:  Option<i32>,
    pub description: Option<String>,
}

/////// Stickers //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StickerCategorie)]
pub struct Sticker {
    pub id:                    i32,
    pub name:                  String,
    pub position:              i16,
    pub sticker_categorie_id: i32,
    pub image:                 String,
}
#[derive(Deserialize, Insertable)]
#[table_name="stickers"]
pub struct NewSticker {
    pub name:        String,
    pub position:    i16,
    pub sticker_categorie_id: i32,
    pub image:       String,
}

/////// SmileCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SmileCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="smile_categories"]
pub struct NewSmileCategorie {
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}
impl SmileCategorie {
    pub fn get_smiles(&self) -> Vec<Smile> {
        use crate::models::other::smiles::dsl::smiles;
        let _connection = establish_connection();

        return smiles
            .filter(schema::smiles::smile_categorie_id.eq(self.id))
            .order(schema::smiles::position.asc())
            .load::<Smile>(&_connection)
            .expect("E.");
    }
}

/////// Smiles //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(SmileCategorie)]
pub struct Smile {
    pub id:               i32,
    pub name:             String,
    pub position:         i16,
    pub smile_categorie_id: i32,
    pub image:            String,
}
#[derive(Deserialize, Insertable)]
#[table_name="smiles"]
pub struct NewSmile {
    pub name:        String,
    pub position:    i16,
    pub smile_categorie_id: i32,
    pub image:       String,
}
