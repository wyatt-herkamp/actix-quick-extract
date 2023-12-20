use actix_web::{http::header::AUTHORIZATION, FromRequest};
use derive_more::{AsRef, Deref, Display, From, Into};
/// The `Authorization` header.
///
/// No parsing is done on the header, it is simply returned as a string.
///
/// ```no_run
/// use actix_quick_extract::headers::RawAuthorization;
/// use actix_web::get;
///
/// #[get("/")]
/// pub async fn index(auth: RawAuthorization) -> String {
///     format!("Your auth Header is: {}", auth)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[as_ref(str)]
#[deref(forward)]
#[repr(transparent)]
pub struct RawAuthorization(pub String);

super::simple_header!(RawAuthorization, AUTHORIZATION, "Authorization");
