use diesel_attendance_list::*;

fn main() {
    let mut datasource = DataSource::new();

    let event = datasource.get_event("computador").unwrap();
    let event_server = datasource.get_event_in_server("rust", "771935467951489084").unwrap();
    let inexistent_event = datasource.get_event_in_server("rust", "1123141512412423").is_none();
    datasource.insert_user_to_event("cleiton", "<@1414123123>", "rust");
    let users_event = datasource.get_users_event("rust").unwrap();

    println!("Event: {:#?}", event);
    println!("Event: {:#?}", event_server);
    println!("Inexistent event: {:#?}", inexistent_event);
    println!("Users from {}: {:#?}", "rust", users_event);
}