use actix_web::{http::header::ORIGIN, FromRequest};
use derive_more::{AsRef, Deref, Display, Into};

use super::HeaderType;
use crate::ExtractError;
/// The `Origin` header.
///
/// ```no_run
/// use actix_quick_extract::headers::Origin;
/// use actix_web::get;
/// #[get("/")]
/// pub async fn index(origin: Origin) -> String {
///     format!("Your Origin Header is: {}", origin)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{url}")]
pub struct Origin {
    #[into]
    #[as_ref(str)]
    #[deref(forward)]
    pub url: String,
    pub is_https: bool,
}

impl HeaderType for Origin {
    #[inline]
    fn from_request(req: &actix_web::HttpRequest) -> Result<Self, crate::ExtractError>
    where
        Self: Sized,
    {
        let (url, https) = if let Some(value) = req.headers().get(ORIGIN) {
            value
                .to_str()
                .map(|value| (value.to_string(), value.starts_with("https")))
                .map_err(|v| ExtractError::ToStrError("Origin", v))?
        } else {
            log::debug!("No Origin Header Found");
            return Err(ExtractError::MissingHeader("Origin"));
        };

        Ok(Origin {
            url,
            is_https: https,
        })
    }
}

crate::ready_impl_from_request!(Origin as Header);
