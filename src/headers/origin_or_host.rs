use actix_web::{http::header::ORIGIN, FromRequest};
use derive_more::{AsRef, Deref, Display, Into};

use super::HeaderType;
use crate::ExtractError;

/// Attempts to get the Origin header. If not tries the Host Header
///
/// ```no_run
/// use actix_quick_extract::headers::OriginOrHost;
/// use actix_web::get;
/// #[get("/")]
/// pub async fn index(origin_or_host: OriginOrHost) -> String {
///     format!("Your origin or host Header is: {}", origin_or_host)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{url}")]
pub struct OriginOrHost {
    #[into]
    #[as_ref(str)]
    #[deref(forward)]
    pub url: String,
    /// Will be none if the header was Host
    pub is_https: bool,
}

impl HeaderType for OriginOrHost {
    #[inline]
    fn from_request(req: &actix_web::HttpRequest) -> Result<Self, ExtractError> {
        let (url, https) = if let Some(value) = req.headers().get(ORIGIN) {
            value
                .to_str()
                .map(|value| (value.to_owned(), value.starts_with("https")))
                .map_err(|v| ExtractError::ToStrError("Origin", v))?
        } else if let Some(value) = req.headers().get(actix_web::http::header::HOST) {
            value
                .to_str()
                .map(|value| (value.to_owned(), req.connection_info().scheme() == "https"))
                .map_err(|v| ExtractError::ToStrError("Host", v))?
        } else {
            log::debug!("No Origin or Host Header Found");
            return Err(ExtractError::MissingHeader("Origin or Host"));
        };

        Ok(OriginOrHost {
            url,
            is_https: https,
        })
    }
}
crate::ready_impl_from_request!(OriginOrHost as Header);
