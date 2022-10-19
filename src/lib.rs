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

    pub fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) {
        match self.repository.insert_event(name, creator, server_id) {
            Ok(rows_inserted) => println!("Linhas inseridas: {}", rows_inserted),
            Err(err) => println!("Não foi possível inserir o evento: {}", err)
        }
    }

    pub fn insert_user_to_event(&mut self, name: &str, mention: &str, event_name: &str) {
        match self.repository.insert_user_to_event(name, mention, event_name) {
            Ok(rows_inserted) => println!("Linhas inseridas: {}", rows_inserted),
            Err(err) => println!(
                "Não foi possível inserir o usuário ´{}´ no evento ´{}´: {}", mention, event_name, err),
        }
    }

    pub fn get_users_event(&mut self, event_name: &str) -> Option<Vec<User>> {
        match self.repository.get_users_event(event_name) {
            Ok(users) => Some(users),
            Err(err) => {
                println!("Não foi possível listar os usuários do evento ´{}´: {}", event_name, err);
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_all_events_test() {
        let mut datasource = DataSource::new();
        let result = datasource.get_all_events();
        assert!(result.is_some());
        assert!(result.unwrap().len() > 0);
    }

    #[test]
    fn get_event_test() {
        let mut datasource = DataSource::new();
        let result = datasource.get_event("rust");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, String::from("evento rust cargo"));
    }
}