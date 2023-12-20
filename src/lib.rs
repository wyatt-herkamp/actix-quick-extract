#![deny(clippy::str_to_string)]
#![deny(missing_docs)]
//! # Actix-Quick-Extract
//!
//! Extract information from a web request easily.
use actix_web::{http::header::ToStrError, ResponseError};
use thiserror::Error;

pub mod headers;
mod ip_addr;
#[doc(inline)]
pub use ip_addr::IpAddr;
/// Errors that can occur when extracting headers
///
/// All Errors return a `400 Bad Request` response
#[derive(Debug, Error)]
pub enum ExtractError {
    /// Header was not found
    #[error("No {0} Header Found")]
    MissingHeader(&'static str),
    /// Header was not valid UTF-8
    #[error("Header {0} Header was not valid UTF-8")]
    ToStrError(&'static str, ToStrError),
    /// Information such as IpAddr was not found
    #[error("Missing {0}")]
    MissingInfo(&'static str),
}
impl ResponseError for ExtractError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ExtractError::MissingHeader(header_name) => {
                actix_web::HttpResponse::BadRequest().body(format!("No {header_name} Header Found"))
            }
            ExtractError::ToStrError(header_name, error) => actix_web::HttpResponse::BadRequest()
                .body(format!("{header_name} Header was not valid UTF-8: {error}")),
            ExtractError::MissingInfo(name) => {
                actix_web::HttpResponse::BadRequest().body(format!("Missing {name}"))
            }
        }
    }
}

macro_rules! ready_impl_from_request {
    ($f:ty, $fn_name:ident) => {
        impl FromRequest for $f {
            type Error = crate::ExtractError;
            type Future = futures_util::future::Ready<Result<$f, crate::ExtractError>>;

            fn from_request(
                req: &actix_web::HttpRequest,
                _: &mut actix_web::dev::Payload,
            ) -> Self::Future {
                futures_util::future::ready(<$f>::$fn_name(req))
            }
        }
    };
    ($f:ty as Header) => {
        impl FromRequest for $f {
            type Error = crate::ExtractError;
            type Future = futures_util::future::Ready<Result<$f, crate::ExtractError>>;

            fn from_request(
                req: &actix_web::HttpRequest,
                _: &mut actix_web::dev::Payload,
            ) -> Self::Future {
                futures_util::future::ready(<$f as crate::headers::HeaderType>::from_request(req))
            }
        }
    };
}
pub(crate) use ready_impl_from_request;
