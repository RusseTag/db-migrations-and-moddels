use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::admins)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::category)]
pub struct Category {
    pub id: i32,
    pub category_name: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::orders)]
pub struct Order{
    pub id : i32,
    pub product_id : i32,
    pub adress: String,
    pub postal_number: i32,
    pub phone: i32,
    pub email: String,
    pub price: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::products)]
pub struct Product{
    pub id : i32,
    pub name : String,
    pub aliexpress_link : String,
    pub cost_price : i32,
    pub sell_price : i32,
    pub category_id : i32,
}



