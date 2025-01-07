use leptos::prelude::*;
use crate::index::Index;
use crate::login_register::login_register::LoginRegister;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShowSelect {
    Index,
    LoginRegister,
}

#[component]
pub fn Root() -> impl IntoView {
    let (show_select, set_show_select) = signal(ShowSelect::LoginRegister);
    view! {
        <Show when=move || show_select.get() == ShowSelect::Index
              fallback=move || view! {
                <LoginRegister set_show_select/>
              }
        >
           <Index set_show_select/>
       </Show>
    }
}