//! A collection of types that make extracting headers from requests easier
//!
//! # Note
//!
//! Headers are assumed to be valid strings. If they are not valid strings, then the request will fail
//! If you want to handle invalid strings you will need to access the headers directly
//!
//! The data is cloned out of the request. if you are wanting to prevent this. You will need to access the headers directly
mod authorization;
mod host;
mod origin;
mod origin_or_host;
mod user_agent;
use actix_web::FromRequest;
#[doc(inline)]
pub use authorization::RawAuthorization;
use derive_more::Into;
#[doc(inline)]
pub use host::Host;
#[doc(inline)]
pub use origin::Origin;
#[doc(inline)]
pub use origin_or_host::OriginOrHost;
#[doc(inline)]
pub use user_agent::UserAgent;

use crate::ExtractError;

/// Will accept no value. But will deny if the header is not valid. Such as if it is not valid UTF-8
///
/// It wraps the value in an Option
///
/// None means the header was not present
///
/// ```no_run
/// use actix_quick_extract::headers::{AcceptNoneDenyBad, Origin};
/// use actix_web::get;
///
/// #[get("/")]
/// pub async fn index(origin: AcceptNoneDenyBad<Origin>) -> String {
///     format!("Your Origin Header is: {:?}", origin.0)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Into)]
#[repr(transparent)]
pub struct AcceptNoneDenyBad<T: HeaderType>(pub Option<T>);

/// A type that can be extracted from a request header
pub trait HeaderType {
    fn from_request(req: &actix_web::HttpRequest) -> Result<Self, crate::ExtractError>
    where
        Self: Sized;
}

impl<T: HeaderType> FromRequest for AcceptNoneDenyBad<T> {
    type Error = ExtractError;
    type Future = futures_util::future::Ready<Result<Self, crate::ExtractError>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let result = <T>::from_request(req).map(|v| AcceptNoneDenyBad(Some(v)));
        let result = if let Err(ExtractError::MissingHeader(_)) = &result {
            Ok(AcceptNoneDenyBad(None))
        } else {
            result
        };
        futures_util::future::ready(result)
    }
}

macro_rules! simple_header {
    ($for_type:ty, $header:ident, $header_name:literal) => {
        impl crate::headers::HeaderType for $for_type {
            #[inline]
            fn from_request(req: &actix_web::HttpRequest) -> Result<Self, crate::ExtractError> {
                let header_value = if let Some(value) = req.headers().get($header) {
                    value
                        .to_str()
                        .map(|value| value.to_owned())
                        .map_err(|v| crate::ExtractError::ToStrError($header_name, v))?
                } else {
                    log::debug!(concat!("No `", $header_name, "` Header Found"));
                    return Err(crate::ExtractError::MissingHeader($header_name));
                };

                Ok(<$for_type>::from(header_value))
            }
        }
        crate::ready_impl_from_request!($for_type as Header);
    };
}

pub(crate) use simple_header;
