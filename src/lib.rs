pub mod architecture;
pub mod models;

use crate::architecture::mongodb::MongoDb;
use crate::models::mymodel::MyModel;
use ddi::Service;
use std::error::Error;
use std::fmt::Display;

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    author: String,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Book {{ id: {:?}, title: {}, author: {} }}",
            self.id, self.title, self.author
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BookTest {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    author: String,
    test: String,
}

impl Display for BookTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BookTest with id {}", self.test)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Books {
    Book(Book),
    Test(BookTest),
}

pub struct CalDav {
    mongodb: Service<MongoDb>,
}

impl CalDav {
    pub fn new(mongodb: Service<MongoDb>) -> Self {
        Self { mongodb }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mymodel = MyModel::new("John".to_string(), 42);
        println!("this is my model {}", mymodel);

        let typed_collection = self.mongodb.db.collection::<Books>("books");

        let books = vec![
            Books::Book(Book {
                id: None,
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
            }),
            Books::Book(Book {
                id: None,
                title: "To Kill a Mockingbird".to_string(),
                author: "Harper Lee".to_string(),
            }),
        ];

        typed_collection.insert_many(books, None).await?;

        let mut cursor = typed_collection.find(None, None).await?;

        println!("test");

        // Iterate over the results of the cursor.
        while let Some(book) = cursor.try_next().await? {
            match book {
                Books::Book(book) => println!("Book: {}", book),
                Books::Test(book) => println!("BookTest: {}", book),
            }
        }

        let count = typed_collection.count_documents(None, None).await?;
        println!("count: {}", count);

        Ok(())
    }
}
