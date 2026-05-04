use dioxus::prelude::*;
use dioxus_i18n::t;

const FOOTER_CLASS: &str = "page-footer";

#[component]
pub fn PageFooter() -> Element {
    rsx! {
        footer { class: FOOTER_CLASS,
            {t!("footer-rights")}
        }
    }
}
