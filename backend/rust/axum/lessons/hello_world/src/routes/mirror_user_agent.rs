use axum::http::HeaderMap;

pub async fn mirror_user_agent(header: HeaderMap) -> String {
    let user_agent = header.get("user-agent").unwrap();
    user_agent.to_str().unwrap().to_owned()
}
