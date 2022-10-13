use diesel::prelude::*;
use diesel_demo::*;
use self::schema::*;
use self::models::*;

fn main() {
    let connection = &mut establish_connection();

    let events_list = get_events(connection);
    let users_list = get_users(connection);
    let result_join = users_from_event(connection);

    println!("Users: {:#?}\n\n", users_list);
    println!("Events: {:#?}\n\n", events_list);
    println!("result: {:#?}", result_join);
}

fn get_users(connection: &mut PgConnection) -> Vec<User> {
    users::table
        .limit(5)
        .load(connection)
        .expect("Error loading users")
}

fn get_events(connection: &mut PgConnection) -> Vec<Event> {
    events::table
        .limit(5)
        .load(connection)
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
