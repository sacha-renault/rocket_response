use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Serialize;

pub type ApiResult<T, E> = Result<status::Custom<Json<T>>, status::Custom<Json<E>>>;

// 200 OK
pub fn ok<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::Ok, Json(data)))
}

// 201 Created
pub fn created<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::Created, Json(data)))
}

// 202 Accepted
pub fn accepted<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::Accepted, Json(data)))
}

// 203 Non-Authoritative Information
pub fn non_authoritative<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::NonAuthoritativeInformation, Json(data)))
}

// 204 No Content
pub fn no_content() -> Result<status::Custom<Json<()>>, ()> {
    Ok(status::Custom(Status::NoContent, Json(())))
}

// 205 Reset Content
pub fn reset_content() -> Result<status::Custom<Json<()>>, ()> {
    Ok(status::Custom(Status::ResetContent, Json(())))
}

// 206 Partial Content
pub fn partial_content<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::PartialContent, Json(data)))
}

// 207 Multi-Status (WebDAV)
pub fn multi_status<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::MultiStatus, Json(data)))
}

// 208 Already Reported (WebDAV)
pub fn already_reported<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::AlreadyReported, Json(data)))
}

// 226 IM Used (HTTP Delta Encoding)
pub fn im_used<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::ImUsed, Json(data)))
}

// 300 Multiple Choices
pub fn multiple_choices<T>(data: T) -> Result<status::Custom<Json<T>>, ()>
where
    T: Serialize,
{
    Ok(status::Custom(Status::MultipleChoices, Json(data)))
}

// 301 Moved Permanently
pub fn moved_permanently(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::MovedPermanently, Json(location.to_string())))
}

// 302 Found
pub fn found(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::Found, Json(location.to_string())))
}

// 303 See Other
pub fn see_other(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::SeeOther, Json(location.to_string())))
}

// 304 Not Modified
pub fn not_modified() -> Result<status::Custom<()>, ()> {
    Ok(status::Custom(Status::NotModified, ()))
}

// 305 Use Proxy (Deprecated)
pub fn use_proxy(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::UseProxy, Json(location.to_string())))
}

// 307 Temporary Redirect
pub fn temporary_redirect(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::TemporaryRedirect, Json(location.to_string())))
}

// 308 Permanent Redirect
pub fn permanent_redirect(location: &str) -> Result<status::Custom<Json<String>>, ()> {
    Ok(status::Custom(Status::PermanentRedirect, Json(location.to_string())))
}

// 400 Bad Request
pub fn bad_request<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::BadRequest, Json(data)))
}

// 401 Unauthorized
pub fn unauthorized<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::Unauthorized, Json(data)))
}

// 402 Payment Required
pub fn payment_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::PaymentRequired, Json(data)))
}

// 403 Forbidden
pub fn forbidden<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::Forbidden, Json(data)))
}

// 404 Not Found
pub fn not_found<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::NotFound, Json(data)))
}

// 405 Method Not Allowed
pub fn method_not_allowed<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::MethodNotAllowed, Json(data)))
}

// 406 Not Acceptable
pub fn not_acceptable<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::NotAcceptable, Json(data)))
}

// 407 Proxy Authentication Required
pub fn proxy_auth_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::ProxyAuthenticationRequired, Json(data)))
}

// 408 Request Timeout
pub fn request_timeout<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::RequestTimeout, Json(data)))
}

// 409 Conflict
pub fn conflict<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::Conflict, Json(data)))
}

// 410 Gone
pub fn gone<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::Gone, Json(data)))
}

// 411 Length Required
pub fn length_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::LengthRequired, Json(data)))
}

// 412 Precondition Failed
pub fn precondition_failed<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::PreconditionFailed, Json(data)))
}

// 413 Payload Too Large
pub fn payload_too_large<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::PayloadTooLarge, Json(data)))
}

// 414 URI Too Long
pub fn uri_too_long<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::UriTooLong, Json(data)))
}

// 415 Unsupported Media Type
pub fn unsupported_media_type<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::UnsupportedMediaType, Json(data)))
}

// 416 Range Not Satisfiable
pub fn range_not_satisfiable<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::RangeNotSatisfiable, Json(data)))
}

// 417 Expectation Failed
pub fn expectation_failed<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::ExpectationFailed, Json(data)))
}

// 418 I'm a Teapot (RFC 2324)
pub fn im_a_teapot<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::ImATeapot, Json(data)))
}

// 421 Misdirected Request
pub fn misdirected_request<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::MisdirectedRequest, Json(data)))
}

// 422 Unprocessable Entity (WebDAV)
pub fn unprocessable_entity<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::UnprocessableEntity, Json(data)))
}

// 423 Locked (WebDAV)
pub fn locked<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::Locked, Json(data)))
}

// 424 Failed Dependency (WebDAV)
pub fn failed_dependency<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::FailedDependency, Json(data)))
}

// 426 Upgrade Required
pub fn upgrade_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::UpgradeRequired, Json(data)))
}

// 428 Precondition Required
pub fn precondition_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::PreconditionRequired, Json(data)))
}

// 429 Too Many Requests
pub fn too_many_requests<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::TooManyRequests, Json(data)))
}

// 431 Request Header Fields Too Large
pub fn request_header_fields_too_large<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::RequestHeaderFieldsTooLarge, Json(data)))
}

// 451 Unavailable For Legal Reasons
pub fn unavailable_for_legal_reasons<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::UnavailableForLegalReasons, Json(data)))
}

// 500 Internal Server Error
pub fn internal_server_error<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::InternalServerError, Json(data)))
}

// 501 Not Implemented
pub fn not_implemented<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::NotImplemented, Json(data)))
}

// 502 Bad Gateway
pub fn bad_gateway<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::BadGateway, Json(data)))
}

// 503 Service Unavailable
pub fn service_unavailable<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::ServiceUnavailable, Json(data)))
}

// 504 Gateway Timeout
pub fn gateway_timeout<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::GatewayTimeout, Json(data)))
}

// 505 HTTP Version Not Supported
pub fn http_version_not_supported<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::HttpVersionNotSupported, Json(data)))
}

// 506 Variant Also Negotiates (RFC 2295)
pub fn variant_also_negotiates<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::VariantAlsoNegotiates, Json(data)))
}

// 507 Insufficient Storage (WebDAV)
pub fn insufficient_storage<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::InsufficientStorage, Json(data)))
}

// 508 Loop Detected (WebDAV)
pub fn loop_detected<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::LoopDetected, Json(data)))
}

// 510 Not Extended (WebDAV)
pub fn not_extended<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::NotExtended, Json(data)))
}

// 511 Network Authentication Required
pub fn network_auth_required<T>(data: T) -> Result<(), status::Custom<Json<T>>>
where
    T: Serialize,
{
    Err(status::Custom(Status::NetworkAuthenticationRequired, Json(data)))
}