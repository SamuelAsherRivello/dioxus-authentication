use dioxus::prelude::*;

use authentication::{current_demo_origin_warning, AuthenticationComponent};

const PAGE_CLASS: &str = "home-page";
const HERO_CLASS: &str = "home-hero";
const TITLE_CLASS: &str = "home-hero__title";
const BODY_TEXT_CLASS: &str = "home-page__body";
const SECTION_CLASS: &str = "home-section";
const SECTION_TITLE_CLASS: &str = "home-section__title";
const DEMO_WARNING_CLASS: &str = "home-demo-warning";
const CODE_BLOCK_CLASS: &str = "home-code";

#[component]
pub fn Page01() -> Element {
    rsx! {
        main { class: PAGE_CLASS,
            section { class: HERO_CLASS,
                h1 { class: TITLE_CLASS, "Dioxus Authentication" }
                p { class: BODY_TEXT_CLASS,
                    "A reusable Dioxus authentication library and demo focused on passkey-first sign in."
                }
            }

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, "Component" }
                p { class: BODY_TEXT_CLASS,
                    "AuthenticationComponent renders the auth status, passkey login, and logout controls while AuthenticationService handles the workflow."
                }
            }

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, "Usage" }
                pre { class: CODE_BLOCK_CLASS,
                    code {
                        span { class: "home-code__keyword", "use" }
                        " dioxus::prelude::*;\n"
                        span { class: "home-code__keyword", "use" }
                        " authentication::"
                        span { class: "home-code__type", "AuthenticationComponent" }
                        ";\n\n"
                        span { class: "home-code__attribute", "#[component]" }
                        "\n"
                        span { class: "home-code__keyword", "fn" }
                        " "
                        span { class: "home-code__function", "Home" }
                        "() -> "
                        span { class: "home-code__type", "Element" }
                        " {{\n    "
                        span { class: "home-code__macro", "rsx!" }
                        " {{\n        "
                        span { class: "home-code__type", "AuthenticationComponent" }
                        " {{ "
                        span { class: "home-code__property", "expiration_hours" }
                        ": "
                        span { class: "home-code__enum", "Some" }
                        "("
                        span { class: "home-code__number", "48" }
                        ") }}\n    }}\n}}"
                    }
                }
            }

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, "Demo" }
                if let Some(warning) = current_demo_origin_warning() {
                    p { class: DEMO_WARNING_CLASS, role: "alert", "{warning}" }
                }
                AuthenticationComponent { expiration_hours: Some(48) }
            }
        }
    }
}
