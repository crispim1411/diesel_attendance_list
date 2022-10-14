use diesel::prelude::*;
use diesel_demo::*;
use self::schema::*;
use self::models::*;

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct EventForm<'a> {
    name: &'a str,
    creator: &'a str,
    server_id: &'a str,
}

fn main() {
    let connection = &mut establish_connection();

    let events_list = get_all_events(connection);
    let users_list = get_users(connection);
    let result_join = users_from_event(connection);
    let event = get_event(connection, "rust");

    println!("Users: {:#?}\n\n", users_list);
    println!("Events: {:#?}\n\n", events_list);
    println!("result: {:#?}\n\n", result_join);
    println!("Event: {:#?}\n\n", event);

    let new_event = EventForm {
        name: "Criado via diesel 2",
        creator: "<@!280092021786673152>",    
        server_id: "771935467951489084",
    };
    let result_save = insert_event(connection, &new_event);
    println!("{} events has been saved\n", result_save);
}

fn get_users(connection: &mut PgConnection) -> Vec<User> {
    users::table
        .limit(5)
        .load(connection)
        .expect("Error loading users")
}

fn get_all_events(connection: &mut PgConnection) -> Vec<Event> {
    events::table
        .limit(5)
        .load(connection)
        .expect("Error loading events")
}   

fn get_event(connection: &mut PgConnection, name: &str) -> Event {
    events::table
        .filter(events::name.like(format!("%{}%", name)))
        .first(connection)
        .expect("Error loading events")
}

fn users_from_event(connection: &mut PgConnection) -> Vec<User> {
    users::table
        .inner_join(events::table)
        .filter(events::name.like("%computador%"))
        .select(users::all_columns)
        .load(connection)
        .expect("Error loading the event")
}

fn insert_event(connection: &mut PgConnection, new_event: &EventForm) -> usize {
    diesel::insert_into(events::table)
        .values(new_event)
        .execute(connection)
        .expect("Error saving the event")
}
