use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

use crate::{
    AuthenticationMethod, AuthenticationProvider, AuthenticationSessionConfig,
    AuthenticationStatus,
};

const AUTHENTICATION_CSS: Asset = asset!("/assets/styles/authentication_styles.css");
const AUTH_CARD_CLASS: &str = "auth-card";
const AUTH_HEADER_CLASS: &str = "auth-card__header";
const AUTH_TITLE_CLASS: &str = "auth-card__title";
const AUTH_STATUS_CLASS: &str = "auth-card__status";
const AUTH_STATUS_LABEL_CLASS: &str = "auth-card__status-label";
const AUTH_STATUS_BADGE_CLASS: &str = "auth-card__status-badge";
const AUTH_STATUS_BADGE_ACTIVE_CLASS: &str =
    "auth-card__status-badge auth-card__status-badge--active";
const AUTH_STATUS_BADGE_ERROR_CLASS: &str =
    "auth-card__status-badge auth-card__status-badge--error";
const AUTH_INSTRUCTION_CLASS: &str = "auth-card__instruction";
const AUTH_PROVIDER_BAR_CLASS: &str = "auth-card__provider-bar";
const AUTH_PROVIDER_LABEL_CLASS: &str = "auth-card__provider-label";
const AUTH_PROVIDER_TAB_CLASS: &str = "auth-card__provider-tab";
const AUTH_ACTIONS_CLASS: &str = "auth-card__button-menu";
const AUTH_PRIMARY_BUTTON_CLASS: &str = "auth-button auth-button--primary";
const AUTH_SECONDARY_BUTTON_CLASS: &str = "auth-button auth-button--secondary";

#[derive(Clone, PartialEq)]
pub struct AuthenticationViewConfig {
    pub session_config: AuthenticationSessionConfig,
    pub providers: Vec<AuthenticationProvider>,
    pub status: Signal<Option<Result<AuthenticationStatus, String>>>,
    pub is_busy: Signal<bool>,
    pub on_login: EventHandler<String>,
    pub on_logout: EventHandler<()>,
}

impl AuthenticationViewConfig {
    pub fn new(
        session_config: AuthenticationSessionConfig,
        providers: Vec<AuthenticationProvider>,
        status: Signal<Option<Result<AuthenticationStatus, String>>>,
        is_busy: Signal<bool>,
        on_login: EventHandler<String>,
        on_logout: EventHandler<()>,
    ) -> Self {
        Self {
            session_config,
            providers,
            status,
            is_busy,
            on_login,
            on_logout,
        }
    }
}

#[component]
pub fn AuthenticationView(config: AuthenticationViewConfig) -> Element {
    let status = config.status;
    let is_busy = config.is_busy;
    let expiration_hours = config.session_config.expiration_hours();
    let providers = config.providers;
    let initial_provider_id = providers
        .first()
        .map(|provider| provider.id().to_string())
        .unwrap_or_default();
    let mut selected_provider_id = use_signal(|| initial_provider_id);
    let on_login = config.on_login;
    let on_logout = config.on_logout;

    let current_status = status();
    let is_authenticated = current_status
        .as_ref()
        .and_then(|result: &Result<AuthenticationStatus, String>| result.as_ref().ok())
        .map(|status| status.is_authenticated)
        .unwrap_or(false);
    let login_supported = current_status
        .as_ref()
        .and_then(|result: &Result<AuthenticationStatus, String>| result.as_ref().ok())
        .map(|status| status.login_supported)
        .unwrap_or(false);
    let status_text = match current_status {
        Some(Ok(ref status)) => localized_status_text(status),
        Some(Err(_)) => t!("auth-status.error"),
        None => t!("auth-status.checking"),
    };
    let detail_text = match current_status {
        Some(Ok(ref status)) => localized_detail_text(status, expiration_hours),
        Some(Err(_)) => t!("auth-detail.error"),
        None => t!("auth-detail.checking"),
    };
    let status_badge_class = status_badge_class(&current_status);
    let has_provider = !selected_provider_id().is_empty();

    rsx! {
        document::Link { rel: "stylesheet", href: AUTHENTICATION_CSS }
        div { class: AUTH_CARD_CLASS,
            div { class: AUTH_HEADER_CLASS,
                h2 { class: AUTH_TITLE_CLASS, {t!("auth.title")} }
            }
            p { class: AUTH_STATUS_CLASS,
                span { class: AUTH_STATUS_LABEL_CLASS, "{t!(\"auth-status.label\")}: " }
                span { class: status_badge_class, "{status_text}" }
            }
            Tabs {
                default_value: selected_provider_id(),
                horizontal: true,
                disabled: is_busy() || is_authenticated,
                on_value_change: move |provider_id: String| selected_provider_id.set(provider_id),
                TabList { class: AUTH_PROVIDER_BAR_CLASS,
                    span { class: AUTH_PROVIDER_LABEL_CLASS, "{t!(\"auth-provider.label\")}: " }
                    for (index, provider) in providers.into_iter().enumerate() {
                        TabTrigger {
                            class: AUTH_PROVIDER_TAB_CLASS,
                            index,
                            value: provider.id().to_string(),
                            "{provider.display_text()}"
                        }
                    }
                }
                TabContent {
                    class: "auth-card__provider-content",
                    index: 0usize,
                    value: selected_provider_id(),
                    p { class: AUTH_INSTRUCTION_CLASS, "{detail_text}" }
                    div { class: AUTH_ACTIONS_CLASS,
                        button {
                            class: AUTH_PRIMARY_BUTTON_CLASS,
                            r#type: "button",
                            disabled: is_busy() || is_authenticated || !login_supported || !has_provider,
                            onclick: move |_| on_login.call(selected_provider_id()),
                            {t!("auth-button.login")}
                        }
                        button {
                            class: AUTH_SECONDARY_BUTTON_CLASS,
                            r#type: "button",
                            disabled: is_busy() || !is_authenticated,
                            onclick: move |_| on_logout.call(()),
                            {t!("auth-button.logout")}
                        }
                    }
                }
            }
        }
    }
}

fn status_badge_class(status: &Option<Result<AuthenticationStatus, String>>) -> &'static str {
    match status {
        Some(Ok(status)) if status.is_authenticated => AUTH_STATUS_BADGE_ACTIVE_CLASS,
        Some(Err(_)) => AUTH_STATUS_BADGE_ERROR_CLASS,
        _ => AUTH_STATUS_BADGE_CLASS,
    }
}

fn localized_status_text(status: &AuthenticationStatus) -> String {
    if status.is_authenticated {
        return t!("auth-status.logged-in");
    }

    if !status.login_supported {
        t!("auth-status.unavailable")
    } else {
        t!("auth-status.logged-out")
    }
}

fn localized_detail_text(status: &AuthenticationStatus, expiration_hours: u32) -> String {
    if status.is_authenticated {
        let method = localized_method_label(status.auth_method);
        if let Some(authenticated_at) = status.authenticated_at.as_deref() {
            return t!(
                "auth-detail.logged-in-session",
                method: method,
                authenticated_at: authenticated_at,
                expiration_hours: expiration_hours
            );
        }

        return t!(
            "auth-detail.session-expires",
            expiration_hours: expiration_hours
        );
    }

    match status.auth_method {
        AuthenticationMethod::WebPasskey if status.passkey_supported => {
            t!("auth-detail.web-passkey")
        }
        AuthenticationMethod::WebPasskey => t!("auth-detail.web-unavailable"),
        AuthenticationMethod::WindowsPasskey if status.passkey_supported => {
            t!("auth-detail.windows-passkey")
        }
        AuthenticationMethod::WindowsPasskey => t!("auth-detail.windows-unavailable"),
        AuthenticationMethod::UnsupportedNative => t!("auth-detail.native-unavailable"),
    }
}

fn localized_method_label(method: AuthenticationMethod) -> String {
    match method {
        AuthenticationMethod::WebPasskey | AuthenticationMethod::UnsupportedNative => {
            t!("auth-method.passkey")
        }
        AuthenticationMethod::WindowsPasskey => t!("auth-method.windows-passkey"),
    }
}
