//! error types
//!
//! `gauthz::Errors` are designed to play well with the `error_chain`
//! crate ecosystem

use hyper::Error as HttpError;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

#[derive(Deserialize)]
pub(crate) struct ApiError {
    pub error: String,
    pub error_description: String,
}

error_chain! {
      errors {
          /// Represents an HTTP response code error, usually related to client
          /// code
          Api(error: String, error_description: String) {
            display("{}: '{}'", error, error_description)
            description(error_description)
          }
      }
      foreign_links {
          Codec(SerdeError) #[doc = "Represents serialization related errors"];
          Http(HttpError) #[doc = "Represents HTTP protocol level errors"];
          IO(IoError) #[doc = "Represents generally IO errors"];
      }
  }

#[cfg(test)]
mod tests {
    use super::ErrorKind;
    #[test]
    fn api_description() {
        assert_eq!(
            ErrorKind::Api("foobar".into(), "it happened".into()).description(),
            "it happened"
        )
    }

    #[test]
    fn api_display() {
        assert_eq!(
            ErrorKind::Api("foobar".into(), "it happened".into()).to_string(),
            "foobar: 'it happened'"
        )
    }
}
