# Rust ORM
Teste utilizando o ORM Diesel para Rust para estabelecer conexão com base de dados PostgreSQL. O banco utilizado é o mesmo do projeto de bot para discord [Attendance List Bot](https://github.com/).

# Configuração
Adicionar bibliotecas ao projeto

    cargo add dotenv
    cargo add diesel --features postgres,chrono

Instalar o Diesel CLI (PostgreSQL já instalado)

    cargo install diesel_cli --no-default-features --features postgres,chrono

Salvar a string de conexão num arquivo .env

    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

Chamar o diesel CLI, após isso teremos os esquemas da base de dados salvos em src/schema.rs

    diesel setup

# Codificando

## Conexão
```rust
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
```

## Leitura
Objeto DAO para leitura dos objetos SQL 

```rust
#[derive(Queryable)]
struct Event {
    pub id: i32,
    pub name: String,
    pub creator: String,
    pub server_id: String,
    pub date_created: NaiveDate,
    pub expiration: i32,
}
```
Função para obter todos os eventos cadastrados
```rust
fn get_all_events(connection: &mut PgConnection) -> Vec<Event> {
    events::table
        .load(connection)
        .expect("Error loading events")
} 
```

## Inserção
Objeto DAO para escrita na base de dados
```rust
#[derive(Insertable)]
#[diesel(table_name = events)]
struct EventForm<'a> {
    name: &'a str,
    creator: &'a str,
    server_id: &'a str,
}
```
Função para cadastrar um novo evento
```rust
fn insert_event(connection: &mut PgConnection, new_event: &EventForm) -> usize {
    diesel::insert_into(events::table)
        .values(new_event)
        .execute(connection)
        .expect("Error saving the event")
}
```