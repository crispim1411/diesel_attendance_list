use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub creator: String,
    pub server_id: String,
    pub date_created: NaiveDate,
    pub expiration: i32,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mention: String,
    pub event_id: i32,
}