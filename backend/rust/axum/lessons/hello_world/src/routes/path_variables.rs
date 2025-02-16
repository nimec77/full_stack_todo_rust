use axum::extract::Path;

pub async fn path_variables(Path(id): Path<u32>) -> String {
    format!("Get id: {id}")
}

pub async fn hard_coded_path() -> String {
    "You got 42!".to_owned()
}
