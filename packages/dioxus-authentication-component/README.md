# Authentication Package

Reusable Dioxus passkey authentication package for web and desktop apps.

## Public Surface

| Item | Description |
| ---- | ----------- |
| `AuthenticationView` | Dioxus component that renders status, login/logout actions, and built-in stylesheet. |
| `AuthenticationViewConfig` | Component config for session detail, provider choices, status signal, busy signal, and login/logout callbacks. |
| `AuthenticationConfirmationPrompt` | Optional localized logout confirmation dialog. |
| `prelude` | Short import module for common component, service, config, provider, locale, and constant exports. |
| `authentication_locales()` | Built-in Fluent resources for registering the auth package with a Dioxus i18n provider. |
| `AuthenticationService` | Service boundary for current session, status projection, login, logout, session expiration, browser WebAuthn checks, and native Windows WebAuthn checks. |
| `AuthenticationSession` | Canonical service-layer session shape for local demo sessions now and Dioxus fullstack request sessions later. |
| `AuthenticationStatus` | Rendered auth state projected from `AuthenticationSession`, including the active passkey credential id to use as a database key after login. |
| `AuthenticationProvider` | Provider option model with an id and display text for the one-choice provider bar. |
| `AuthenticationPasskeyConfig` | Passkey registration metadata for app id, relying-party name, user name, and user display name. |
| Session time format | Short local timestamp using 24-hour time with timezone shown onscreen. |

## Usage

```rust
use authentication::prelude::*;
use dioxus::prelude::*;

#[component]
fn Home() -> Element {


    // Configuration
    let app_id = "my_app_id"; // Unique key for your app
    let username = "my_email@my_email.com";
    let session_config = AuthenticationSessionConfig::new(
        48, // Hours till expiration
        app_id,
    );
    let passkey_config = AuthenticationPasskeyConfig::new(
        app_id,
        "My Dioxus Authentication", // App label
        "my_email@my_email.com", // User identifier, any format
        "My Demo User", // User label
    );


    // Callbacks
    let on_login = EventHandler::new(move |_provider_id: String| {
        println!("You are logged in as user {username}");
    });
    let on_logout = EventHandler::new(move |_| {
        println!("You are logged out");
    });


    // Component
    let config = AuthenticationViewConfig::new(
        session_config, // Session settings
        vec![passkey_provider("PassKey")], // Pick one provider
        auth_status,
        auth_is_busy,
        on_login, // Sign in
        on_logout, // Sign out
    );

    rsx! {
        AuthenticationView { config }
    }
}
```

```rust
use authentication::AuthenticationConfirmationPrompt;
use dioxus::prelude::*;

rsx! {
    AuthenticationConfirmationPrompt {
        open: is_logout_confirmation_open,
        on_answer: move |confirmed| handle_logout_confirmation(confirmed),
    }
}
```

The auth package stores its Fluent bundles under `assets/i18n/`. Register those resources in your app's single global `dioxus-i18n` provider, then render `AuthenticationView` from any page. The confirmation prompt is exported separately so apps can use their own logout flow without carrying this dialog.

## Passkey Config Flow

Passkey RP and user metadata are now supplied in the flow before calling
`AuthenticationService::login`, so the view receives only rendered state and callbacks.

| Config Value | Default | Effect |
| ------------ | ------- | ------ |
| `app_id` | `my_app_id` | Keep it stable to reuse the demo key; change it to force new local test credentials. |
| `passkey_database_key` | Generated credential id | Exposed on `AuthenticationStatus` after login so an app can key related demo data. |
| `providers` | `PassKey` | Renders the provider radio bar and passes the selected provider id into login. |

## Session Flow

`AuthenticationService::current_session` returns `AuthenticationSession`, then
`AuthenticationStatus::from_session` keeps the existing view-facing status API
stable. The current web and desktop demo backends still use local passkey
session state, while a production Dioxus fullstack app can replace that backend
with request-extracted server sessions without changing `AuthenticationView`.
