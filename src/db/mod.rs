use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Db {
    connection: Arc<Mutex<Connection>>,
}

#[derive(Serialize, Deserialize)]
pub struct Voter {
    pub id: i64,
    pub name: String,
}

pub struct NewVoter {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub sonarr: SettingsConfig,
    pub radarr: SettingsConfig,
    pub credentials: SettingsCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsCredentials {
    pub username: String,
    pub password: String,
}

impl Db {
    pub fn new(connection: Connection) -> Self {
        let connection = Arc::new(Mutex::new(connection));
        Db { connection }
    }

    pub fn create_voter(&self, new_voter: NewVoter) -> Result<Voter, Error> {
        let connection = &self.get_connection()?;
        let id: i64 = connection.query_row(
            "INSERT INTO voters (name) VALUES (?1) RETURNING id",
            [&new_voter.name],
            |row| row.get(0),
        )?;
        let voter = Voter {
            id,
            name: new_voter.name,
        };
        Ok(voter)
    }
    pub fn find_voters(&self) -> Result<Vec<Voter>, Error> {
        let connection = &self.get_connection()?;
        let mut statement = connection.prepare("SELECT id, name FROM voters").unwrap();
        let iter = statement.query_map([], |row| {
            let voter = Voter {
                id: row.get(0)?,
                name: row.get(1)?,
            };
            Ok(voter)
        })?;
        Ok(iter.collect::<rusqlite::Result<Vec<Voter>>>()?)
    }

    pub fn find_voter_by_id(&self, id: i64) -> Result<Voter, Error> {
        let connection = &self.get_connection()?;
        connection.query_row("SELECT id, name FROM voters WHERE id = ?1", [id], |row| {
            Ok(Voter {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
    }

    pub fn update_voter(&self, id: i64, name: String) -> Result<Voter, Error> {
        let connection = &self.get_connection()?;
        connection.execute(
            "UPDATE voters SET name = ?1 WHERE id = ?2",
            [&name, &id.to_string()],
        )?;
        Ok(Voter { id, name })
    }

    pub fn delete_voter(&self, id: i64) -> Result<(), Error> {
        let connection = &self.get_connection()?;
        connection.execute("DELETE FROM voters WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<Settings, Error> {
        let connection = &self.get_connection()?;
        let json: String =
            connection.query_row("SELECT data FROM settings LIMIT 1", [], |row| row.get(0))?;
        let settings: Settings =
            serde_json::from_str(&json).expect("Failed to deserialize settings");
        Ok(settings)
    }

    pub fn set_settings(&self, settings: &Settings) -> Result<(), Error> {
        let connection = &self.get_connection()?;
        let data = serde_json::to_string(settings).unwrap();
        let cloned = &data.clone();
        connection
            .execute("INSERT INTO settings (id, data) VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET data = ?3", (1, data, cloned))
            .map(|_| Ok(()))?
    }

    pub fn initialize_schemas(&self) -> Result<(), Error> {
        self.create_settings_table()
            .expect("Failed to create settings table");
        self.create_voters_table()
            .expect("Failed to create voters table");
        Ok(())
    }

    fn create_settings_table(&self) -> Result<(), Error> {
        let query =
            "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY, data TEXT NOT NULL)";
        let connection = &self.get_connection()?;
        connection.execute(query, ()).unwrap();
        Ok(())
    }

    fn create_voters_table(&self) -> Result<(), Error> {
        let query =
            "CREATE TABLE IF NOT EXISTS voters (id INTEGER PRIMARY KEY, name TEXT NOT NULL)";
        let connection = &self.get_connection()?;
        connection.execute(query, ()).unwrap();
        Ok(())
    }

    fn get_connection(&self) -> Result<MutexGuard<Connection>, Error> {
        Ok(self.connection.lock().expect("Failed to acquire lock"))
    }
}
