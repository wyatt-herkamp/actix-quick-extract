# Actix-Quick-Extract

Extract information from a web request easily.

## Example

```rust
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
```
