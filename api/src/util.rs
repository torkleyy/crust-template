use couch_rs::error::CouchError;

pub type ResponderResult<T> = Result<T, crate::error::Error>;

pub trait MapCouchErr {
    type Responder;

    fn map_couch_err(self) -> Self::Responder;
}

impl<T> MapCouchErr for Result<T, CouchError> {
    type Responder = ResponderResult<T>;

    fn map_couch_err(self) -> Self::Responder {
        self.map_err(Into::into)
    }
}
