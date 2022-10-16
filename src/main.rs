use diesel_attendance_list::*;

fn main() {
    let mut repository = Repository::new();

    let event = repository.get_event("rust", "771935467951489084");
    let events_list = repository.get_all_events();

    match event {
        Some(ev) =>println!("Event: {:#?}\n\n", ev),
        None => println!("Nenhum evento encontrado"),
    }
    match events_list {
        Some(ev) =>println!("Event: {:#?}\n\n", ev),
        None => println!("Nenhum evento encontrado"),
    }
}