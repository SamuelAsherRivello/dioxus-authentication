//! Reusable Dioxus authentication package.

mod localization;
mod providers;
mod services;
mod view;

pub use localization::{
    authentication_locales, default_authentication_locale, AuthenticationLocaleResource,
    AUTH_EN_US, AUTH_ES_MX, AUTH_FR_FR, AUTH_PT_BR,
};
pub use providers::{
    passkey_provider, AuthenticationProvider, AuthenticationProviderContract, PASSKEY_PROVIDER_ID,
};
pub use services::authentication_service::{
    AuthenticationMethod, AuthenticationPasskeyConfig, AuthenticationService,
    AuthenticationSession, AuthenticationSessionConfig, AuthenticationStatus,
    DEFAULT_PASSKEY_APP_ID, DEFAULT_PASSKEY_EXPIRATION_HOURS, DEFAULT_PASSKEY_RELYING_PARTY_NAME,
    DEFAULT_PASSKEY_USER_DISPLAY_NAME, DEFAULT_PASSKEY_USER_NAME,
};
pub use view::authentication_confirmation_prompt::AuthenticationConfirmationPrompt;
pub use view::authentication_view::{AuthenticationView, AuthenticationViewConfig};

/// Common imports for applications embedding the authentication component.
pub mod prelude {
    pub use crate::{
        authentication_locales, default_authentication_locale, passkey_provider,
        AuthenticationConfirmationPrompt, AuthenticationLocaleResource, AuthenticationMethod,
        AuthenticationPasskeyConfig, AuthenticationProvider, AuthenticationProviderContract,
        AuthenticationService, AuthenticationSession, AuthenticationSessionConfig,
        AuthenticationStatus, AuthenticationView, AuthenticationViewConfig, AUTH_EN_US, AUTH_ES_MX,
        AUTH_FR_FR, AUTH_PT_BR, DEFAULT_PASSKEY_APP_ID, DEFAULT_PASSKEY_EXPIRATION_HOURS,
        DEFAULT_PASSKEY_RELYING_PARTY_NAME, DEFAULT_PASSKEY_USER_DISPLAY_NAME,
        DEFAULT_PASSKEY_USER_NAME, PASSKEY_PROVIDER_ID,
    };
}
