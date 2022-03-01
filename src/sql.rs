use rusqlite::{Connection, Result, params, Error};
use ::std::collections::HashMap;

/// start or cli notes app from adam berg, this mas a main() before.
pub fn create_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("bills.db")?;
    //println!("creating db...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bills (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL UNIQUE,
                  amount          INTEGER NOT NULL
                  )",
        [],
    )?;
    //println!("db created or existing...");
    Ok(())
}


pub fn load_db(all_bills: &mut HashMap<String, f64>) -> Result<(), Error> {
    //println!("Connecting to db to load...");
    let conn = Connection::open("bills.db")?;
    //println!("Connected to db...");
    let mut stmt = conn.prepare("SELECT name, amount FROM bills")?;
    println!("loading DB...");

    let res = stmt.query_map([], |row| {
        let name = row.get(0)?;
        //println!("let name = ran...");
        let amount = row.get(1)?;
        //println!("let amount = ran...");
        all_bills.insert(name, amount);
        Ok(())
    })?;

// wtf !!! must use res or sql query doesnt work!
    for r in res {
        let _ = r;
    }

    Ok(())
}


pub fn save_db(all_bills: &mut HashMap<String, f64>) -> Result<(), Error> {
    println!("Saving db...");
// add error control here....>
    let conn = Connection::open("bills.db")?;
    for (name, amount) in all_bills {
        let amount = *amount; //was .clone()
        conn.execute(
            "INSERT INTO bills (name, amount) VALUES (?1, ?2)",
            params![name, amount],
        )?;
        //println!("Current Bill Name: {},  Amount{}", name, amount)
    }
    Ok(())
}
