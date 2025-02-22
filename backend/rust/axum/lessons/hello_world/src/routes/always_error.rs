use axum::http::StatusCode;

pub async fn always_error() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}
