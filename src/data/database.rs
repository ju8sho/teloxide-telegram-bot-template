use rusqlite::{Connection, params, Result};
use crate::models::model::Todo;

pub fn create_table() -> Result<Connection> {
    let conn = Connection::open("data.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
        todo_id INTEGER PRIMARY KEY
        todo_name TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn add_data(db: &Connection, data: &Todo) -> Result<()> {
    let mut stmt = db.prepare(
        "INSERT OR IGNORE INTO todos (todo_id, todo_name) VALUES (?1, ?2)"
    )?;
    stmt.execute(params![data.todo_id, data.todo_name])?;
    Ok(())
}

pub fn get_data(db: &Connection) -> Result<Vec<Todo>>{
    let mut stmt = db.prepare("SELECT * FROM todos")?;
    let list = stmt.query_map([],|row| {
       Ok(Todo {
           todo_id: row.get(0)?,
           todo_name: row.get(1)?,
       })
    })?;

    let response: Result<Vec<Todo>> = list.collect();
    response
}