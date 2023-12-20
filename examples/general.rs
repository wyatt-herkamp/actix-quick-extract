use std::io;

use actix_quick_extract::{
    headers::{AcceptNoneDenyBad, Host, Origin, OriginOrHost, RawAuthorization, UserAgent},
    IpAddr,
};
use actix_web::{get, App, HttpServer};

#[actix_web::main]
pub async fn main() -> io::Result<()> {
    let bind_address = option_env!("BIND_ADDRESS").unwrap_or("0.0.0.0:8080");
    let server = HttpServer::new(move || {
        App::new()
            .service(ip_addr)
            .service(host)
            .service(origin)
            .service(origin_or_host)
            .service(accept_none_deny_bad)
            .service(user_agent)
            .service(raw_authorization)
    });
    server.workers(1).bind(bind_address)?.run().await
}

#[get("/ip-addr")]
pub async fn ip_addr(ip_addr: IpAddr) -> String {
    format!("Your ip_addr Header is: {}", ip_addr)
}

#[get("/host")]
pub async fn host(host: Host) -> String {
    format!("Your host Header is: {}", host)
}

#[get("/origin")]
pub async fn origin(origin: Origin) -> String {
    format!("Your Origin Header is: {}", origin)
}

#[get("/accept-none-deny-bad")]
pub async fn accept_none_deny_bad(accept_none_deny_bad: AcceptNoneDenyBad<Origin>) -> String {
    format!("Your accept_none_deny_bad: {:?}", accept_none_deny_bad.0)
}
#[get("/origin-or-host")]
pub async fn origin_or_host(origin_or_host: OriginOrHost) -> String {
    format!("Your origin or host Header is: {}", origin_or_host)
}

#[get("/user-agent")]
pub async fn user_agent(user_agent: UserAgent) -> String {
    format!("Your user agent Header is: {}", user_agent)
}

#[get("/raw-authorization")]
pub async fn raw_authorization(auth: RawAuthorization) -> String {
    format!("Your raw authorization Header is: {}", auth)
}
