use leptos::prelude::*;
use kit::password::hash_password;
use crate::root::ShowSelect;

#[component]
pub fn SetPassword(set_show_select: WriteSignal<ShowSelect>, password: ReadSignal<String>, set_password: WriteSignal<String>) -> impl IntoView {
    let (re_password, set_re_password) = signal("".to_string());
    let (is_same, set_is_same) = signal(true);
    let event = move |_| {
        if password.get().is_empty() {
            return;
        }
        if password.get() == re_password.get() {
            set_password.set(hash_password(&password.get()));

            set_show_select.set(ShowSelect::Index);
            set_is_same.set(true);
        } else {
            set_is_same.set(false);
        }
    };
    view! {
        <h1 class="text-3xl font-bold mb-6 text-center">"Set Password"</h1>

        <div class="space-y-4">
            <div>
                <label for="setPassword" class="block text-sm font-medium text-center">Enter your password</label>
                <input on:input:target=move |ev| {set_password.set(ev.target().value())} prop:value=password id="setPassword" type="password" required class="mt-1 block w-full px-3 py-2  border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"/>
            </div>
            <div>
                <Show when=move || is_same.get()
                    fallback=|| {
                        view!{<label for="rePassword" class="block text-sm font-medium text-center text-rose-600">"Passwords do not match"</label>}
                    }>
                    <label for="rePassword" class="block text-sm font-medium text-center">Enter your password again</label>
                </Show>
                <input on:input:target=move |ev| {set_re_password.set(ev.target().value())} prop:value=re_password id="rePassword" type="password"  required class="mt-1 block w-full px-3 py-2  border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"/>
            </div>
        </div>

        <div class="mt-4 text-center text-sm">
            <button on:click=event class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Confirm</button>
        </div>
    }
}