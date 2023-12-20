use actix_web::FromRequest;
use derive_more::{AsRef, Deref, Display, Into};

use crate::ExtractError;

/// Gets the IpAddr from the request
/// Uses [`actix_web::dev::ConnectionInfo::realip_remote_addr`]
/// ```no_run
/// use actix_quick_extract::IpAddr;
/// use actix_web::get;
/// #[get("/")]
/// pub async fn index(ip_addr: IpAddr) -> String {
///     format!("Your ip_addr Header is: {}", ip_addr)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, AsRef, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[as_ref(str)]
#[deref(forward)]
#[repr(transparent)]
pub struct IpAddr(String);

impl IpAddr {
    #[inline]
    fn from_request_inner(req: &actix_web::HttpRequest) -> Result<Self, ExtractError> {
        let Some(addr) = req
            .connection_info()
            .realip_remote_addr()
            .map(|v| v.to_owned())
        else {
            log::debug!("No Ip Addr Found");
            return Err(ExtractError::MissingInfo("IpAddr"));
        };

        Ok(IpAddr(addr))
    }
}

crate::ready_impl_from_request!(IpAddr, from_request_inner);
