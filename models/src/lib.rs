use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "couch")]
use couch_rs::document::TypedCouchDocument;
#[cfg(feature = "couch")]
use couch_rs_derive::CouchDocument;
use schemars::JsonSchema;

#[cfg(feature = "couch")]
pub mod db_utils;

pub type DocumentId = String;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(transparent)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(Utc::now())
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[cfg_attr(feature = "couch", derive(CouchDocument))]
pub struct Author {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

    /// the author's full name
    pub name: String,
    /// the date the author was born, in [RFC-3339](https://datatracker.ietf.org/doc/html/rfc3339)
    /// format.
    pub birth_date: Timestamp,
}

impl Model for Author {
    const TYPE: &'static str = "Author";
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[cfg_attr(feature = "couch", derive(CouchDocument))]
pub struct Book {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,

    pub title: String,
    pub author_id: DocumentId,
}

impl Model for Book {
    const TYPE: &'static str = "Book";
}

pub trait Model {
    const TYPE: &'static str;

    fn js_type_filter() -> String {
        format!(
            r##"function (doc) {{
    if (doc.type === '{}') {{
        emit(doc._id, doc);
    }}
}}
"##,
            Self::TYPE
        )
    }
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct PagedResponseData<T> {
    pub total: u64,
    pub rows: Vec<T>,
}

#[cfg(test)]
mod tests {}
