use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use couch_rs::database::Database;
use couch_rs::error::{CouchError, CouchResult};
use models::{Author, Book, DocumentId, Timestamp};
use models::db_utils::connect_from_env;

#[tokio::main]
async fn main() -> CouchResult<()> {
    let db = connect_from_env().await?;
    println!("Inserting sample data...");
    populate_db(&db).await?;
    println!("Done inserting sample data.");

    Ok(())
}

fn r(min: u32, max: u32) -> u32 {
    min + (rand::random::<u32>() % (max - min))
}

fn author(name: &str) -> Author {
    let d = NaiveDate::from_ymd(r(1900, 2020) as i32, r(1, 12), r(1, 30));
    let t = NaiveTime::from_hms_milli(r(0, 23), r(0, 60), 0, 0);

    let dt = NaiveDateTime::new(d, t);
    let dt = DateTime::from_utc(dt, Utc);

    Author {
        _id: "".to_string(),
        _rev: "".to_string(),
        name: name.to_owned(),
        birth_date: Timestamp(dt),
    }
}

fn book(title: &str, author: &DocumentId) -> Book {
    Book {
        _id: "".to_string(),
        _rev: "".to_string(),
        title: title.to_string(),
        author_id: author.clone(),
    }
}

async fn populate_db(db: &Database) -> Result<(), CouchError> {
    let mut authors = vec![
        author("Author 1"),
        author("Author 2"),
        author("Author 3"),
        author("Author 4"),
        author("Author 5"),
        author("Author 6"),
        author("Author 7"),
        author("Author 8"),
        author("Author 9"),
        author("Author 10"),
        author("Author 11"),
        author("Author 12"),
    ];

    db.bulk_upsert(&mut authors).await?;

    let mut books = vec![
        book("Book 1", &authors[3]._id),
        book("Book 2", &authors[1]._id),
        book("Book 3", &authors[1]._id),
        book("Book 4", &authors[2]._id),
        book("Book 5", &authors[0]._id),
        book("Book 6", &authors[4]._id),
        book("Book 7", &authors[1]._id),
        book("Book 8", &authors[5]._id),
        book("Book 9", &authors[8]._id),
        book("Book 10", &authors[9]._id),
        book("Book 11", &authors[10]._id),
        book("Book 12", &authors[8]._id),
        book("Book 13", &authors[6]._id),
        book("Book 14", &authors[6]._id),
        book("Book 15", &authors[7]._id),
        book("Book 16", &authors[8]._id),
        book("Book 17", &authors[11]._id),
        book("Book 18", &authors[10]._id),
    ];

    db.bulk_upsert(&mut books).await?;

    Ok(())
}
