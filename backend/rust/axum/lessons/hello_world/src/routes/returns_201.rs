use axum::http::StatusCode;

pub async fn returns_201() -> (StatusCode, String) {
    (
        StatusCode::CREATED,
        "Created".to_owned(),
    )
}
