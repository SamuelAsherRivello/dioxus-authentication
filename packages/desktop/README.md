# Desktop App

This crate contains the desktop app entrypoint and desktop-only assets. It renders the reusable `authentication` package through the shared UI, with Windows desktop backed by native Win32 WebAuthn passkey prompts.

Run it from the workspace root:

```sh
dx serve --package desktop --desktop
```
