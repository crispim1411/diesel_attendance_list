use chrono::NaiveDate;
use diesel::prelude::*;
use crate::database::schema::*;

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

#[derive(Insertable)]
#[diesel(table_name=events)]
pub struct EventForm<'a> {
    pub name: &'a str,
    pub creator: &'a str,
    pub server_id: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct UserForm<'a> {
    pub name: &'a str,
    pub mention: &'a str,
    pub event_id: &'a i32,
}