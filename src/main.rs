//! vocabulary_trainer is a Rust mini project that let's you
//! store and access vocabulary sets.
//! All data that is collected is stored in sqlite databases.
//! The database backend is found in the db module in src/db/

//use macroquad::prelude::*;
use rusqlite::{Connection, Result};

mod db;
use db::vocab_table::VocabTable;

mod utils;


fn main() -> Result<()> {
    let conn = Connection::open("voci.db")?;
    let mut main_table = VocabTable::open_or_create(&conn, "main_voci_table")?;
    // main_table.insert("s1", "d1")?;
    // main_table.insert("s2", "d2")?;
    main_table.print_self()?;
    Ok(())
}


