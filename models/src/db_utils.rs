use std::env::var;
use couch_rs::Client;
use couch_rs::database::Database;
use couch_rs::error::CouchResult;

pub async fn connect_from_env() -> CouchResult<Database> {
    connect_from_env_db("BOOKS").await
}

pub async fn connect_from_env_db(db: &str) -> CouchResult<Database> {
    let uri = var("DB_URI").expect("DB_URI env var missing or invalid");
    let username = var("DB_USER").expect("DB_USER env var missing or invalid");
    let password = var("DB_PASS").expect("DB_PASS env var missing or invalid");
    let db_name = format!("DB_NAME_{}", db);
    let db_name = var(&db_name).expect(&(db_name + " env var missing or invalid"));

    let couch_client = Client::new(&uri, &username, &password)?;
    let db = couch_client.db(&db_name).await?;

    Ok(db)
}
