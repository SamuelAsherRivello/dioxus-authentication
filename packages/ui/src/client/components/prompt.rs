use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};

const PROMPT_BACKDROP_CLASS: &str = "prompt-backdrop";
const PROMPT_DIALOG_CLASS: &str = "prompt-dialog";
const PROMPT_TITLE_CLASS: &str = "prompt-dialog__title";
const PROMPT_DESCRIPTION_CLASS: &str = "screen-reader-only";
const PROMPT_ACTIONS_CLASS: &str = "prompt-dialog__actions";
const PROMPT_BUTTON_CLASS: &str = "prompt-dialog__button";
const PROMPT_PRIMARY_BUTTON_CLASS: &str = "prompt-dialog__button prompt-dialog__button--primary";

#[component]
pub fn ConfirmationPrompt(mut open: Signal<bool>, on_answer: EventHandler<bool>) -> Element {
    rsx! {
        AlertDialogRoot {
            class: PROMPT_BACKDROP_CLASS,
            open: open(),
            on_open_change: move |value| open.set(value),
            AlertDialogContent {
                class: PROMPT_DIALOG_CLASS,
                AlertDialogTitle {
                    class: PROMPT_TITLE_CLASS,
                    "Are you sure?"
                }
                AlertDialogDescription {
                    class: PROMPT_DESCRIPTION_CLASS,
                    "Confirm or cancel this prompt."
                }
                AlertDialogActions { class: PROMPT_ACTIONS_CLASS,
                    AlertDialogAction {
                        class: PROMPT_PRIMARY_BUTTON_CLASS,
                        on_click: move |_| on_answer.call(true),
                        "Ok"
                    }
                    AlertDialogCancel {
                        class: PROMPT_BUTTON_CLASS,
                        on_click: move |_| on_answer.call(false),
                        "Cancel"
                    }
                }
            }
        }
    }
}

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
                    "{prompt_message}"
                }
                AlertDialogDescription {
                    class: PROMPT_DESCRIPTION_CLASS,
                    "{prompt_message}"
                }
                AlertDialogActions { class: PROMPT_ACTIONS_CLASS,
                    AlertDialogAction {
                        class: PROMPT_PRIMARY_BUTTON_CLASS,
                        on_click: move |_| message.set(None),
                        "Ok"
                    }
                }
            }
        }
    }
}
