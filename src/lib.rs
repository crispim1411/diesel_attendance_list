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

    pub fn get_all_events(&mut self) -> Vec<Event> {
        match self.repository.get_all_events() {
            Ok(events) => { return events },
            Err(err) => { println!("Error getting events: {:?}", err) },
        };
        vec![]
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

    pub fn get_users_event(&mut self, event_id: &i32) -> Vec<User> {
        match self.repository.get_users_event(event_id) {
            Ok(users) => { return users },
            Err(err) => {
                println!("Não foi possível listar os usuários do evento ´{}´: {}", event_id, err);
            }
        }
        vec![]
    }

    pub fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) {
        match self.repository.insert_event(name, creator, server_id) {
            Ok(rows_inserted) => println!("Linhas inseridas: {}", rows_inserted),
            Err(err) => println!("Não foi possível inserir o evento: {}", err)
        }
    }

    pub fn insert_user_to_event(&mut self, name: &str, mention: &str, event_id: &i32) {
        match self.repository.insert_user_to_event(name, mention, event_id) {
            Ok(rows_inserted) => println!("Usuários inseridos: {}", rows_inserted),
            Err(err) => println!(
                "Não foi possível inserir o usuário ´{}´ no evento ´{}´: {}", mention, event_id, err),
        }
    }

    pub fn delete_event(&mut self, event_id: &i32) {
        match self.repository.delete_event(event_id) {
            Ok(rows_deleted) => println!("Eventos removidos: {}", rows_deleted),
            Err(err) => println!(
                "Não foi possível remover o evento ´{}´: {}", event_id, err),
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
        assert!(result.len() > 0);
    }

    #[test]
    fn get_event_test() {
        let mut datasource = DataSource::new();
        let result = datasource.get_event("rust");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, String::from("evento rust cargo"));
    }

    #[test]
    fn insert_delete_event() {
        let mut datasource = DataSource::new();
        datasource.insert_event("evento teste lib insert", "test rust", "<@1234>");
        let event = datasource.get_event("evento teste lib insert").unwrap();
        datasource.delete_event(&event.id);
        let event_removed = datasource.get_event("evento teste lib insert");
        assert!(event_removed.is_none());
    }
}