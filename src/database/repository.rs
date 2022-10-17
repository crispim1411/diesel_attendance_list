use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenv::dotenv;
use std::env;

use super::schema::*;
use crate::models::*;

pub trait IRepository {
    fn get_all_events(&mut self) -> Result<Vec<Event>, DieselError>;
    fn get_event(&mut self, name: &str) -> Result<Event, DieselError>;
    fn get_event_in_server(&mut self, name: &str, server_id: &str) -> Result<Event, DieselError>;
    // fn insert_event(&mut self, name: &str, creator: &str, server_id: &str);
    // fn insert_user(&mut self, name: &str, mention: &str, event_name: &str, server_id: &str);
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
}