# Rust ORM
Teste utilizando o ORM Diesel para Rust para estabelecer conexão com base de dados PostgreSQL. Neste estudo foi utilizado a mesma do projeto de bot para discord [Attendance List Bot](https://github.com/crispim1411/attendance_list).

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

## Repository Pattern
O objeto de conexão é armazenado numa struct PostgreSQLRepository e instanciada por outra struct DataSource que por sua vez espera uma implementação da interface IRepository. O código final possui baixo acoplamento com a forma que a conexão a base de dados foi implementada. 

```rust
pub struct PostgreSQLRepository {
    connection: PgConnection
}

pub struct DataSource {
    repository: Box<dyn IRepository>,
}

pub trait IRepository {
    fn get_all_events(&mut self) -> Result<Vec<Event>, DieselError>;
    fn get_event(&mut self, name: &str) -> Result<Event, DieselError>;
    fn get_event_in_server(&mut self, name: &str, server_id: &str) -> Result<Event, DieselError>;
    ...
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
Função interna do repositório
```rust
fn get_event(&mut self, name: &str) -> Result<Event, DieselError> {
    events::table
        .filter(events::name.like(format!("%{}%", name)))
        .first(&mut self.connection)
}
```

Função de interface ao repositório
```rust
pub fn get_event(&mut self, name: &str) -> Option<Event> {
    match self.repository.get_event(name) {
        Ok(event) => Some(event),
        Err(err) => {
            println!("Error getting event with name ´{}´: {:?}", name, err);
            None
        }
    }
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
Função do repositório
```rust
fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) -> Result<usize, DieselError> {  
    let new_event = EventForm { name, creator, server_id };
    diesel::insert_into(events::table)
        .values(new_event)
        .execute(&mut self.connection)
}
```

Função de interface ao repositório
```rust
pub fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) {
    match self.repository.insert_event(name, creator, server_id) {
        Ok(rows_inserted) => println!("Linhas inseridas: {}", rows_inserted),
        Err(err) => println!("Não foi possível inserir o evento: {}", err)
    }
}
```

## Remoção
Função do repositório
```rust
fn delete_event(&mut self, event_id: &i32) -> Result<usize, DieselError> {
    self.remove_all_users_from_event(event_id)?;

    diesel::delete(events::table)
        .filter(events::id.eq(event_id))
        .execute(&mut self.connection)
}
```

Função de interface ao repositório
```rust
pub fn delete_event(&mut self, event_id: &i32) {
    match self.repository.delete_event(event_id) {
        Ok(rows_deleted) => println!("Eventos removidos: {}", rows_deleted),
        Err(err) => println!(
            "Não foi possível remover o evento ´{}´: {}", event_id, err),
    }
}
```
