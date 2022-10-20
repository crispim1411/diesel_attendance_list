# Rust ORM
Teste utilizando o ORM Diesel para Rust para estabelecer conexão com base de dados PostgreSQL. O banco utilizado é o mesmo do projeto de bot para discord [Attendance List Bot](https://github.com/crispim1411/attendance_list).

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
O objeto de conexão é armazenado numa struct PostgreSQLRepository e instanciada por outra struct DataSource que por sua vez espera um interface IRepository. O código final possui baixo acoplamento com qual base de dados está sendo utilizada.

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
Função para encontrar um evento através de uma palavra chave em seu atributo nome
```rust
fn get_event(&mut self, name: &str) -> Result<Event, DieselError> {
    events::table
        .filter(events::name.like(format!("%{}%", name)))
        .first(&mut self.connection)
}
```

Função intermediária com o núcleo que consume o dado, tratando a resposta dada pela base de dados
```rust
fn get_event(&mut self, name: &str) -> Option<Event> {
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
Função para cadastrar um novo evento
```rust
fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) -> Result<usize, DieselError> {  
    let new_event = EventForm { name, creator, server_id };
    diesel::insert_into(events::table)
        .values(new_event)
        .execute(&mut self.connection)
}
```

Função intermediária com o núcleo que consume o dado, tratando a resposta dada pela base de dados
```rust
fn insert_event(&mut self, name: &str, creator: &str, server_id: &str) {
    match self.repository.insert_event(name, creator, server_id) {
        Ok(rows_inserted) => println!("Linhas inseridas: {}", rows_inserted),
        Err(err) => println!("Não foi possível inserir o evento: {}", err)
    }
}
```

