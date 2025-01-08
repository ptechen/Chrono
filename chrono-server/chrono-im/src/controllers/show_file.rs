use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use models::avatars::Avatars;

pub async fn show_file(Path((dir, file)): Path<(String, String)>) -> impl IntoResponse {
    match dir.as_str() {
        "avatar" => {
            if let Ok(Some(avatar)) =
                Avatars::select_optional_by_id(&file.replace(".webp", "")).await
            {
                let headers = HeaderMap::new();
                (StatusCode::OK, headers, avatar.avatar)
            } else {
                (StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new())
            }
        }
        _ => (StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new()),
    }
}
