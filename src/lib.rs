use self::models::*;
use self::database::repository::{IRepository, PostgreSQLRepository};

mod database;
mod models;

pub struct DataSource {
    repository: Box<dyn IRepository>,
}

impl DataSource {
    pub fn new() -> Self {
        DataSource { repository: Box::new(PostgreSQLRepository::new()) }
    }

    pub fn get_all_events(&mut self) -> Option<Vec<Event>> {
        match self.repository.get_all_events() {
            Ok(events) => Some(events),
            Err(err) => {
                println!("Error getting events: {:?}", err);
                None
            }
        }
    }

    pub fn get_event(&mut self, name: &str) -> Option<Event> {
        match self.repository.get_event(name) {
            Ok(event) => Some(event),
            Err(err) => {
                println!("Error getting event with name ´{}´: {:?}", name, err);
                None
            }
        }
    }

    pub fn get_event_in_server(&mut self, name: &str, server_id: &str) -> Option<Event> {
        match self.repository.get_event_in_server(name, server_id) {
            Ok(event) => Some(event),
            Err(err) => {
                println!("Error getting event with name ´{}´ in server ´{}´: {:?}", name, server_id, err);
                None
            }
        }
    }
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
