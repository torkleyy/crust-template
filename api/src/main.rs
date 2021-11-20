#[macro_use]
extern crate rocket;

use couch_rs::database::Database;
use couch_rs::error::CouchError;
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use models::db_utils::{connect_from_env_db};

mod error;
mod paged;
mod single;
mod util;
mod v1;

pub struct Databases {
    pub users: Database,
    pub books: Database,
}

#[get("/health")]
fn health(_dbs: &State<Databases>) -> &'static str {
    "server running\n"
}

#[get("/", format = "text/html")]
fn index_redirect() -> Redirect {
    Redirect::to("/rapidoc")
}

async fn build_rocket() -> Result<Rocket<Build>, CouchError> {
    let dbs = Databases {
        users: connect_from_env_db("USERS").await?,
        books: connect_from_env_db("BOOKS").await?,
    };

    let rocket = rocket::build()
        .manage(dbs)
        .mount("/", routes![health, index_redirect])
        .mount("/v1", v1::routes())
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc",
            make_rapidoc(&RapiDocConfig {
                title: Some("RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("OpenAPI", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );
    Ok(rocket)
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = build_rocket().await?.launch().await {
        drop(e);
    };

    Ok(())
}
