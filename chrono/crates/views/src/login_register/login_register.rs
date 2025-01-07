use leptos::prelude::*;
use crate::root::ShowSelect;
use crate::login_register::login::Login;
use crate::login_register::register::Register;
use crate::login_register::set_password::SetPassword;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoginRegisterState {
    Login,
    Register,
    SetPassword,
}

#[component]
pub fn LoginRegister(set_show_select: WriteSignal<ShowSelect>) -> impl IntoView {
    let (state, set_state) = signal(LoginRegisterState::Login);
    let (mnemonic, set_mnemonic) = signal(String::new());
    let (password, set_password) = signal("".to_string());
    view! {
        <div class="min-h-screen flex items-center justify-center p-4">
            <div class="p-8 rounded-lg w-full max-w-md">
                <Show
                    when=move || { state.get() == LoginRegisterState::Login }
                    fallback=move|| view! { <Show
                    when=move || { state.get() == LoginRegisterState::Register }
                    fallback=move|| view! { <SetPassword set_show_select password set_password/> }
                >
                    <Register set_state mnemonic set_mnemonic/>
                </Show> }
                >
                    <Login set_state mnemonic set_mnemonic/>
                </Show>
            </div>
        </div>
    }
}