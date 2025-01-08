mod chat;
mod friend;
mod handler_404;
mod hello_world;
mod show_file;
mod signup_login;
mod upload_file;
mod verification_code;
mod ws;

use crate::controllers::chat::{chat_list, chat_send};
use crate::controllers::friend::{friend_add, friend_list, friend_reply, friend_search};
use crate::controllers::hello_world::healthcheck;
use crate::controllers::show_file::show_file;
use crate::controllers::signup_login::{login, reset_password, signup};
use crate::controllers::upload_file::upload_files;
use crate::controllers::verification_code::{send_verification_code, verify_verification_code};
use crate::controllers::ws::ws_handler;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    Router::new()
        .route("/api/healthcheck", get(healthcheck))
        .route("/api/verification_code/send", post(send_verification_code))
        .route(
            "/api/verification_code/verify",
            post(verify_verification_code),
        )
        .route("/api/signup", post(signup))
        .route("/api/login", post(login))
        .route("/api/password/reset", post(reset_password))
        .route("/api/friend/add", post(friend_add))
        .route("/api/friend/list", post(friend_list))
        .route("/api/friend/search", post(friend_search))
        .route("/api/friend/reply", post(friend_reply))
        .route("/api/upload/{file_name}", post(upload_files))
        .route("/api/show/{dir}/{file}", get(show_file))
        .route("/api/chat/list", post(chat_list))
        .route("/api/chat/send", post(chat_send))
        .route("/ws", get(ws_handler))
        .fallback(handler_404::handler_404)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
