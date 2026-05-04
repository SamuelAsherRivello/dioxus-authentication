use dioxus::prelude::*;

use crate::prompt::MessagePrompt;
use crate::service::{
    AuthenticationService, AuthenticationSessionConfig, AuthenticationStatus,
    DEFAULT_PASSKEY_EXPIRATION_HOURS,
};

const AUTHENTICATION_CSS: Asset = asset!("/assets/authentication.css");
const AUTH_CARD_CLASS: &str = "auth-card";
const AUTH_HEADER_CLASS: &str = "auth-card__header";
const AUTH_TITLE_CLASS: &str = "auth-card__title";
const AUTH_SUBTITLE_CLASS: &str = "auth-card__subtitle";
const AUTH_DETAIL_CLASS: &str = "auth-card__detail";
const AUTH_ACTIONS_CLASS: &str = "auth-card__actions";
const AUTH_PRIMARY_BUTTON_CLASS: &str = "auth-button auth-button--primary";
const AUTH_SECONDARY_BUTTON_CLASS: &str = "auth-button auth-button--secondary";

/// Renders the reusable passkey-first authentication card.
///
/// The component owns only UI state: current auth status, busy state, and the
/// prompt message. All platform-specific passkey work stays behind
/// `AuthenticationService` so this component can compile for web and desktop.
#[component]
pub fn AuthenticationComponent(expiration_hours: Option<u32>) -> Element {
    let expiration_hours = expiration_hours.unwrap_or(DEFAULT_PASSKEY_EXPIRATION_HOURS);
    let session_config = AuthenticationSessionConfig::hours(expiration_hours);
    let mut status = use_signal(|| None::<Result<AuthenticationStatus, String>>);
    let mut is_busy = use_signal(|| true);
    let mut prompt = use_signal(|| None::<String>);

    use_future(move || async move {
        status.set(Some(AuthenticationService::status(session_config).await));
        is_busy.set(false);
    });

    let current_status = status();
    let is_authenticated = current_status
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|status| status.is_authenticated)
        .unwrap_or(false);
    let passkey_supported = current_status
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|status| status.passkey_supported)
        .unwrap_or(false);
    let status_text = match current_status {
        Some(Ok(status)) => status.message,
        Some(Err(message)) => message,
        None => "Checking authentication...".to_string(),
    };
    let detail_text = if is_authenticated {
        format!("Passkey expires after {expiration_hours} hours.")
    } else if passkey_supported {
        "Use your passkey to sign in securely.".to_string()
    } else {
        "Passkey sign-in is unavailable in this environment.".to_string()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: AUTHENTICATION_CSS }
        div { class: AUTH_CARD_CLASS,
            div { class: AUTH_HEADER_CLASS,
                h2 { class: AUTH_TITLE_CLASS, "Authentication" }
                p { class: AUTH_SUBTITLE_CLASS, "{status_text}" }
            }
            p { class: AUTH_DETAIL_CLASS, "{detail_text}" }
            div { class: AUTH_ACTIONS_CLASS,
                button {
                    class: AUTH_PRIMARY_BUTTON_CLASS,
                    r#type: "button",
                    disabled: is_busy() || is_authenticated || !passkey_supported,
                    onclick: move |_| {
                        is_busy.set(true);
                        spawn(async move {
                            match AuthenticationService::login(session_config).await {
                                Ok(next_status) => {
                                    status.set(Some(Ok(next_status)));
                                    prompt.set(Some("You are now logged in".to_string()));
                                }
                                Err(message) => {
                                    status.set(Some(Err(message.clone())));
                                    prompt.set(Some(message));
                                }
                            }
                            is_busy.set(false);
                        });
                    },
                    "Log In"
                }
                button {
                    class: AUTH_SECONDARY_BUTTON_CLASS,
                    r#type: "button",
                    disabled: is_busy() || !is_authenticated,
                    onclick: move |_| {
                        is_busy.set(true);
                        spawn(async move {
                            match AuthenticationService::logout().await {
                                Ok(next_status) => {
                                    status.set(Some(Ok(next_status)));
                                    prompt.set(Some("You are now logged out".to_string()));
                                }
                                Err(message) => {
                                    status.set(Some(Err(message.clone())));
                                    prompt.set(Some(message));
                                }
                            }
                            is_busy.set(false);
                        });
                    },
                    "Log Out"
                }
            }
        }
        MessagePrompt { message: prompt }
    }
}
