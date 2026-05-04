use std::fmt;

/// Contract implemented by authentication providers.
pub trait AuthenticationProviderContract {
    fn id(&self) -> &str;
    fn display_text(&self) -> &str;
}

/// Lightweight metadata for rendering and routing provider login actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationProvider {
    pub id: String,
    pub display_text: String,
}

impl AuthenticationProvider {
    pub fn new(id: impl Into<String>, display_text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_text: display_text.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_text(&self) -> &str {
        &self.display_text
    }
}

impl AuthenticationProviderContract for AuthenticationProvider {
    fn id(&self) -> &str {
        &self.id
    }

    fn display_text(&self) -> &str {
        &self.display_text
    }
}

impl fmt::Display for AuthenticationProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub const PASSKEY_PROVIDER_ID: &str = "passkey";

pub fn passkey_provider(display_text: impl Into<String>) -> AuthenticationProvider {
    AuthenticationProvider::new(PASSKEY_PROVIDER_ID, display_text)
}

pub fn is_passkey_provider(provider_id: &str) -> bool {
    provider_id.eq_ignore_ascii_case(PASSKEY_PROVIDER_ID)
}
