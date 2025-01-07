use leptos::prelude::*;
use crate::login::Login;
use crate::register::Register;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoginRegisterState {
    Login,
    Register,
}

#[component]
pub fn LoginRegister() -> impl IntoView {
    let (state, set_state) = signal(LoginRegisterState::Login);
    view! {
        <div class="min-h-screen flex items-center justify-center p-4">
            <div class="p-8 rounded-lg w-full max-w-md">
                <Show
                    when=move || { state.get() == LoginRegisterState::Login }
                    fallback=move|| view! { <Register set_state/> }
                >
                    <Login set_state/>
                </Show>
            </div>
        </div>
    }
}