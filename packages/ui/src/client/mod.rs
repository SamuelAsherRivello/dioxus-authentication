use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    HomePage {},
}

#[component]
fn AppLayout() -> Element {
    let theme = use_signal(services::storage_service::load_theme);
    let language = use_signal(services::storage_service::load_language);
    let initial_language = language();
    use_init_i18n(|| services::localization_service::config(initial_language));

    use_context_provider(|| theme);
    use_context_provider(|| language);

    let shell_class = format!("app-shell {}", theme().class_name());

    rsx! {
        div { class: "{shell_class}",
            ErrorBoundary {
                handle_error: |error_context: ErrorContext| rsx! {
                    AppErrorFallback { error_context }
                },
                PageHeader {}
                PageStack {}
                PageFooter {}
            }
        }
    }
}

#[component]
fn PageStack() -> Element {
    rsx! {
        div { class: "page-stack",
            Page { route: Route::HomePage {}, will_preload: true, HomePage {} }
        }
    }
}

mod app;
pub use app::App;

pub mod pages {
    pub mod home_page;
}
pub use pages::home_page::HomePage;

pub mod components {
    pub mod app_error;
    pub mod developer_tools;
    pub mod page;
    pub mod page_footer;
    pub mod page_header;
    pub mod prompt;
}
pub use components::app_error::AppErrorFallback;
pub use components::developer_tools::DeveloperTools;
pub use components::page::Page;
pub use components::page_footer::PageFooter;
pub use components::page_header::PageHeader;

pub mod services;
pub use services::localization_service::default_locale;
pub use services::storage_service::Theme;
