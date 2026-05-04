use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::client::components::developer_tools::DeveloperTools;
use crate::client::components::toast::{Toast, ToastRegion};
use crate::client::Route;

const HEADER_CLASS: &str = "page-header";
const NAV_CLASS: &str = "page-header__nav";
const PAGE_LIST_CLASS: &str = "page-header__pages";
const PAGE_LINK_BASE_CLASS: &str = "page-header__page-link";
const PAGE_LINK_ACTIVE_CLASS: &str = "page-header__page-link page-header__page-link--active";

#[component]
pub fn PageHeader() -> Element {
    let active_route = use_route::<Route>();
    let is_page01 = active_route == (Route::Page01 {});
    let toast = use_context::<Signal<Option<Toast>>>();

    rsx! {
        header { id: "page-header", class: HEADER_CLASS,
            nav { class: NAV_CLASS,
                div { class: PAGE_LIST_CLASS,
                    Link {
                        class: if is_page01 {
                            PAGE_LINK_ACTIVE_CLASS
                        } else {
                            PAGE_LINK_BASE_CLASS
                        },
                        to: Route::Page01 {},
                        aria_current: if is_page01 { "page" } else { "false" },
                        aria_label: t!("view-page-01"),
                        "data-tooltip": t!("view-page-01"),
                        span { class: "page-header__label-full", {t!("nav-page-01")} }
                        span { class: "page-header__label-short", {t!("nav-page-01-short")} }
                    }
                }
                DeveloperTools {}
            }
            ToastRegion { toast }
        }
    }
}
