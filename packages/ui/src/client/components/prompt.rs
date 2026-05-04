use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogContent, AlertDialogDescription,
    AlertDialogRoot, AlertDialogTitle,
};

const PROMPT_BACKDROP_CLASS: &str = "prompt-backdrop";
const PROMPT_DIALOG_CLASS: &str = "prompt-dialog";
const PROMPT_TITLE_CLASS: &str = "prompt-dialog__title";
const PROMPT_DESCRIPTION_CLASS: &str = "prompt-dialog__body";
const PROMPT_ACTIONS_CLASS: &str = "prompt-dialog__actions";
const PROMPT_PRIMARY_BUTTON_CLASS: &str = "prompt-dialog__button prompt-dialog__button--primary";

#[component]
pub fn MessagePrompt(mut message: Signal<Option<String>>) -> Element {
    let is_open = message().is_some();
    let prompt_message = message().unwrap_or_default();

    rsx! {
        AlertDialogRoot {
            class: PROMPT_BACKDROP_CLASS,
            open: is_open,
            on_open_change: move |value: bool| {
                if !value {
                    message.set(None);
                }
            },
            AlertDialogContent {
                class: PROMPT_DIALOG_CLASS,
                AlertDialogTitle {
                    class: PROMPT_TITLE_CLASS,
                    {t!("auth-message.title")}
                }
                AlertDialogDescription {
                    class: PROMPT_DESCRIPTION_CLASS,
                    "{prompt_message}"
                }
                AlertDialogActions { class: PROMPT_ACTIONS_CLASS,
                    AlertDialogAction {
                        class: PROMPT_PRIMARY_BUTTON_CLASS,
                        on_click: move |_| message.set(None),
                        {t!("auth-message.ok")}
                    }
                }
            }
        }
    }
}
