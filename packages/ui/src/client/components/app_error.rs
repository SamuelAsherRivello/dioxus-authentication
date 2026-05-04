use dioxus::prelude::*;
use dioxus_i18n::t;

const ERROR_PAGE_CLASS: &str = "app-error";
const ERROR_CARD_CLASS: &str = "app-error__card";
const ERROR_TITLE_CLASS: &str = "app-error__title";
const ERROR_MESSAGE_CLASS: &str = "app-error__message";
const ERROR_BUTTON_CLASS: &str = "app-error__button";

#[component]
pub fn AppErrorFallback(error_context: ErrorContext) -> Element {
    let error_message = error_context
        .error()
        .map(|error| error.to_string())
        .unwrap_or_else(|| t!("app-error-default-message"));

    rsx! {
        main { class: ERROR_PAGE_CLASS,
            section { class: ERROR_CARD_CLASS,
                h1 { class: ERROR_TITLE_CLASS, {t!("app-error-title")} }
                p { class: ERROR_MESSAGE_CLASS, "{error_message}" }
                button {
                    class: ERROR_BUTTON_CLASS,
                    r#type: "button",
                    onclick: move |_| {
                        error_context.clear_errors();
                    },
                    {t!("app-error-retry")}
                }
            }
        }
    }
}
