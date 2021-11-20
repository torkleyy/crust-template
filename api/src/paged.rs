use crate::util::MapCouchErr;
use crate::Databases;
use couch_rs::types::query::QueryParams;
use models::PagedResponseData;
use rocket::serde::json::Json;
use rocket::State;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Deserialize, FromForm, JsonSchema, Serialize)]
pub struct UserQuery {
    #[field(default = 0)]
    #[serde(default)]
    start: u64,
    #[field(default = UserQuery::default_limit())]
    #[schemars(default = "UserQuery::default_limit")]
    limit: u64,
}

impl UserQuery {
    fn default_limit() -> u64 {
        20
    }
}

pub type PagedResponse<T> = Json<PagedResponseData<T>>;

pub type PagedError = crate::error::Error;
pub type PagedResult<T> = Result<PagedResponse<T>, PagedError>;

pub async fn paged_query<T: Debug + DeserializeOwned>(
    dbs: &State<Databases>,
    query: UserQuery,
    design_name: &str,
    view_name: &str,
) -> PagedResult<T> {
    let UserQuery { start, limit } = query;

    let result = dbs
        .books
        .query::<Value, T, Value>(
            design_name,
            view_name,
            Some(QueryParams {
                limit: Some(limit),
                skip: Some(start),
                ..Default::default()
            }),
        )
        .await;
    let result = result.map_couch_err()?;
    Ok(PagedResponseData {
        total: result.total_rows.unwrap_or(0) as u64,
        rows: result
            .rows
            .into_iter()
            .map(|row| row.value)
            .collect::<Vec<_>>(),
    }
    .into())
}
