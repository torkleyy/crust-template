use crate::util::{MapCouchErr, ResponderResult};
use crate::Databases;
use couch_rs::types::query::QueryParams;
use rocket::serde::json::Json;
use rocket::State;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

pub type SingleResult<T> = ResponderResult<Option<Json<T>>>;

pub async fn query_by_key<T: Debug + DeserializeOwned>(
    dbs: &State<Databases>,
    key: String,
    design_name: &str,
    view_name: &str,
) -> SingleResult<T> {
    let result = dbs
        .books
        .query::<Value, T, Value>(
            design_name,
            view_name,
            Some(QueryParams {
                key: Some(key),
                ..Default::default()
            }),
        )
        .await
        .map_couch_err()?
        .rows
        .drain(..)
        .next()
        .map(|v| v.value);

    Ok(result.map(Json))
}
