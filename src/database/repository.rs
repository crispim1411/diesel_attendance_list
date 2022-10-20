use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenv::dotenv;
use std::env;

use super::schema::*;
use crate::models::*;

pub trait IRepository {
    // gets
    fn get_all_events(&mut self) -> Result<Vec<Event>, DieselError>;
    fn get_event(&mut self, name: &str) -> Result<Event, DieselError>;
    fn get_event_in_server(&mut self, name: &str, server_id: &str) -> Result<Event, DieselError>;
    fn get_users_event(&mut self, event_name: &str) -> Result<Vec<User>, DieselError>;
    fn check_user_in_event(&mut self, mention: &str, event_id: &i32) -> Result<bool, DieselError>;
    // inserts
    fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) -> Result<usize, DieselError>;
    fn insert_user_to_event(&mut self, name: &str, mention: &str, event_name: &str) -> Result<usize, DieselError>;
}

pub struct PostgreSQLRepository {
    connection: PgConnection
}

impl PostgreSQLRepository {
    pub fn new() -> Self {
        PostgreSQLRepository { connection: PostgreSQLRepository::establish_connection() }   
    }

    fn establish_connection() -> PgConnection {
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to database"))
    }
}

impl IRepository for PostgreSQLRepository {
    fn get_event(&mut self, name: &str) -> Result<Event, DieselError> {
        events::table
            .filter(events::name.like(format!("%{}%", name)))
            .first(&mut self.connection)
    }

    fn get_event_in_server(&mut self, name: &str, server_id: &str) -> Result<Event, DieselError> {
        events::table
            .filter(events::name.like(format!("%{}%", name))
                .and(events::server_id.eq(server_id)))
            .first(&mut self.connection)
    }

    fn get_all_events(&mut self) -> Result<Vec<Event>, DieselError> {
        events::table.load(&mut self.connection)
    }

    fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) -> Result<usize, DieselError> {  
        let new_event = EventForm { name, creator, server_id };
        diesel::insert_into(events::table)
            .values(new_event)
            .execute(&mut self.connection)
    }

    fn insert_user_to_event(&mut self, name: &str, mention: &str, event_name: &str) -> Result<usize, DieselError> {
        let event;
        match self.get_event(event_name) {
            Ok(ev) => event = ev,
            Err(err) => return Err(err),
        }
        let user_to_insert = UserForm { name, mention, event_id: &event.id };
        diesel::insert_into(users::table)
            .values(user_to_insert)
            .execute(&mut self.connection)
    }

    fn get_users_event(&mut self, event_name: &str) -> Result<Vec<User>, DieselError> {
        users::table
            .inner_join(events::table)
            .filter(events::name.like(format!("%{}%", event_name)))
            .select(users::all_columns)
            .load(&mut self.connection)
    }
}