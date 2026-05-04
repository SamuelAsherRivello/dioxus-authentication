use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_primitives::aspect_ratio::AspectRatio;

use crate::client::components::prompt::ConfirmationPrompt;
use crate::client::components::toast::{Toast, ToastTone};
use crate::client::pages::template_page::TemplatePage;

const MOBILE_FRAME_WRAP_CLASS: &str = "mobile-frame-wrap";
const MOBILE_FRAME_CLASS: &str = "mobile-frame";
const OPEN_PROMPT_BUTTON_CLASS: &str = "prompt-demo-button";

#[component]
pub fn Page03() -> Element {
    let mut is_prompt_open = use_signal(|| false);
    let mut toast = use_context::<Signal<Option<Toast>>>();
    let mut toast_sequence = use_signal(|| 20_000_u64);

    rsx! {
        TemplatePage {
            title: t!("page-03-title"),
            body_01: t!("page-03-body-01"),
            body_02: t!("page-03-body-02"),
            body_03: t!("page-03-body-03"),
            data_text: None,
        }
        div { class: MOBILE_FRAME_WRAP_CLASS,
            AspectRatio { ratio: 9.0 / 16.0,
                div { class: MOBILE_FRAME_CLASS,
                    button {
                        class: OPEN_PROMPT_BUTTON_CLASS,
                        r#type: "button",
                        onclick: move |_| is_prompt_open.set(true),
                        "Open Prompt"
                    }
                }
            }
        }
        ConfirmationPrompt {
            open: is_prompt_open,
            on_answer: move |confirmed| {
                let next_id = *toast_sequence.peek() + 1;
                toast_sequence.set(next_id);
                toast.set(Some(Toast {
                    id: next_id,
                    message: if confirmed {
                        "Ok selected".to_string()
                    } else {
                        "Cancel selected".to_string()
                    },
                    tone: if confirmed {
                        ToastTone::Success
                    } else {
                        ToastTone::Info
                    },
                }));
            }
        }
    }
}
