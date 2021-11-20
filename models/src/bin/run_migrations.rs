use couch_rs::database::Database;
use couch_rs::error::{CouchError, CouchResult};
use couch_rs::Client;
use models::Model;
use serde_json::json;
use models::db_utils::connect_from_env;

#[tokio::main]
async fn main() -> CouchResult<()> {
    let db = connect_from_env().await?;
    prepare_database(&db).await?;

    Ok(())
}

async fn prepare_database(db: &Database) -> Result<(), CouchError> {
    println!("Updating design documents...");

    let mut design_typed = json!({
        "_id": "_design/typed",
        "language": "javascript",
        "views": {
            "authors": {
                "map": models::Author::js_type_filter(),
            },
            "books": {
                "map": models::Book::js_type_filter(),
            },
        },
    });

    db.upsert(&mut design_typed).await?;

    println!("Updated design documents");

    Ok(())
}
