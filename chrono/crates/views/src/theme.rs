use leptos::prelude::*;
use display_enum::Display;
#[derive(Display,Clone)]
pub enum Theme {
    Light,
    Dark,
}
#[component]
pub fn ChangeTheme() -> impl IntoView {
    let (theme, set_theme) = signal(Theme::Light);
    let event = move |_| {
        let mut doc = document();
        match theme.get() {
            Theme::Light => set_theme.set(Theme::Dark),
            Theme::Dark => set_theme.set(Theme::Light),
        }
        let _ = doc.get_element_by_id("theme").unwrap().set_attribute("class", match theme.get() {
            Theme::Light => "bg-white text-black",
            Theme::Dark => "bg-black text-white",
        });
    };
    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl">{ move || theme.get().to_string() }</h1>
        </div>
        <div class="container mx-auto p-4">
            <h1 class="text-2xl">主题切换</h1>
            <button on:click=move |e| {event(e)} id="theme-toggle" class="bg-blue-500 px-4 py-2 rounded">切换主题</button>
        </div>
    }
}