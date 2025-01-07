use leptos::prelude::*;
use crate::login_register::LoginRegisterState;
use kit::generate_mnemonic::generate_mnemonic;
#[component]
pub fn Register(set_state: WriteSignal<LoginRegisterState>) -> impl IntoView {
    let (mnemonic, set_mnemonic) = signal(generate_mnemonic());
    view! {
        <div id="registerForm" class="space-y-4">
            <h2 class="text-xl font-semibold text-center">Register New Account</h2>
            <div>
                <label for="newMnemonic" class="text-sm font-medium text-center">Your new mnemonic phrase:</label>
                <textarea id="newMnemonic" rows="3" readonly class="mt-1 block w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm">{mnemonic}</textarea>
            </div>
            <button on:click=move |x1| {set_mnemonic.set(generate_mnemonic())} class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500">Generate New Mnemonic</button>
            <button on:click=move |x| {set_state.set(LoginRegisterState::Login)} class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Register</button>
        </div>

        <div id="registerToggle" class="mt-4 text-center text-sm hidden">
            <span>Already have an account?</span>
            <a id="showLogin" class="hover:underline">Login</a>
        </div>
    }
}