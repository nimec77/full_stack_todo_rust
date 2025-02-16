use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let header_value = headers.get("x-custom-header").unwrap();

    let header = header_value.to_str().unwrap().to_owned();

    header
}
