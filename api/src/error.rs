use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
};
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::schemars::{self, Map};
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner, OpenApiError};

/// Error messages returned to user
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct Error {
    /// The title of the error message
    pub err: String,
    /// The description of the error
    pub msg: Option<String>,
    // HTTP Status Code returned
    #[serde(skip)]
    pub http_status_code: u16,
}

impl OpenApiResponderInner for Error {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description:
                    "The request given is wrongly formatted or data asked could not be fulfilled."
                        .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "This response is given when you request a page that does not exists."
                    .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description:
                    "This response is given when you request body is not correctly formatted."
                        .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "This response is given when something went wrong on the server."
                    .to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Error `{}`: {}",
            self.err,
            self.msg.as_deref().unwrap_or("<no message>")
        )
    }
}

impl std::error::Error for Error {}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.http_status_code))
            .ok()
    }
}

impl From<rocket::serde::json::Error<'_>> for Error {
    fn from(err: rocket::serde::json::Error) -> Self {
        use rocket::serde::json::Error::*;
        match err {
            Io(io_error) => Error {
                err: "io error".to_owned(),
                msg: Some(io_error.to_string()),
                http_status_code: 422,
            },
            Parse(_raw_data, parse_error) => Error {
                err: "parse error".to_owned(),
                msg: Some(parse_error.to_string()),
                http_status_code: 422,
            },
        }
    }
}

impl From<couch_rs::error::CouchError> for Error {
    fn from(e: couch_rs::error::CouchError) -> Self {
        match e.status.as_u16() {
            501 => Error {
                err: "unexpected database response".to_owned(),
                msg: Some(e.message),
                http_status_code: 500,
            },
            _ => Error {
                err: "database error".to_string(),
                msg: Some(e.status.to_string()),
                http_status_code: 500,
            },
        }
    }
}
