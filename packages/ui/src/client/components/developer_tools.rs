use dioxus::prelude::*;
use dioxus_i18n::unic_langid::LanguageIdentifier;
use dioxus_i18n::{prelude::i18n, t};

use crate::client::services::localization_service::{
    locale_flag_asset, locale_identifier, supported_locales,
};
use crate::client::services::storage_service::{save_language, save_theme, Theme};

const TOOLS_CLASS: &str = "developer-tools";
const TOOL_GROUP_CLASS: &str = "developer-tools__group";
const TOOL_LABEL_CLASS: &str = "developer-tools__label";
const TOOL_CONTROLS_CLASS: &str = "developer-tools__controls";
const ICON_LINK_CLASS: &str = "developer-tools__icon-link";
const GITHUB_ICON_CLASS: &str = "developer-tools__github-icon";
const TOOL_BUTTON_CLASS: &str = "developer-tools__button";
const TOOL_BUTTON_TEXT_CLASS: &str = "developer-tools__button-text";
const TOOL_ICON_CLASS: &str = "developer-tools__icon";
const LANGUAGE_MENU_CLASS: &str = "language-menu";
const LANGUAGE_BUTTON_CLASS: &str = "language-menu__button";
const FLAG_CLASS: &str = "language-menu__flag";
const LANGUAGE_CARET_CLASS: &str = "language-menu__caret";
const LANGUAGE_OPTIONS_CLASS: &str = "language-menu__options";
const LANGUAGE_OPTION_CLASS: &str = "language-menu__option";
const LANGUAGE_OPTION_ACTIVE_CLASS: &str = "language-menu__option language-menu__option--active";

#[component]
pub fn DeveloperTools() -> Element {
    let mut theme = use_context::<Signal<Theme>>();
    let mut language = use_context::<Signal<LanguageIdentifier>>();
    let selected_language = language();
    let mut language_menu_open = use_signal(|| false);
    let mut i18n = i18n();

    rsx! {
        div { class: TOOLS_CLASS,
            div { class: TOOL_GROUP_CLASS,
                span { class: TOOL_LABEL_CLASS, {t!("dev-tools")} }
                div { class: TOOL_CONTROLS_CLASS,
                    a {
                    class: ICON_LINK_CLASS,
                    href: "https://github.com/SamuelAsherRivello/dioxus-authentication",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    aria_label: t!("open-github-repository"),
                    "data-tooltip": t!("open-github-repository"),
                    svg {
                        class: GITHUB_ICON_CLASS,
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        path {
                            fill: "currentColor",
                            d: "M12 .5C5.65.5.5 5.65.5 12c0 5.1 3.29 9.42 7.86 10.95.58.11.79-.25.79-.56v-2.16c-3.2.7-3.88-1.36-3.88-1.36-.52-1.33-1.28-1.68-1.28-1.68-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.76 2.7 1.25 3.36.96.1-.75.4-1.25.73-1.54-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.29 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18A11.1 11.1 0 0 1 12 6.06c.98 0 1.96.13 2.88.39 2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.8 1.19 1.83 1.19 3.09 0 4.42-2.69 5.39-5.25 5.68.41.35.78 1.05.78 2.12v3.18c0 .31.21.67.8.56A11.51 11.51 0 0 0 23.5 12C23.5 5.65 18.35.5 12 .5Z",
                        }
                    }
                    }
                }
            }
            button {
                    class: TOOL_BUTTON_CLASS,
                    r#type: "button",
                    aria_label: t!("toggle-theme"),
                    "data-tooltip": t!("toggle-theme"),
                    onclick: move |_| {
                        let next_theme = theme.peek().toggled();
                        theme.set(next_theme);
                        save_theme(next_theme);
                    },
                    svg {
                        class: TOOL_ICON_CLASS,
                        width: "18",
                        height: "18",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                    }
                    span { class: TOOL_BUTTON_TEXT_CLASS, {t!("theme")} }
                }
            div { class: LANGUAGE_MENU_CLASS,
                button {
                        class: LANGUAGE_BUTTON_CLASS,
                        r#type: "button",
                        aria_label: t!("language-selector"),
                        "data-tooltip": t!("language-selector"),
                        aria_expanded: "{language_menu_open()}",
                        onclick: move |_| {
                            let is_open = *language_menu_open.peek();
                            language_menu_open.set(!is_open);
                        },
                        img {
                            class: FLAG_CLASS,
                            src: locale_flag_asset(&selected_language),
                            width: "24",
                            height: "16",
                            alt: "",
                        }
                        span { class: LANGUAGE_CARET_CLASS, "▾" }
                    }
                if language_menu_open() {
                    div { class: LANGUAGE_OPTIONS_CLASS,
                        for option_language in supported_locales().iter() {
                            button {
                                    class: if locale_identifier(option_language) == selected_language {
                                        LANGUAGE_OPTION_ACTIVE_CLASS
                                    } else {
                                        LANGUAGE_OPTION_CLASS
                                    },
                                    r#type: "button",
                                    aria_label: match option_language.label_key() {
                                        "language-en" => t!("language-en"),
                                        "language-es" => t!("language-es"),
                                        "language-pt" => t!("language-pt"),
                                        "language-fr" => t!("language-fr"),
                                        _ => t!("language-en"),
                                    },
                                    "data-tooltip": match option_language.label_key() {
                                        "language-en" => t!("language-en"),
                                        "language-es" => t!("language-es"),
                                        "language-pt" => t!("language-pt"),
                                        "language-fr" => t!("language-fr"),
                                        _ => t!("language-en"),
                                    },
                                    onclick: move |_| {
                                        let language_key = locale_identifier(option_language);
                                        language.set(language_key.clone());
                                        i18n.set_language(language_key.clone());
                                        save_language(language_key);
                                        language_menu_open.set(false);
                                    },
                                    img {
                                        class: FLAG_CLASS,
                                        src: locale_flag_asset(&locale_identifier(option_language)),
                                        width: "24",
                                        height: "16",
                                        alt: "",
                                    }
                            }
                        }
                    }
                }
            }
        }
    }
}
