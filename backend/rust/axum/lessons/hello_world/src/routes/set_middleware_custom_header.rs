use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let message = headers
        .get("x-message")
        .ok_or(StatusCode::BAD_REQUEST)?;
    let message = message.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;

    request.extensions_mut().insert(HeaderMessage(message.to_owned()));

    Ok(next.run(request).await)
}
