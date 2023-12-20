use actix_web::{http::header::USER_AGENT, FromRequest};
use derive_more::{AsRef, Deref, Display, From, Into};

use super::simple_header;

/// The `user-agent` header.
///
/// ```no_run
/// use actix_quick_extract::headers::UserAgent;
/// use actix_web::get;
///
/// #[get("/")]
/// pub async fn index(user_agent: UserAgent) -> String {
///     format!("Your user agent Header is: {}", user_agent)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[as_ref(str)]
#[deref(forward)]
#[repr(transparent)]
pub struct UserAgent(pub String);

simple_header!(UserAgent, USER_AGENT, "User-Agent");
