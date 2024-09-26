use rusqlite::{Connection, Result};
use crate::models::model::Mahsulot;


pub fn mahsulotlar_jadvalini_yaratish(conn: &Connection) -> Result<()> {

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mahsulotlar (
        id INTEGER PRIMARY KEY,
        nomi TEXT NOT NULL,
        narxi REAL NOT NULL
        )",
        [],
    )?;
    Ok(())
}


pub async fn get_products() -> Result<Vec<Mahsulot>, rusqlite::Error> {
    let conn = Connection::open("products.db")?;
    let mut stmt = conn.prepare("SELECT id, nomi, narxi FROM mahsulotlar")?;
    let mahsulot_iter = stmt.query_map([], |row| {
        Ok(Mahsulot {
            id: row.get(0)?,
            nomi: row.get(1)?,
            narxi: row.get(2)?,
        })
    })?;

    let mut mahsulotlar = Vec::new();
    for mahsulot in mahsulot_iter {
        mahsulotlar.push(mahsulot?);
    }
    Ok(mahsulotlar)
}

pub async fn add_product_to_db(nomi: &str, narxi: u32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("products.db")?;
    conn.execute(
        "INSERT INTO mahsulotlar (nomi, narxi) VALUES (?1, ?2)",
        &[nomi, &narxi.to_string()],
    )?;
    Ok(())
}

pub async fn remove_product_from_db(id: u32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("products.db")?;
    conn.execute("DELETE FROM mahsulotlar WHERE id = ?1", &[&id.to_string()])?;
    Ok(())
}