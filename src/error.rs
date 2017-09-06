//! error types
//!
//! `gauthz::Errors` are designed to play well with the `error_chain`
//! crate ecosystem

use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;

#[derive(Deserialize)]
pub(crate) struct ApiError {
    pub error: String,
    pub error_description: String,
}

error_chain! {
      errors {
          /// Represents an http response code error, usually related to client
          /// code
          Api {
              code: StatusCode,
              error: String,
              error_description: String
          } {
              description(error_description)
          }
      }
      foreign_links {
          Codec(SerdeError) #[doc = "Represents serialization related errors"];
          Http(HttpError) #[doc = "Represents HTTP protocol level errors"];
          IO(IoError) #[doc = "Represents generally IO errors"];
          Uri(UriError) #[doc = "Represents URI encoding errors"];
      }
  }
