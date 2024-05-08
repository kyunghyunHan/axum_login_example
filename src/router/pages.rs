use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    let html_content = include_str!("../../index.html");
    Html(html_content)
}


pub async fn error() -> Html<&'static str> {
    let html_content = include_str!("../../error.html");
    Html(html_content)
}

pub async fn sign() -> Html<&'static str> {
    let html_content = include_str!("../../signup.html");
    Html(html_content)
}
pub async fn admin() -> Html<&'static str> {
    let html_content = include_str!("../../admin.html");
    Html(html_content)
}
