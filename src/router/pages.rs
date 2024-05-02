use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    let html_content = include_str!("../../index.html");
    Html(html_content)
}

pub async fn success() -> Html<&'static str> {
    let html_content = include_str!("../../login_success.html");
    Html(html_content)
}
pub async fn error() -> Html<&'static str> {
    let html_content = include_str!("../../error.html");
    Html(html_content)
}
pub async fn upload_page() -> Html<&'static str> {
    let html_content = include_str!("../../upload.html");
    Html(html_content)
}
