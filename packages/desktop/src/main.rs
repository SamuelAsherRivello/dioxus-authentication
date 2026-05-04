use dioxus::prelude::*;

use ui::App as UiApp;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const APP_TITLE: &str = "Dioxus Authentication";

fn main() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title(app_title())
        .with_always_on_top(false);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_background_color((8, 11, 16, 255)),
        )
        .launch(App);
}

fn app_title() -> String {
    std::env::var("DIOXUS_APP_TITLE").unwrap_or_else(|_| APP_TITLE.to_string())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        UiApp {}
    }
}
