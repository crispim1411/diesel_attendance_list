use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::schema::*;
use self::models::*;

mod models;
mod schema;

pub trait IRepository {
    fn get_event(&mut self, name: &str, server_id: &str) -> Option<Event>;
    // fn insert_event(&mut self, name: &str, creator: &str, server_id: &str);
    // fn insert_user(&mut self, name: &str, mention: &str, event_name: &str, server_id: &str);
    fn get_all_events(&mut self) -> Option<Vec<Event>>;
}

pub struct Repository {
    connection: PgConnection
}

impl Repository {
    pub fn new() -> Self {
        Repository { connection: Repository::establish_connection() }   
    }

    fn establish_connection() -> PgConnection {
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to database"))
    }
}

impl IRepository for Repository {
    fn get_event(&mut self, name: &str, server_id: &str) -> Option<Event> {
        events::table
            .filter(events::name.like(format!("%{}%", name)).and(events::server_id.eq(server_id)))
            .first(&mut self.connection)
            .ok()
    }

    fn get_all_events(&mut self) -> Option<Vec<Event>> {
        events::table
            .load(&mut self.connection)
            .ok()
    }

    // fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) {
    //     todo!()
    // }

    // fn insert_user(&mut self, name: &str, mention: &str, event_name: &str, server_id: &str) {
    //     todo!()
    // }
}

// fn users_from_event(connection: &mut PgConnection) -> Vec<User> {
//     users::table
//         .inner_join(events::table)
//         .filter(events::name.like("%computador%"))
//         .select(users::all_columns)
//         .load(connection)
//         .expect("Error loading the event")
// }

// fn insert_event(connection: &mut PgConnection, new_event: &EventForm) -> usize {
//     diesel::insert_into(events::table)
//         .values(new_event)
//         .execute(connection)
//         .expect("Error saving the event")
// }
