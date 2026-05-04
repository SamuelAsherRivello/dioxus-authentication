# Authentication Package

Reusable Dioxus passkey authentication package for the demo web and desktop apps.

## Public Surface

| Item | Description |
| ---- | ----------- |
| `AuthenticationComponent` | Dioxus component with status, passkey login, logout, prompt feedback, and built-in auth stylesheet. |
| `AuthenticationService` | Service boundary for status, login, logout, session expiration, and WebAuthn support checks. |
| `current_demo_origin_warning()` | Helper for showing the localhost requirement when testing browser passkeys. |
| Session time format | Short local browser timestamp using 24-hour time with timezone shown onscreen. |

## Usage

```rust
use authentication::AuthenticationComponent;
use dioxus::prelude::*;

#[component]
fn Home() -> Element {
    rsx! {
        AuthenticationComponent { expiration_hours: Some(48) }
    }
}
```
