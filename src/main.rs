use diesel_attendance_list::*;

fn main() {
    let mut datasource = DataSource::new();

    if let Some(event) = datasource.get_event("computador") {
        println!("Event: {:#?}", event)
    }
    println!("");
    if let Some(event) = datasource.get_event_in_server("rust", "771935467951489084") {
        println!("Event: {:#?}", event)
    }
    println!("");
    if let Some(event) = datasource.get_event_in_server("rust", "1123141512412423") {
        println!("Event: {:#?}", event)
    }
    println!("");
}