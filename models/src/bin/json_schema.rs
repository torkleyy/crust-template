use models::*;
use schemars::schema_for;
use serde_json::to_writer;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

fn prefix() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("schema");

    path
}

fn get_path(s: &str) -> PathBuf {
    let mut path = prefix();
    path.push(s);
    path.set_extension("json");

    path
}

macro_rules! write_schema {
    ($t:ty) => {
        {
            let schema: schemars::schema::RootSchema = schema_for!($t);
            let schema = serde_json::to_value(schema).unwrap();
            let title = schema.get("title").unwrap().as_str().unwrap();

            to_writer(std::fs::File::create(get_path(title)).unwrap(), &schema).unwrap();
        };
    };
    ($($t:ty),*) => {
        $(
        write_schema!($t);
        )*
    };
}

fn main() {
    remove_dir_all(prefix()).unwrap();
    create_dir_all(prefix()).unwrap();

    write_schema![
        Author,
        Book,
        PagedResponseData<Author>,
        PagedResponseData<Book>
    ];
}
