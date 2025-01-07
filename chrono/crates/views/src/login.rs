use leptos::prelude::*;
use crate::login_register::LoginRegisterState;

#[component]
pub fn Login(set_state: WriteSignal<LoginRegisterState>) -> impl IntoView {
    view! {
        <h1 class="text-3xl font-bold mb-6 text-center">"Let's get started"</h1>

        <div id="loginForm" class="space-y-4">
            <h2 class="text-xl font-semibold text-center">Login with Mnemonic</h2>
            <div>
                <label for="loginMnemonic" class="block text-sm font-medium text-center">Enter your mnemonic phrase:</label>
                <textarea id="loginMnemonic" rows="3" required class="mt-1 block w-full px-3 py-2  border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"></textarea>
            </div>
            <button on:click=move |x| {set_state.set(LoginRegisterState::Register)} class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Login</button>
        </div>

        <div id="loginToggle" class="mt-4 text-center text-sm">
            <span>"Don't have an account?"</span>
            <a on:click=move |x| {set_state.set(LoginRegisterState::Register)}  class="text-blue-600 dark:text-blue-400 hover:underline">Register</a>
        </div>
    }
}