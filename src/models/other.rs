use crate::schema::{phone_codes, custom_links};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:     i32,
    pub phone:  String,
    pub code:   i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode {
    pub phone:  String,
    pub code:   i32,
}

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
