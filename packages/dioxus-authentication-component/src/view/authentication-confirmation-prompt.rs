use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};

const AUTHENTICATION_CSS: Asset = asset!("/assets/styles/authentication_styles.css");
const PROMPT_BACKDROP_CLASS: &str = "prompt-backdrop";
const PROMPT_DIALOG_CLASS: &str = "prompt-dialog";
const PROMPT_TITLE_CLASS: &str = "prompt-dialog__title";
const PROMPT_DESCRIPTION_CLASS: &str = "prompt-dialog__body";
const PROMPT_ACTIONS_CLASS: &str = "prompt-dialog__actions";
const PROMPT_BUTTON_CLASS: &str = "prompt-dialog__button";
const PROMPT_PRIMARY_BUTTON_CLASS: &str = "prompt-dialog__button prompt-dialog__button--primary";

#[component]
pub fn AuthenticationConfirmationPrompt(
    mut open: Signal<bool>,
    on_answer: EventHandler<bool>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: AUTHENTICATION_CSS }
        AlertDialogRoot {
            class: PROMPT_BACKDROP_CLASS,
            open: open(),
            on_open_change: move |value| open.set(value),
            AlertDialogContent {
                class: PROMPT_DIALOG_CLASS,
                AlertDialogTitle {
                    class: PROMPT_TITLE_CLASS,
                    {t!("auth-confirmation.title")}
                }
                AlertDialogDescription {
                    class: PROMPT_DESCRIPTION_CLASS,
                    {t!("auth-confirmation.description")}
                }
                AlertDialogActions { class: PROMPT_ACTIONS_CLASS,
                    AlertDialogAction {
                        class: PROMPT_PRIMARY_BUTTON_CLASS,
                        on_click: move |_| {
                            open.set(false);
                            on_answer.call(true);
                        },
                        {t!("auth-confirmation.confirm")}
                    }
                    AlertDialogCancel {
                        class: PROMPT_BUTTON_CLASS,
                        on_click: move |_| {
                            open.set(false);
                            on_answer.call(false);
                        },
                        {t!("auth-confirmation.cancel")}
                    }
                }
            }
        }
    }
}
