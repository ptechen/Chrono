use crate::logics::upload_file::stream_to_file;
use axum::extract::{Path, Request};
use axum::http::StatusCode;

pub async fn upload_files(
    Path(file_name): Path<String>,
    request: Request,
) -> Result<(), (StatusCode, String)> {
    stream_to_file(&file_name, request.into_body().into_data_stream()).await
}
