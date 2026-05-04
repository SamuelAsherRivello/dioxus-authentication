use serde::{Deserialize, Serialize};

/// Default lifetime for the local demo passkey session.
///
/// The browser still owns the passkey credential. This value only controls how
/// long the demo app keeps its local authenticated session marker.
pub const DEFAULT_PASSKEY_EXPIRATION_HOURS: u32 = 48;

/// Current authentication state rendered by `AuthenticationComponent`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthenticationStatus {
    /// `true` when the local demo session exists and has not expired.
    pub is_authenticated: bool,
    /// `true` when the current renderer exposes the browser WebAuthn API.
    pub passkey_supported: bool,
    /// Browser-local display timestamp for the active demo session.
    pub authenticated_at: Option<String>,
    /// User-facing status text derived from the backend state.
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct AuthBackendStatus {
    is_authenticated: bool,
    passkey_supported: bool,
    authenticated_at: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthenticationSessionConfig {
    expiration_hours: u32,
}

impl AuthenticationSessionConfig {
    /// Creates a session config, clamping zero-hour values to one hour.
    pub fn hours(expiration_hours: u32) -> Self {
        Self {
            expiration_hours: expiration_hours.max(1),
        }
    }

    /// Returns the configured local demo session lifetime in hours.
    pub fn expiration_hours(self) -> u32 {
        self.expiration_hours
    }
}

impl Default for AuthenticationSessionConfig {
    fn default() -> Self {
        Self::hours(DEFAULT_PASSKEY_EXPIRATION_HOURS)
    }
}

pub struct AuthenticationService;

impl AuthenticationService {
    /// Localhost origin used for reliable browser WebAuthn testing.
    ///
    /// WebAuthn requires a secure context. Browsers treat localhost as secure,
    /// but the same app served from a LAN IP is not accepted for this demo.
    pub const REQUIRED_DEMO_ORIGIN: &'static str = "http://localhost:8080";

    /// Reads the current authentication state for the active target.
    ///
    /// Web builds inspect the local WebAuthn-backed session. Desktop builds
    /// return an explicit unsupported state because the desktop webview does
    /// not expose a real browser passkey prompt.
    pub async fn status(
        config: AuthenticationSessionConfig,
    ) -> Result<AuthenticationStatus, String> {
        let status = auth_backend_status(config).await?;

        Ok(AuthenticationStatus {
            is_authenticated: status.is_authenticated,
            passkey_supported: status.passkey_supported,
            authenticated_at: status.authenticated_at.clone(),
            message: status_message(
                status.is_authenticated,
                status.passkey_supported,
                status.authenticated_at.as_deref(),
            ),
        })
    }

    /// Starts the login flow and returns the refreshed authentication state.
    ///
    /// In web builds this creates a local demo credential when needed, asks the
    /// browser to authenticate it, then writes a timestamped local session. In
    /// desktop builds this returns an unsupported error.
    pub async fn login(
        config: AuthenticationSessionConfig,
    ) -> Result<AuthenticationStatus, String> {
        auth_backend_login().await?;
        Self::status(config).await
    }

    /// Clears the local demo session and returns the refreshed state.
    pub async fn logout() -> Result<AuthenticationStatus, String> {
        auth_backend_logout().await?;
        Self::status(AuthenticationSessionConfig::default()).await
    }
}

/// Reports whether the app is being served from the required passkey origin.
pub fn demo_origin_warning(current_origin: &str, required_origin: &str) -> Option<String> {
    if current_origin == required_origin {
        None
    } else {
        Some(format!(
            "You are running app from {current_origin} but you must run it from {required_origin}."
        ))
    }
}

#[cfg(target_arch = "wasm32")]
pub fn current_demo_origin_warning() -> Option<String> {
    let current_origin = web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .unwrap_or_else(|| "an unknown origin".to_string());

    demo_origin_warning(&current_origin, AuthenticationService::REQUIRED_DEMO_ORIGIN)
}

#[cfg(not(target_arch = "wasm32"))]
/// Desktop builds do not need an origin warning because passkey login is
/// intentionally disabled there.
pub fn current_demo_origin_warning() -> Option<String> {
    None
}

fn status_message(
    is_authenticated: bool,
    passkey_supported: bool,
    authenticated_at: Option<&str>,
) -> String {
    if is_authenticated {
        match authenticated_at {
            Some(authenticated_at) => {
                format!("Logged in via passkey at {authenticated_at}")
            }
            None => "You are already logged in via passkey".to_string(),
        }
    } else if !passkey_supported {
        "Desktop passkey login is unavailable. Use the web app on localhost for passkey testing."
            .to_string()
    } else {
        "You are not logged in.".to_string()
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn passkey_api_surface_error(
    is_secure_context: bool,
    has_credentials: bool,
    create_is_function: bool,
    get_is_function: bool,
) -> Option<&'static str> {
    if !is_secure_context {
        return Some("Passkeys require a secure browser context such as localhost or HTTPS.");
    }

    if !has_credentials || !create_is_function || !get_is_function {
        return Some("Passkey API is unavailable in this renderer.");
    }

    None
}

#[cfg(target_arch = "wasm32")]
async fn auth_backend_status(
    config: AuthenticationSessionConfig,
) -> Result<AuthBackendStatus, String> {
    let authenticated_at = read_session_authenticated_at(config);

    Ok(AuthBackendStatus {
        is_authenticated: authenticated_at.is_some(),
        passkey_supported: webauthn_supported(),
        authenticated_at,
    })
}

#[cfg(target_arch = "wasm32")]
async fn auth_backend_login() -> Result<(), String> {
    web_login().await
}

#[cfg(target_arch = "wasm32")]
async fn auth_backend_logout() -> Result<(), String> {
    write_storage_value(SESSION_KEY, None)?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn auth_backend_status(
    _config: AuthenticationSessionConfig,
) -> Result<AuthBackendStatus, String> {
    Ok(AuthBackendStatus {
        is_authenticated: false,
        passkey_supported: false,
        authenticated_at: None,
    })
}

#[cfg(not(target_arch = "wasm32"))]
async fn auth_backend_login() -> Result<(), String> {
    Err("Desktop passkey login is unavailable because the desktop webview does not expose a real WebAuthn passkey prompt. Use the web app on localhost for passkey testing.".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn auth_backend_logout() -> Result<(), String> {
    Ok(())
}

#[cfg(target_arch = "wasm32")]
mod web_passkey {
    use super::{AuthenticationSessionConfig, CREDENTIAL_KEY, SESSION_KEY};
    use js_sys::{Array, Date, Function, Object, Reflect, Uint8Array};
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{
        AttestationConveyancePreference, AuthenticatorSelectionCriteria, CredentialCreationOptions,
        CredentialRequestOptions, CredentialsContainer, PublicKeyCredential,
        PublicKeyCredentialCreationOptions, PublicKeyCredentialRequestOptions,
        PublicKeyCredentialRpEntity, PublicKeyCredentialType, PublicKeyCredentialUserEntity,
        UserVerificationRequirement,
    };

    const RP_NAME: &str = "Dioxus Authentication";
    const USER_NAME: &str = "demo@dioxus.local";
    const USER_DISPLAY_NAME: &str = "Dioxus Demo User";

    /// Runs the browser WebAuthn demo login.
    ///
    /// The first login creates a discoverable local credential and stores its
    /// raw id in localStorage so later logins can call `credentials.get()`.
    /// Production passkey flows must add server-issued challenges and
    /// server-side verification before trusting authentication.
    pub async fn login() -> Result<(), String> {
        let credentials = credentials()?;
        let credential_id = match read_storage_value(CREDENTIAL_KEY)? {
            Some(credential_id) => credential_id,
            None => {
                let credential = create_credential(&credentials).await?;
                let credential_id = encode_bytes(&credential_raw_id(&credential));
                write_storage_value(CREDENTIAL_KEY, Some(&credential_id))?;
                credential_id
            }
        };

        authenticate_credential(&credentials, &credential_id).await?;
        write_storage_value(SESSION_KEY, Some(&PasskeySession::now().serialize()))?;
        Ok(())
    }

    /// Returns whether the current browser surface exposes the WebAuthn calls
    /// this demo needs.
    pub fn supported() -> bool {
        credentials().is_ok()
    }

    /// Reads the timestamped local session and expires it according to config.
    pub fn read_session_authenticated_at(config: AuthenticationSessionConfig) -> Option<String> {
        match read_storage_value(SESSION_KEY) {
            Ok(Some(value)) if value == "authenticated" => {
                let session = PasskeySession::now();
                let _ = write_storage_value(SESSION_KEY, Some(&session.serialize()));
                Some(session.authenticated_at_display)
            }
            Ok(Some(value)) => {
                let session = PasskeySession::deserialize(&value)?;
                if session.is_expired(config) {
                    let _ = write_storage_value(SESSION_KEY, None);
                    None
                } else {
                    Some(timestamp_from_epoch_millis(
                        session.authenticated_at_epoch_millis,
                    ))
                }
            }
            _ => None,
        }
    }

    pub fn read_storage_value(key: &str) -> Result<Option<String>, String> {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| "Local auth session storage is unavailable.".to_string())?;

        storage
            .get_item(key)
            .map_err(|_| "Local auth session storage could not be read.".to_string())
    }

    pub fn write_storage_value(key: &str, value: Option<&str>) -> Result<(), String> {
        let storage = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .ok_or_else(|| "Local auth session storage is unavailable.".to_string())?;

        match value {
            Some(value) => storage
                .set_item(key, value)
                .map_err(|_| "Local auth session storage could not be written.".to_string()),
            None => storage
                .remove_item(key)
                .map_err(|_| "Local auth session storage could not be cleared.".to_string()),
        }
    }

    /// Creates the local browser credential used by the demo.
    ///
    /// The browser/authenticator performs the sensitive credential operation;
    /// this app receives only the public credential wrapper and stores the raw
    /// credential id for future demo authentication requests.
    async fn create_credential(
        credentials: &CredentialsContainer,
    ) -> Result<PublicKeyCredential, String> {
        let mut challenge = random_bytes(32)?;
        let mut user_id = random_bytes(16)?;
        let rp = PublicKeyCredentialRpEntity::new(RP_NAME);
        let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
            USER_NAME,
            USER_DISPLAY_NAME,
            &mut user_id,
        );
        let options = PublicKeyCredentialCreationOptions::new_with_u8_slice(
            &mut challenge,
            &credential_parameters(),
            &rp,
            &user,
        );

        let selection = AuthenticatorSelectionCriteria::new();
        selection.set_resident_key("preferred");
        selection.set_user_verification(UserVerificationRequirement::Preferred);
        options.set_authenticator_selection(&selection);
        options.set_attestation(AttestationConveyancePreference::None);
        options.set_timeout(60_000);

        let credential_options = CredentialCreationOptions::new();
        credential_options.set_public_key(&options);

        let promise = credentials
            .create_with_options(&credential_options)
            .map_err(error_message)?;
        let value = JsFuture::from(promise).await.map_err(error_message)?;

        value
            .dyn_into::<PublicKeyCredential>()
            .map_err(|_| "Passkey credential response was not a public key credential.".to_string())
    }

    /// Asks the browser to authenticate the previously created demo credential.
    async fn authenticate_credential(
        credentials: &CredentialsContainer,
        credential_id: &str,
    ) -> Result<(), String> {
        let id = decode_bytes(credential_id)?;
        let mut challenge = random_bytes(32)?;
        let options = PublicKeyCredentialRequestOptions::new_with_u8_slice(&mut challenge);
        options.set_allow_credentials(&credential_descriptors(&id));
        options.set_user_verification(UserVerificationRequirement::Preferred);
        options.set_timeout(60_000);

        let credential_options = CredentialRequestOptions::new();
        credential_options.set_public_key(&options);

        let promise = credentials
            .get_with_options(&credential_options)
            .map_err(error_message)?;
        let value = JsFuture::from(promise).await.map_err(error_message)?;

        value
            .dyn_into::<PublicKeyCredential>()
            .map(|_| ())
            .map_err(|_| {
                "Passkey authentication response was not a public key credential.".to_string()
            })
    }

    /// Fetches and validates `navigator.credentials` before any WebAuthn call.
    fn credentials() -> Result<CredentialsContainer, String> {
        let window = web_sys::window()
            .ok_or_else(|| "Passkey API is unavailable in this renderer.".to_string())?;
        let navigator = window.navigator();
        let credentials = Reflect::get(navigator.as_ref(), &JsValue::from_str("credentials"))
            .map_err(error_message)?;
        let create =
            Reflect::get(&credentials, &JsValue::from_str("create")).unwrap_or(JsValue::UNDEFINED);
        let get =
            Reflect::get(&credentials, &JsValue::from_str("get")).unwrap_or(JsValue::UNDEFINED);

        if let Some(message) = super::passkey_api_surface_error(
            is_secure_context(),
            !credentials.is_null() && !credentials.is_undefined(),
            create.is_instance_of::<Function>(),
            get.is_instance_of::<Function>(),
        ) {
            return Err(message.to_string());
        }

        credentials
            .dyn_into::<CredentialsContainer>()
            .map_err(|_| "Passkey API is unavailable in this renderer.".to_string())
    }

    fn is_secure_context() -> bool {
        Reflect::get(&js_sys::global(), &JsValue::from_str("isSecureContext"))
            .ok()
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    }

    fn display_timestamp(date: &Date) -> String {
        let options = Object::new();
        Reflect::set(
            &options,
            &JsValue::from_str("hour12"),
            &JsValue::from_bool(false),
        )
        .expect("set timestamp hour cycle");
        Reflect::set(
            &options,
            &JsValue::from_str("year"),
            &JsValue::from_str("numeric"),
        )
        .expect("set timestamp year");
        Reflect::set(
            &options,
            &JsValue::from_str("month"),
            &JsValue::from_str("2-digit"),
        )
        .expect("set timestamp month");
        Reflect::set(
            &options,
            &JsValue::from_str("day"),
            &JsValue::from_str("2-digit"),
        )
        .expect("set timestamp day");
        Reflect::set(
            &options,
            &JsValue::from_str("hour"),
            &JsValue::from_str("2-digit"),
        )
        .expect("set timestamp hour");
        Reflect::set(
            &options,
            &JsValue::from_str("minute"),
            &JsValue::from_str("2-digit"),
        )
        .expect("set timestamp minute");
        Reflect::set(
            &options,
            &JsValue::from_str("timeZoneName"),
            &JsValue::from_str("short"),
        )
        .expect("set timestamp timezone");

        date.to_locale_string("en-US", &options)
            .as_string()
            .unwrap_or_else(|| Date::new_0().to_iso_string().into())
    }

    fn local_timestamp() -> String {
        display_timestamp(&Date::new_0())
    }

    fn timestamp_from_epoch_millis(epoch_millis: f64) -> String {
        display_timestamp(&Date::new(&JsValue::from_f64(epoch_millis)))
    }

    fn current_epoch_millis() -> f64 {
        Date::now()
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct PasskeySession {
        authenticated_at_display: String,
        authenticated_at_epoch_millis: f64,
    }

    impl PasskeySession {
        fn now() -> Self {
            Self {
                authenticated_at_display: local_timestamp(),
                authenticated_at_epoch_millis: current_epoch_millis(),
            }
        }

        fn deserialize(value: &str) -> Option<Self> {
            serde_json::from_str(value).ok()
        }

        fn serialize(&self) -> String {
            serde_json::to_string(self).expect("serialize passkey session")
        }

        fn is_expired(&self, config: AuthenticationSessionConfig) -> bool {
            let expires_after_millis = f64::from(config.expiration_hours()) * 60.0 * 60.0 * 1000.0;
            current_epoch_millis() - self.authenticated_at_epoch_millis >= expires_after_millis
        }
    }

    fn credential_parameters() -> JsValue {
        let params = Array::new();
        params.push(&credential_parameter(-7));
        params.push(&credential_parameter(-257));
        params.into()
    }

    fn credential_parameter(algorithm: i32) -> JsValue {
        let parameter = Object::new();
        Reflect::set(
            &parameter,
            &JsValue::from_str("type"),
            &JsValue::from(PublicKeyCredentialType::PublicKey),
        )
        .expect("set credential type");
        Reflect::set(
            &parameter,
            &JsValue::from_str("alg"),
            &JsValue::from_f64(algorithm as f64),
        )
        .expect("set credential algorithm");
        parameter.into()
    }

    fn credential_descriptors(credential_id: &[u8]) -> JsValue {
        let descriptor = Object::new();
        Reflect::set(
            &descriptor,
            &JsValue::from_str("type"),
            &JsValue::from(PublicKeyCredentialType::PublicKey),
        )
        .expect("set credential descriptor type");
        Reflect::set(
            &descriptor,
            &JsValue::from_str("id"),
            &Uint8Array::from(credential_id).into(),
        )
        .expect("set credential descriptor id");

        let descriptors = Array::new();
        descriptors.push(&descriptor);
        descriptors.into()
    }

    fn credential_raw_id(credential: &PublicKeyCredential) -> Vec<u8> {
        Uint8Array::new(&credential.raw_id()).to_vec()
    }

    fn random_bytes(length: usize) -> Result<Vec<u8>, String> {
        let crypto = web_sys::window()
            .ok_or_else(|| "Browser window is unavailable.".to_string())?
            .crypto()
            .map_err(error_message)?;
        let bytes = Uint8Array::new_with_length(length as u32);
        crypto
            .get_random_values_with_array_buffer_view(&bytes)
            .map_err(error_message)?;
        Ok(bytes.to_vec())
    }

    fn encode_bytes(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn decode_bytes(value: &str) -> Result<Vec<u8>, String> {
        if value.len() % 2 != 0 {
            return Err("Stored passkey credential id is malformed.".to_string());
        }

        (0..value.len())
            .step_by(2)
            .map(|index| {
                u8::from_str_radix(&value[index..index + 2], 16)
                    .map_err(|_| "Stored passkey credential id is malformed.".to_string())
            })
            .collect()
    }

    fn error_message(value: JsValue) -> String {
        if let Some(message) = value.as_string() {
            return message;
        }

        Reflect::get(&value, &JsValue::from_str("message"))
            .ok()
            .and_then(|message| message.as_string())
            .unwrap_or_else(|| "Passkey authentication failed.".to_string())
    }
}

#[cfg(target_arch = "wasm32")]
use web_passkey::{
    login as web_login, read_session_authenticated_at, supported as webauthn_supported,
    write_storage_value,
};

#[cfg(target_arch = "wasm32")]
const CREDENTIAL_KEY: &str = "dioxus-authentication.passkey.credential";
#[cfg(target_arch = "wasm32")]
const SESSION_KEY: &str = "dioxus-authentication.passkey.session";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_message_matches_auth_state() {
        assert_eq!(status_message(false, true, None), "You are not logged in.");
        assert_eq!(
            status_message(true, true, Some("05/04/2026, 15:30 GMT-3")),
            "Logged in via passkey at 05/04/2026, 15:30 GMT-3"
        );
        assert_eq!(
            status_message(true, true, None),
            "You are already logged in via passkey"
        );
        assert_eq!(
            status_message(false, false, None),
            "Desktop passkey login is unavailable. Use the web app on localhost for passkey testing."
        );
    }

    #[test]
    fn demo_origin_warning_reports_wrong_origin() {
        assert_eq!(
            demo_origin_warning(
                "http://127.0.0.1:8080",
                AuthenticationService::REQUIRED_DEMO_ORIGIN
            ),
            Some(
                "You are running app from http://127.0.0.1:8080 but you must run it from http://localhost:8080."
                    .to_string()
            )
        );
        assert_eq!(
            demo_origin_warning(
                AuthenticationService::REQUIRED_DEMO_ORIGIN,
                AuthenticationService::REQUIRED_DEMO_ORIGIN
            ),
            None
        );
    }

    #[test]
    fn passkey_api_surface_accepts_complete_secure_surface() {
        assert_eq!(passkey_api_surface_error(true, true, true, true), None);
    }

    #[test]
    fn passkey_api_surface_rejects_missing_credentials_before_create_call() {
        assert_eq!(
            passkey_api_surface_error(true, false, false, false),
            Some("Passkey API is unavailable in this renderer.")
        );
    }

    #[test]
    fn passkey_api_surface_rejects_missing_create_or_get_functions() {
        assert_eq!(
            passkey_api_surface_error(true, true, false, true),
            Some("Passkey API is unavailable in this renderer.")
        );
        assert_eq!(
            passkey_api_surface_error(true, true, true, false),
            Some("Passkey API is unavailable in this renderer.")
        );
    }

    #[test]
    fn passkey_api_surface_rejects_insecure_context_first() {
        assert_eq!(
            passkey_api_surface_error(false, false, false, false),
            Some("Passkeys require a secure browser context such as localhost or HTTPS.")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn desktop_passkey_backend_reports_unsupported() {
        futures::executor::block_on(async {
            let config = AuthenticationSessionConfig::default();
            let status = AuthenticationService::status(config)
                .await
                .expect("desktop status should be readable");
            assert!(!status.is_authenticated);
            assert!(!status.passkey_supported);

            let error = AuthenticationService::login(config)
                .await
                .expect_err("desktop passkey login should be blocked");
            assert!(
                error.contains("desktop webview does not expose a real WebAuthn passkey prompt")
            );
        });
    }
}
