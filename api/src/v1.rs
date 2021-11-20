use crate::paged::{paged_query, PagedResult, UserQuery};
use crate::single::{query_by_key, SingleResult};
use crate::Databases;
use rocket::{Route, State};
use rocket_okapi::{openapi, openapi_get_routes};

pub fn routes() -> Vec<Route> {
    openapi_get_routes![authors, author_by_id, books,]
}

#[openapi]
#[get("/authors?<query..>")]
async fn authors(query: UserQuery, dbs: &State<Databases>) -> PagedResult<models::Author> {
    paged_query(dbs, query, "typed", "authors").await
}

#[openapi]
#[get("/authors/<id>")]
async fn author_by_id(id: String, dbs: &State<Databases>) -> SingleResult<models::Author> {
    query_by_key(dbs, id, "typed", "authors").await
}

#[openapi]
#[get("/books?<query..>")]
async fn books(query: UserQuery, dbs: &State<Databases>) -> PagedResult<models::Book> {
    paged_query(dbs, query, "typed", "books").await
}
