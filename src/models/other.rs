use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    phone_codes,
    custom_links,
    sticker_categories,
    stickers,
    smile_categories,
    smiles,
    reactions,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

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
    pub avatar:      Option<String>,
}

impl StickerCategorie {
    pub fn create_category(name: String, position: i16,
        user_id: Option<i32>, description: Option<String>,
        avatar: Option<String>) -> StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:        name,
            position:    position,
            user_id:     user_id,
            description: description,
            avatar:      avatar,
        };
        let new_cat = diesel::insert_into(schema::sticker_categories::table)
            .values(&new_form)
            .get_result::<StickerCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        user_id: Option<i32>, description: Option<String>,
        avatar: Option<String>) -> &StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:        name,
            position:    position,
            user_id:     user_id,
            description: description,
            avatar:      avatar,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<StickerCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn get_image(&self) -> &str {
        if self.avatar.is_some() {
            return self.avatar.as_deref().unwrap();
        }
        else {
            return "/static/images/no_img/smile.gif";
        }
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="sticker_categories"]
pub struct NewStickerCategorie {
    pub name:        String,
    pub position:    i16,
    pub user_id:     Option<i32>,
    pub description: Option<String>,
    pub avatar:      Option<String>,
}

/////// Stickers //////
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StickerCategorie)]
pub struct Sticker {
    pub id:                   i32,
    pub name:                 String,
    pub position:             i16,
    pub sticker_categorie_id: i32,
    pub image:                String,
}

impl Sticker {
    pub fn create_sticker(name: String, position: i16,
        sticker_categorie_id: i32, image: String) -> Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:                 name,
            position:             position,
            sticker_categorie_id: sticker_categorie_id,
            image:                image,
        };
        let new_sticker = diesel::insert_into(schema::stickers::table)
            .values(&new_form)
            .get_result::<Sticker>(&_connection)
            .expect("Error.");
        return new_sticker;
    }
    pub fn edit_sticker(&self, name: String, position: i16,
        sticker_categorie_id: i32, image: String) -> &Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:                 name,
            position:             position,
            sticker_categorie_id: sticker_categorie_id,
            image:                image,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Sticker>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="stickers"]
pub struct NewSticker {
    pub name:                 String,
    pub position:             i16,
    pub sticker_categorie_id: i32,
    pub image:                String,
}

/////// SmileCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SmileCategorie {
    pub id:          i32,
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
    pub fn create_category(name: String, position: i16,
        description: Option<String>) -> SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        let new_cat = diesel::insert_into(schema::smile_categories::table)
            .values(&new_form)
            .get_result::<SmileCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        description: Option<String>) -> &SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<SmileCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smile_categories"]
pub struct NewSmileCategorie {
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
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

impl Smile {
    pub fn create_smile(name: String, position: i16,
        smile_categorie_id: i32, image: String) -> Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:               name,
            position:           position,
            smile_categorie_id: smile_categorie_id,
            image:              image,
        };
        let new_smile = diesel::insert_into(schema::smiles::table)
            .values(&new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return new_smile;
    }
    pub fn edit_smile(&self, name: String, position: i16,
        smile_categorie_id: i32, image: String) -> &Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:               name,
            position:           position,
            smile_categorie_id: smile_categorie_id,
            image:              image,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smiles"]
pub struct NewSmile {
    pub name:        String,
    pub position:    i16,
    pub smile_categorie_id: i32,
    pub image:       String,
}

/////// Reactions //////

///// Типы реакций
    // 1 thumbs_up     палец вверх
    // 2 thumbs_down   палец вниз
    // 3 red_heart     красное сердце
    // 4 fire          огонь
    // 5 love_face     лицо с поцелуями
    // 6 clapping      апплодисменты
    // 7 beaming       смеющееся лицо
    // 8 thinking      размышляющее лицо
    // 9 exploding     взрывающийся мозг
    // 10 screaming    ужасающееся лицо
    // 11 evil         очень злое лицо
    // 12 crying       плачущее лицо
    // 13 party        вечеринка
    // 14 star_face    звезды в глазах
    // 15 vomiting     рвота на лице
    // 16 pile_of_poo  куча какашек

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Reaction {
    pub id:        i32,
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}

impl Reaction {
    pub fn create_reaction(types: i16, image: String, gif: String,
        name: String, is_active: bool, position: i16) -> Reaction {
        let _connection = establish_connection();
        let new_form = NewReaction {
            types:     types,
            image:     image,
            gif:       gif,
            name:      name,
            is_active: is_active,
            position:  position,
        };
        let new_reaction = diesel::insert_into(schema::reactions::table)
            .values(&new_form)
            .get_result::<Reaction>(&_connection)
            .expect("Error.");
        return new_reaction;
    }
    pub fn edit_reaction(&self, types: i16, image: String, gif: String,
        name: String, is_active: bool, position: i16) -> &Reaction {
        let _connection = establish_connection();
        let new_form = NewReaction {
            types:     types,
            image:     image,
            gif:       gif,
            name:      name,
            is_active: is_active,
            position:  position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Reaction>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="reactions"]
pub struct NewReaction {
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}
