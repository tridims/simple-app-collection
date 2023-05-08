use postgres::Error as PostgresError;
use postgres::{Client, NoTls}; // for creating connection without tls

// Initiate and return connection to database
pub fn get_connection(db_url: &str) -> Result<Client, PostgresError> {
    let client = Client::connect(db_url, NoTls)?;
    Ok(client)
}

// db setup
pub fn set_database(db_url: &str) -> Result<(), PostgresError> {
    let mut client = Client::connect(db_url, NoTls)?;
    client.batch_execute(
        "
      CREATE TABLE IF NOT EXISTS notes (
          id SERIAL PRIMARY KEY,
          title VARCHAR NOT NULL,
          body VARCHAR NOT NULL
      )
      ",
    )?;
    Ok(())
}
