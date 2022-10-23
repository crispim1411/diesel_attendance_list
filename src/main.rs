use diesel_attendance_list::*;

fn main() {
    let mut datasource = DataSource::new();

    let event = datasource.get_event("Criado via diesel 2").unwrap();    // let event_server = datasource.get_event_in_server("rust", "771935467951489084").unwrap();
    
    let users_event = datasource.get_users_event(&event.id);
    println!("Users from {}: {:#?}", event.name, users_event);

    datasource.insert_user_to_event("baph", "@baph132", &event.id);
    datasource.insert_user_to_event("beli", "@beli132", &event.id);

    let users_event = datasource.get_users_event(&event.id);
    println!("Users from {}: {:#?}", event.name, users_event);
    
    datasource.delete_event(&event.id);
}