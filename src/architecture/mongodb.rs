use mongodb::{Client, Database};

pub struct MongoDb {
    pub db: Database,
}

impl MongoDb {
    pub fn new(client: &Client, db_name: String) -> Self {
        Self {
            db: client.database(db_name.as_str()),
        }
    }
}

impl Clone for MongoDb {
    fn clone(&self) -> Self {
        Self { db: self.db.clone() }
    }
}
