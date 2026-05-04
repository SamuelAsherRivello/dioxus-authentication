use dioxus::prelude::*;

const PAGE_CLASS: &str = "template-page";
const TITLE_CLASS: &str = "template-page__title";
const DATA_CLASS: &str = "template-page__data";
const BODY_CLASS: &str = "template-page__body";
const BODY_TEXT_CLASS: &str = "template-page__body-text";
const README_LINK_CLASS: &str = "template-page__readme-link";

#[component]
pub fn TemplatePage(
    title: String,
    body_01: String,
    body_02: String,
    body_03: String,
    data_text: Option<String>,
    #[props(default = None)] readme_text: Option<String>,
    #[props(default = None)] readme_href: Option<String>,
) -> Element {
    rsx! {
        main { class: PAGE_CLASS,
            h1 { class: TITLE_CLASS, "{title}" }
            if let Some(data_text) = data_text {
                p { class: DATA_CLASS, "{data_text}" }
            }
            div { class: BODY_CLASS,
                p { class: BODY_TEXT_CLASS, "{body_01}" }
                p { class: BODY_TEXT_CLASS, "{body_02}" }
                p { class: BODY_TEXT_CLASS, "{body_03}" }
            }
            if let (Some(readme_text), Some(readme_href)) = (readme_text, readme_href) {
                a {
                    class: README_LINK_CLASS,
                    href: readme_href,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{readme_text}"
                }
            }
        }
    }
}
