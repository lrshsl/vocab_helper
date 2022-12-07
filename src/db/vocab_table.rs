use rusqlite::{Error, Connection, Result};
use crate::utils::{Pair, VocabSet};


/// Query that's used to create a vocabulary table with sqlite
const CREATE_TABLE1_QUERY: &str = "
create table if not exists main_voci_table (
    id integer primary key,
    key text not null unique,
    definition text not null
);";


/// Struct for creating and handeling vocabulary sets
/// All entries are stored in a sqlite database with the
/// same name as in 'self.table_name'
pub struct VocabTable<'a> {
    conn: &'a Connection,
    table_name: String,
}

impl<'a> VocabTable<'a> {

    /// Opens a table with the name 'table_name'. If it does not exist
    /// yet, the table is created.
    pub fn open_or_create(conn: &'a Connection, table_name: &str) -> Result<Self> {

        // Create the table if necessairy
        conn.execute(CREATE_TABLE1_QUERY, [])?;

        // Return instance of VocabTable
        Ok(Self {
            conn,
            table_name: table_name.to_owned(),
        })
    }

    /// Inserts a pair into the table.
    /// Ignores UNIQUE-Errors which gets thrown by sqlite
    /// when trying to insert a key which is already in the table
    pub fn insert(&mut self, key: &str, definition: &str) -> Result<usize> {
        match self.conn.execute("INSERT INTO main_voci_table (key, definition) VALUES (?1, ?2)", [key, definition]) {
            Err(err) => match err {
                Error::SqliteFailure(e, _) => if e.extended_code == 2067 { Ok(0) } else { Err(err) }
                _ => Err(err)
            }
            some => some
        }
    }

    /// Reads all key-definition pairs from the table
    pub fn get_entries(&'a self) -> Result<VocabSet> {

        // Prepare the query
        let query_str = format!["select key, definition from {}", &self.table_name];
        let mut stmt = self.conn.prepare(&query_str)?;

        // Execute it and convert every element to a 'Pair'
        let pair_iter = stmt.query_map([], |row| { Ok(Pair{
                key: row.get(0)?,
                definition: row.get(1)?
            })
        })?;

        // Collect the iterator into a vector 'Vec' and return it
        pair_iter.collect::<Result<VocabSet>>()
    }
    
    /// Prints every element in order (for debugging)
    pub fn print_self(&self) -> Result<()> {
        
        // This loop just unpacks the elements in-place (while iterating)
        for (i, (key, def)) in self.get_entries()?
                    .into_iter()
                    .map(|e| (e.key, e.definition))
                    .enumerate() {
            println!["{}) {}: {}", i, key, def]
        }

        // This function has to return a 'Result',
        // when nothing goes wrong (at the question mark '?' operator above),
        // everything is 'Ok()'
        Ok(())
    }
}


