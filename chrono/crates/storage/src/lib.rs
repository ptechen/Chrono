use gloo_storage::Storage;
use std::sync::{LazyLock, RwLock};

pub static TOKEN: LazyLock<RwLock<String>> = LazyLock::new(|| {
    let token = gloo_storage::LocalStorage::get("Auth").unwrap_or_default();
    RwLock::new(token)
});

pub fn set_token(token: &str) {
    gloo_storage::LocalStorage::set("Auth", token).unwrap_or_default();
    let mut w = TOKEN.write().unwrap();
    *w = token.to_string();
}

pub fn get_token() -> String {
    let token = TOKEN.read().unwrap();
    token.to_string()
}

pub fn del_token() {
    gloo_storage::LocalStorage::delete("Auth");
}
