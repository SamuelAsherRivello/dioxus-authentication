use crate::providers::is_passkey_provider;
use serde::{Deserialize, Serialize};

/// Default lifetime for the local passkey session.
///
/// The browser still owns the passkey credential. This value only controls how
/// long the app keeps its local authenticated session marker.
pub const DEFAULT_PASSKEY_EXPIRATION_HOURS: u32 = 48;
pub const DEFAULT_PASSKEY_APP_ID: &str = "my_app_id";
pub const DEFAULT_PASSKEY_RELYING_PARTY_NAME: &str = "Dioxus Authentication";
pub const DEFAULT_PASSKEY_USER_NAME: &str = "user@dioxus.local";
pub const DEFAULT_PASSKEY_USER_DISPLAY_NAME: &str = "Dioxus Authentication User";

/// Current authentication state rendered by `AuthenticationView`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthenticationStatus {
    /// `true` when the local session exists and has not expired.
    pub is_authenticated: bool,
    /// `true` when the current renderer supports the login action.
    pub login_supported: bool,
    /// `true` when the current renderer exposes the browser WebAuthn API.
    pub passkey_supported: bool,
    /// Browser-local display timestamp for the active session.
    pub authenticated_at: Option<String>,
    /// Stored passkey credential id that can identify the signed-in user record.
    pub passkey_database_key: Option<String>,
    /// Authentication method used by the current target.
    pub auth_method: AuthenticationMethod,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct AuthBackendStatus {
    is_authenticated: bool,
    login_supported: bool,
    passkey_supported: bool,
    authenticated_at: Option<String>,
    passkey_database_key: Option<String>,
    auth_method: AuthenticationMethod,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum AuthenticationMethod {
    WebPasskey,
    WindowsPasskey,
    UnsupportedNative,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticationSessionConfig {
    expiration_hours: u32,
    app_id: String,
}

impl AuthenticationSessionConfig {
    pub fn new(expiration_hours: u32, app_id: impl Into<String>) -> Self {
        Self {
            expiration_hours: expiration_hours.max(1),
            app_id: non_empty_or_default(app_id.into(), DEFAULT_PASSKEY_APP_ID),
        }
    }

    /// Creates a session config, clamping zero-hour values to one hour.
    pub fn hours(expiration_hours: u32) -> Self {
        Self::new(expiration_hours, DEFAULT_PASSKEY_APP_ID)
    }

    /// Uses a stable app id for reusable local demo passkey state.
    pub fn with_app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = non_empty_or_default(app_id.into(), DEFAULT_PASSKEY_APP_ID);
        self
    }

    /// Returns the configured local session lifetime in hours.
    pub fn expiration_hours(&self) -> u32 {
        self.expiration_hours
    }

    /// Returns the local demo passkey namespace.
    pub fn app_id(&self) -> &str {
        &self.app_id
    }
}

impl Default for AuthenticationSessionConfig {
    fn default() -> Self {
        Self::hours(DEFAULT_PASSKEY_EXPIRATION_HOURS)
    }
}

/// Display metadata sent to the passkey prompt during credential registration.
///
/// Browsers and operating systems own the final prompt UI. These values control
/// the WebAuthn relying-party and user labels that the prompt may show.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct AuthenticationPasskeyConfig {
    app_id: String,
    relying_party_name: String,
    user_name: String,
    user_display_name: String,
}

impl AuthenticationPasskeyConfig {
    pub fn new(
        app_id: impl Into<String>,
        relying_party_name: impl Into<String>,
        user_name: impl Into<String>,
        user_display_name: impl Into<String>,
    ) -> Self {
        Self {
            app_id: non_empty_or_default(app_id.into(), DEFAULT_PASSKEY_APP_ID),
            relying_party_name: non_empty_or_default(
                relying_party_name.into(),
                DEFAULT_PASSKEY_RELYING_PARTY_NAME,
            ),
            user_name: non_empty_or_default(user_name.into(), DEFAULT_PASSKEY_USER_NAME),
            user_display_name: non_empty_or_default(
                user_display_name.into(),
                DEFAULT_PASSKEY_USER_DISPLAY_NAME,
            ),
        }
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn relying_party_name(&self) -> &str {
        &self.relying_party_name
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn user_display_name(&self) -> &str {
        &self.user_display_name
    }
}

impl Default for AuthenticationPasskeyConfig {
    fn default() -> Self {
        Self::new(
            DEFAULT_PASSKEY_APP_ID,
            DEFAULT_PASSKEY_RELYING_PARTY_NAME,
            DEFAULT_PASSKEY_USER_NAME,
            DEFAULT_PASSKEY_USER_DISPLAY_NAME,
        )
    }
}

fn non_empty_or_default(value: String, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.to_string()
    } else {
        value.to_string()
    }
}

pub struct AuthenticationService;

impl AuthenticationService {
    /// Reads the current authentication state for the active target.
    ///
    /// Web builds inspect the browser WebAuthn-backed session. Windows desktop
    /// builds inspect a native Win32 WebAuthn-backed session.
    pub async fn status(
        config: AuthenticationSessionConfig,
    ) -> Result<AuthenticationStatus, String> {
        let status = auth_backend_status(config).await?;

        Ok(AuthenticationStatus {
            is_authenticated: status.is_authenticated,
            login_supported: status.login_supported,
            passkey_supported: status.passkey_supported,
            authenticated_at: status.authenticated_at.clone(),
            passkey_database_key: status.passkey_database_key.clone(),
            auth_method: status.auth_method,
        })
    }

    /// Starts the login flow and returns the refreshed authentication state.
    ///
    /// In web builds this creates a local credential when needed, asks the
    /// browser to authenticate it, then writes a timestamped local session. In
    /// Windows desktop builds this uses the Win32 WebAuthn API so Windows owns
    /// the real passkey prompt.
    pub async fn login(
        config: AuthenticationSessionConfig,
        provider: impl AsRef<str>,
        passkey_config: AuthenticationPasskeyConfig,
    ) -> Result<AuthenticationStatus, String> {
        let provider = provider.as_ref().trim();

        if provider.is_empty() {
            return Err("Authentication provider is required.".to_string());
        }

        if is_passkey_provider(provider) {
            auth_backend_login(passkey_config).await?;
        } else {
            return Err(format!("Unsupported authentication provider: {provider}"));
        }
        Self::status(config).await
    }

    /// Clears the local session and returns the refreshed state.
    pub async fn logout(
        config: AuthenticationSessionConfig,
    ) -> Result<AuthenticationStatus, String> {
        auth_backend_logout(config.clone()).await?;
        Self::status(config).await
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
    let app_id = config.app_id().to_string();
    let authenticated_at = read_session_authenticated_at(config);
    let passkey_database_key = authenticated_at
        .as_ref()
        .and_then(|_| read_passkey_database_key(&app_id));

    Ok(AuthBackendStatus {
        is_authenticated: authenticated_at.is_some(),
        login_supported: webauthn_supported(),
        passkey_supported: webauthn_supported(),
        authenticated_at,
        passkey_database_key,
        auth_method: AuthenticationMethod::WebPasskey,
    })
}

#[cfg(target_arch = "wasm32")]
async fn auth_backend_login(passkey_config: AuthenticationPasskeyConfig) -> Result<(), String> {
    web_login(passkey_config).await
}

#[cfg(target_arch = "wasm32")]
async fn auth_backend_logout(config: AuthenticationSessionConfig) -> Result<(), String> {
    web_passkey::logout(config).await
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
async fn auth_backend_status(
    config: AuthenticationSessionConfig,
) -> Result<AuthBackendStatus, String> {
    let app_id = config.app_id().to_string();
    let authenticated_at = windows_passkey::read_session_authenticated_at(config);
    let passkey_database_key = authenticated_at
        .as_ref()
        .and_then(|_| windows_passkey::read_passkey_database_key(&app_id));
    let passkey_supported = windows_passkey::supported();

    Ok(AuthBackendStatus {
        is_authenticated: authenticated_at.is_some(),
        login_supported: passkey_supported,
        passkey_supported,
        authenticated_at,
        passkey_database_key,
        auth_method: AuthenticationMethod::WindowsPasskey,
    })
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
async fn auth_backend_login(passkey_config: AuthenticationPasskeyConfig) -> Result<(), String> {
    windows_passkey::login(passkey_config)
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
async fn auth_backend_logout(config: AuthenticationSessionConfig) -> Result<(), String> {
    windows_passkey::logout(config)
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
async fn auth_backend_status(
    _config: AuthenticationSessionConfig,
) -> Result<AuthBackendStatus, String> {
    Ok(AuthBackendStatus {
        is_authenticated: false,
        login_supported: false,
        passkey_supported: false,
        authenticated_at: None,
        passkey_database_key: None,
        auth_method: AuthenticationMethod::UnsupportedNative,
    })
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
async fn auth_backend_login(_passkey_config: AuthenticationPasskeyConfig) -> Result<(), String> {
    Err("Real passkey sign-in is unavailable on this desktop target.".to_string())
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
async fn auth_backend_logout(_config: AuthenticationSessionConfig) -> Result<(), String> {
    Ok(())
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
mod windows_passkey {
    use super::AuthenticationPasskeyConfig;
    use super::AuthenticationSessionConfig;
    use chrono::{Local, TimeZone};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use windows::core::{w, BOOL, PCWSTR};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Networking::WindowsWebServices::{
        WebAuthNAuthenticatorGetAssertion, WebAuthNAuthenticatorMakeCredential,
        WebAuthNFreeAssertion, WebAuthNFreeCredentialAttestation,
        WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable,
        WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE,
        WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS,
        WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION,
        WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS,
        WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION, WEBAUTHN_CLIENT_DATA,
        WEBAUTHN_CLIENT_DATA_CURRENT_VERSION, WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256,
        WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256, WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
        WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION,
        WEBAUTHN_CREDENTIAL, WEBAUTHN_CREDENTIALS, WEBAUTHN_CREDENTIAL_CURRENT_VERSION,
        WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY, WEBAUTHN_HASH_ALGORITHM_SHA_256,
        WEBAUTHN_RP_ENTITY_INFORMATION, WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION,
        WEBAUTHN_USER_ENTITY_INFORMATION, WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION,
        WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED,
    };

    const RP_ID: PCWSTR = w!("localhost");
    const SESSION_FILE: &str = "data/authentication/native-passkey-session.json";

    pub fn read_session_authenticated_at(config: AuthenticationSessionConfig) -> Option<String> {
        let mut state = read_state().ok()?;
        if state
            .passkey_config
            .as_ref()
            .is_none_or(|passkey_config| passkey_config.app_id() != config.app_id())
        {
            return None;
        }

        let authenticated_at_epoch_seconds = state.authenticated_at_epoch_seconds?;

        if session_is_expired(authenticated_at_epoch_seconds, config) {
            state.authenticated_at_epoch_seconds = None;
            let _ = write_state(&state);
            None
        } else {
            Some(format_timestamp(authenticated_at_epoch_seconds))
        }
    }

    pub fn read_passkey_database_key(app_id: &str) -> Option<String> {
        let state = read_state().ok()?;
        if state
            .passkey_config
            .as_ref()
            .is_some_and(|passkey_config| passkey_config.app_id() == app_id)
        {
            state.credential_id_hex
        } else {
            None
        }
    }

    pub fn supported() -> bool {
        unsafe {
            WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable()
                .map(|available| available.as_bool())
                .unwrap_or(false)
        }
    }

    pub fn login(passkey_config: AuthenticationPasskeyConfig) -> Result<(), String> {
        if !supported() {
            return Err("Windows passkey verification is unavailable on this device.".to_string());
        }

        let mut state = read_state().unwrap_or_default();
        if state.credential_id_hex.is_none()
            || state.passkey_config.as_ref() != Some(&passkey_config)
        {
            state.credential_id_hex = Some(make_credential(&passkey_config)?);
            state.passkey_config = Some(passkey_config);
        } else if let Some(credential_id_hex) = state.credential_id_hex.as_deref() {
            get_assertion(credential_id_hex)?;
        }

        state.authenticated_at_epoch_seconds = Some(current_epoch_seconds());
        write_state(&state)
    }

    pub fn logout(config: AuthenticationSessionConfig) -> Result<(), String> {
        let mut state = read_state().unwrap_or_default();
        if state
            .passkey_config
            .as_ref()
            .is_some_and(|passkey_config| passkey_config.app_id() != config.app_id())
        {
            return Ok(());
        }

        state.authenticated_at_epoch_seconds = None;
        write_state(&state)
    }

    fn make_credential(passkey_config: &AuthenticationPasskeyConfig) -> Result<String, String> {
        let challenge = random_bytes(32)?;
        let mut user_id = random_bytes(16)?;
        let mut client_data_json = client_data_json("webauthn.create", &challenge);
        let client_data = client_data(&mut client_data_json);
        let rp_name = wide_null(passkey_config.relying_party_name());
        let user_name = wide_null(passkey_config.user_name());
        let user_display_name = wide_null(passkey_config.user_display_name());
        let rp = WEBAUTHN_RP_ENTITY_INFORMATION {
            dwVersion: WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION,
            pwszId: RP_ID,
            pwszName: PCWSTR::from_raw(rp_name.as_ptr()),
            pwszIcon: PCWSTR::null(),
        };
        let user = WEBAUTHN_USER_ENTITY_INFORMATION {
            dwVersion: WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION,
            cbId: user_id.len() as u32,
            pbId: user_id.as_mut_ptr(),
            pwszName: PCWSTR::from_raw(user_name.as_ptr()),
            pwszIcon: PCWSTR::null(),
            pwszDisplayName: PCWSTR::from_raw(user_display_name.as_ptr()),
        };
        let mut credential_parameters = [
            credential_parameter(WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256),
            credential_parameter(WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256),
        ];
        let parameters = WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
            cCredentialParameters: credential_parameters.len() as u32,
            pCredentialParameters: credential_parameters.as_mut_ptr(),
        };
        let options = WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
            dwVersion: WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION,
            dwTimeoutMilliseconds: 60_000,
            bRequireResidentKey: BOOL(0),
            dwUserVerificationRequirement: WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED,
            dwAttestationConveyancePreference: WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE,
            ..Default::default()
        };

        let attestation = unsafe {
            WebAuthNAuthenticatorMakeCredential(
                HWND(std::ptr::null_mut()),
                &rp,
                &user,
                &parameters,
                &client_data,
                Some(&options),
            )
            .map_err(|error| format!("Windows passkey registration failed: {error}"))?
        };

        if attestation.is_null() {
            return Err("Windows passkey registration did not return a credential.".to_string());
        }

        let credential_id = unsafe {
            let attestation_ref = &*attestation;
            let bytes = std::slice::from_raw_parts(
                attestation_ref.pbCredentialId,
                attestation_ref.cbCredentialId as usize,
            )
            .to_vec();
            WebAuthNFreeCredentialAttestation(Some(attestation));
            bytes
        };

        Ok(encode_bytes(&credential_id))
    }

    fn get_assertion(credential_id_hex: &str) -> Result<(), String> {
        let mut credential_id = decode_bytes(credential_id_hex)?;
        let challenge = random_bytes(32)?;
        let mut client_data_json = client_data_json("webauthn.get", &challenge);
        let client_data = client_data(&mut client_data_json);
        let mut credential = WEBAUTHN_CREDENTIAL {
            dwVersion: WEBAUTHN_CREDENTIAL_CURRENT_VERSION,
            cbId: credential_id.len() as u32,
            pbId: credential_id.as_mut_ptr(),
            pwszCredentialType: WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY,
        };
        let credentials = WEBAUTHN_CREDENTIALS {
            cCredentials: 1,
            pCredentials: &mut credential,
        };
        let options = WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
            dwVersion: WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION,
            dwTimeoutMilliseconds: 60_000,
            CredentialList: credentials,
            dwUserVerificationRequirement: WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED,
            ..Default::default()
        };

        let assertion = unsafe {
            WebAuthNAuthenticatorGetAssertion(
                HWND(std::ptr::null_mut()),
                RP_ID,
                &client_data,
                Some(&options),
            )
            .map_err(|error| format!("Windows passkey verification failed: {error}"))?
        };

        if assertion.is_null() {
            return Err("Windows passkey verification did not return an assertion.".to_string());
        }

        unsafe {
            WebAuthNFreeAssertion(assertion);
        }
        Ok(())
    }

    fn credential_parameter(algorithm: i32) -> WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
        WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
            dwVersion: WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION,
            pwszCredentialType: WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY,
            lAlg: algorithm,
        }
    }

    fn client_data(client_data_json: &mut [u8]) -> WEBAUTHN_CLIENT_DATA {
        WEBAUTHN_CLIENT_DATA {
            dwVersion: WEBAUTHN_CLIENT_DATA_CURRENT_VERSION,
            cbClientDataJSON: client_data_json.len() as u32,
            pbClientDataJSON: client_data_json.as_mut_ptr(),
            pwszHashAlgId: WEBAUTHN_HASH_ALGORITHM_SHA_256,
        }
    }

    fn wide_null(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn client_data_json(operation: &str, challenge: &[u8]) -> Vec<u8> {
        format!(
            "{{\"type\":\"{operation}\",\"challenge\":\"{}\",\"origin\":\"https://localhost\",\"crossOrigin\":false}}",
            encode_bytes(challenge)
        )
        .into_bytes()
    }

    fn read_state() -> Result<NativePasskeyState, String> {
        let path = state_path();
        if !path.exists() {
            return Ok(NativePasskeyState::default());
        }

        let value = fs::read_to_string(&path)
            .map_err(|error| format!("Could not read Windows passkey session state: {error}"))?;
        serde_json::from_str(&value)
            .map_err(|error| format!("Windows passkey session state is malformed: {error}"))
    }

    fn write_state(state: &NativePasskeyState) -> Result<(), String> {
        let path = state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Could not create auth data directory: {error}"))?;
        }

        let value = serde_json::to_string_pretty(state).map_err(|error| {
            format!("Could not serialize Windows passkey session state: {error}")
        })?;
        fs::write(path, value)
            .map_err(|error| format!("Could not write Windows passkey session state: {error}"))
    }

    fn state_path() -> PathBuf {
        PathBuf::from(SESSION_FILE)
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    struct NativePasskeyState {
        credential_id_hex: Option<String>,
        passkey_config: Option<AuthenticationPasskeyConfig>,
        authenticated_at_epoch_seconds: Option<u64>,
    }

    fn current_epoch_seconds() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }

    fn session_is_expired(
        authenticated_at_epoch_seconds: u64,
        config: AuthenticationSessionConfig,
    ) -> bool {
        let expires_after_seconds = u64::from(config.expiration_hours()) * 60 * 60;
        current_epoch_seconds().saturating_sub(authenticated_at_epoch_seconds)
            >= expires_after_seconds
    }

    fn format_timestamp(epoch_seconds: u64) -> String {
        Local
            .timestamp_opt(epoch_seconds as i64, 0)
            .single()
            .map(|timestamp| timestamp.format("%m/%d/%Y, %H:%M %Z").to_string())
            .unwrap_or_else(|| format!("{epoch_seconds} seconds since Unix epoch"))
    }

    fn random_bytes(length: usize) -> Result<Vec<u8>, String> {
        let mut bytes = vec![0_u8; length];
        getrandom::fill(&mut bytes)
            .map_err(|error| format!("Could not generate passkey challenge bytes: {error}"))?;
        Ok(bytes)
    }

    fn encode_bytes(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn decode_bytes(value: &str) -> Result<Vec<u8>, String> {
        if value.len() % 2 != 0 {
            return Err("Stored Windows passkey credential id is malformed.".to_string());
        }

        (0..value.len())
            .step_by(2)
            .map(|index| {
                u8::from_str_radix(&value[index..index + 2], 16)
                    .map_err(|_| "Stored Windows passkey credential id is malformed.".to_string())
            })
            .collect()
    }
}

#[cfg(target_arch = "wasm32")]
mod web_passkey {
    use super::{
        AuthenticationPasskeyConfig, AuthenticationSessionConfig, CREDENTIAL_KEY,
        PASSKEY_CONFIG_KEY, SESSION_KEY,
    };
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

    /// Runs the browser WebAuthn login.
    ///
    /// The first login creates a discoverable local credential and stores its
    /// raw id in localStorage so later logins can call `credentials.get()`.
    /// Production passkey flows must add server-issued challenges and
    /// server-side verification before trusting authentication.
    pub async fn login(passkey_config: AuthenticationPasskeyConfig) -> Result<(), String> {
        let credentials = credentials()?;
        let credential_key = storage_key(CREDENTIAL_KEY, passkey_config.app_id());
        let passkey_config_key = storage_key(PASSKEY_CONFIG_KEY, passkey_config.app_id());
        let session_key = storage_key(SESSION_KEY, passkey_config.app_id());
        let credential_id = match read_storage_value(&credential_key)? {
            Some(credential_id) if stored_passkey_config_matches(&passkey_config)? => credential_id,
            _ => {
                let credential = create_credential(&credentials, &passkey_config).await?;
                let credential_id = encode_bytes(&credential_raw_id(&credential));
                write_storage_value(&credential_key, Some(&credential_id))?;
                let serialized_config =
                    serde_json::to_string(&passkey_config).map_err(|error| {
                        format!("Could not serialize passkey display config: {error}")
                    })?;
                write_storage_value(&passkey_config_key, Some(&serialized_config))?;
                credential_id
            }
        };

        authenticate_credential(&credentials, &credential_id).await?;
        write_storage_value(&session_key, Some(&PasskeySession::now().serialize()))?;
        Ok(())
    }

    /// Returns whether the current browser surface exposes the WebAuthn calls
    /// this authentication flow needs.
    pub fn supported() -> bool {
        credentials().is_ok()
    }

    pub async fn logout(config: AuthenticationSessionConfig) -> Result<(), String> {
        let session_key = storage_key(SESSION_KEY, config.app_id());
        write_storage_value(&session_key, None)
    }

    /// Reads the timestamped local session and expires it according to config.
    pub fn read_session_authenticated_at(config: AuthenticationSessionConfig) -> Option<String> {
        let session_key = storage_key(SESSION_KEY, config.app_id());
        match read_storage_value(&session_key) {
            Ok(Some(value)) if value == "authenticated" => {
                let session = PasskeySession::now();
                let _ = write_storage_value(&session_key, Some(&session.serialize()));
                Some(session.authenticated_at_display)
            }
            Ok(Some(value)) => {
                let session = PasskeySession::deserialize(&value)?;
                if session.is_expired(config) {
                    let _ = write_storage_value(&session_key, None);
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

    pub fn read_passkey_database_key(app_id: &str) -> Option<String> {
        let credential_key = storage_key(CREDENTIAL_KEY, app_id);
        read_storage_value(&credential_key).ok().flatten()
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

    fn stored_passkey_config_matches(
        passkey_config: &AuthenticationPasskeyConfig,
    ) -> Result<bool, String> {
        let passkey_config_key = storage_key(PASSKEY_CONFIG_KEY, passkey_config.app_id());
        let Some(value) = read_storage_value(&passkey_config_key)? else {
            return Ok(false);
        };

        Ok(serde_json::from_str::<AuthenticationPasskeyConfig>(&value)
            .map(|stored_config| stored_config == *passkey_config)
            .unwrap_or(false))
    }

    fn storage_key(base_key: &str, app_id: &str) -> String {
        format!("{base_key}.{app_id}")
    }

    /// Creates the local browser credential.
    ///
    /// The browser/authenticator performs the sensitive credential operation;
    /// this app receives only the public credential wrapper and stores the raw
    /// credential id for future authentication requests.
    async fn create_credential(
        credentials: &CredentialsContainer,
        passkey_config: &AuthenticationPasskeyConfig,
    ) -> Result<PublicKeyCredential, String> {
        let mut challenge = random_bytes(32)?;
        let mut user_id = random_bytes(16)?;
        let rp = PublicKeyCredentialRpEntity::new(passkey_config.relying_party_name());
        let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
            passkey_config.user_name(),
            passkey_config.user_display_name(),
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

    /// Asks the browser to authenticate the previously created credential.
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
    login as web_login, read_passkey_database_key, read_session_authenticated_at,
    supported as webauthn_supported,
};

#[cfg(target_arch = "wasm32")]
const CREDENTIAL_KEY: &str = "dioxus-authentication.passkey.credential";
#[cfg(target_arch = "wasm32")]
const PASSKEY_CONFIG_KEY: &str = "dioxus-authentication.passkey.config";
#[cfg(target_arch = "wasm32")]
const SESSION_KEY: &str = "dioxus-authentication.passkey.session";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_status_exposes_state_without_display_text() {
        let status = backend_status(
            true,
            true,
            true,
            Some("05/04/2026, 15:30 GMT-3"),
            AuthenticationMethod::WebPasskey,
        );

        assert!(status.is_authenticated);
        assert!(status.login_supported);
        assert!(status.passkey_supported);
        assert_eq!(
            status.authenticated_at.as_deref(),
            Some("05/04/2026, 15:30 GMT-3")
        );
        assert_eq!(status.auth_method, AuthenticationMethod::WebPasskey);
    }

    fn backend_status(
        is_authenticated: bool,
        login_supported: bool,
        passkey_supported: bool,
        authenticated_at: Option<&str>,
        auth_method: AuthenticationMethod,
    ) -> AuthBackendStatus {
        AuthBackendStatus {
            is_authenticated,
            login_supported,
            passkey_supported,
            authenticated_at: authenticated_at.map(str::to_string),
            passkey_database_key: None,
            auth_method,
        }
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

    #[test]
    fn session_config_new_clamps_hours_and_defaults_empty_app_id() {
        let config = AuthenticationSessionConfig::new(0, " ");

        assert_eq!(config.expiration_hours(), 1);
        assert_eq!(config.app_id(), DEFAULT_PASSKEY_APP_ID);
    }

    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
    #[test]
    fn unsupported_native_backend_refuses_login() {
        futures::executor::block_on(async {
            let config = AuthenticationSessionConfig::default();
            let status = AuthenticationService::status(config)
                .await
                .expect("unsupported native status should be readable");
            assert!(!status.is_authenticated);
            assert!(!status.login_supported);
            assert!(!status.passkey_supported);

            let error = AuthenticationService::login(
                config,
                crate::providers::PASSKEY_PROVIDER_ID,
                AuthenticationPasskeyConfig::default(),
            )
            .await
            .expect_err("unsupported native login should be unavailable");
            assert_eq!(
                error,
                "Real passkey sign-in is unavailable on this desktop target."
            );
        });
    }

    #[test]
    fn passkey_config_replaces_empty_labels_with_defaults() {
        let config = AuthenticationPasskeyConfig::new("", " ", "", "Demo User");

        assert_eq!(config.app_id(), DEFAULT_PASSKEY_APP_ID);
        assert_eq!(
            config.relying_party_name(),
            DEFAULT_PASSKEY_RELYING_PARTY_NAME
        );
        assert_eq!(config.user_name(), DEFAULT_PASSKEY_USER_NAME);
        assert_eq!(config.user_display_name(), "Demo User");
    }
}
