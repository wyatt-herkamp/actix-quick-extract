use actix_web::{http::header::HOST, FromRequest};
use derive_more::{AsRef, Deref, Display, From, Into};

use super::simple_header;

/// The `Host` header.
///
/// ```no_run
/// use actix_quick_extract::headers::Host;
/// use actix_web::get;
///
/// #[get("/")]
/// pub async fn index(host: Host) -> String {
///     format!("Your host Header is: {}", host)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[as_ref(str)]
#[deref(forward)]
#[repr(transparent)]
pub struct Host(pub String);

simple_header!(Host, HOST, "Host");
