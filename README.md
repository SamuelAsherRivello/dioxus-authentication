# Dioxus Authentication

Dioxus Authentication is a reusable passkey-first authentication component for Dioxus 0.7 apps. This repo also includes a runnable demo so you can try the component first, then copy the focused `packages/authentication` package into your own Dioxus workspace when it fits your project.

## TOC

- [TOC](#toc)
- [Live Demo](#live-demo)
- [Pics](#pics)
- [Getting Started](#getting-started)
  - [Common Scripts](#common-scripts)
  - [Other Scripts](#other-scripts)
- [Authentication Component](#authentication-component)
- [Demo](#demo)
- [Bring Auth Into Your Project](#bring-auth-into-your-project)
- [Details](#details)
  - [Structure](#structure)
  - [Features](#features)
- [Credits](#credits)

## Live Demo

https://samuelasherrivello.github.io/dioxus-authentication/

The static web build is exported and hosted automatically with each push to the main branch. For real passkey testing, run the app locally at `http://localhost:8080`; browsers require a secure context and this demo is coded to use localhost for WebAuthn.

## Pics

| Screenshot | Infographic |
| ---------- | ----------- |
| ![Screenshot of the Dioxus Authentication demo](./Documentation/Images/Screenshot01.png) | ![Infographic for the Dioxus Authentication component and demo architecture](./Documentation/Images/Infographic01.png) |

## Getting Started

### Common Scripts

| Command | Required? | Description |
| ------- | --------- | ----------- |
| `.\Scripts\Common\InstallDependencies.ps1` | ✅ | Installs Rust, the wasm target, Dioxus CLI, and runs a validation build. |
| `.\Scripts\Common\RunWeb.ps1` | ✅ | Starts the web demo at `http://localhost:8080` for passkey testing; pass `-Lan` for same-Wi-Fi viewing without passkey support. |

### Other Scripts

| Command | Required? | Description |
| ------- | --------- | ----------- |
| `.\Scripts\Other\RunDesktop.ps1` | ❌ | Starts the desktop demo with Dioxus desktop. Passkey login is intentionally unavailable in desktop WebView. |
| `.\Scripts\Other\RunTests.ps1` | ❌ | Runs RunWeb script tests and UI/auth crate tests. |

## Authentication Component

`packages/authentication` is the primary artifact in this repo. It owns the reusable component, auth service boundary, passkey prompt feedback, and component stylesheet so it can be carried into another Dioxus app without dragging the whole demo along.

| Public Surface | Role |
| -------------- | ---- |
| `AuthenticationComponent` | Dioxus component that renders auth status, passkey login, logout, prompt feedback, and built-in auth styling. |
| `AuthenticationService` | Async service boundary for current status, login, logout, session expiration, and platform support checks. |
| `AuthenticationSessionConfig` | Small configuration type for the local demo session lifetime. |
| `current_demo_origin_warning()` | Helper for showing the localhost requirement when testing browser passkeys. |
| `DEFAULT_PASSKEY_EXPIRATION_HOURS` | Default local demo session lifetime, currently 48 hours. |

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

## Demo

The demo proves the package inside a normal Dioxus workspace. Use it to inspect the UI, test the local passkey flow in a browser, and confirm how the component behaves on desktop before reusing the package elsewhere.

| Demo Surface | Description |
| ------------ | ----------- |
| Home route | Presents the component, a short usage embed, and the live passkey demo. |
| Web runner | Uses typed WebAuthn browser bindings through `web-sys` and stores the local demo credential/session in browser localStorage. |
| Desktop runner | Keeps desktop support intact, but reports passkey login as unavailable instead of simulating a browser passkey prompt. |
| Demo warning | Shows when the web app is not running from `http://localhost:8080`, which is the intended passkey test origin. |

## Bring Auth Into Your Project

The lightest path is to copy only the reusable package and then reference it from your app crate. The demo packages are useful as examples, but they are not required for adoption.

| Step | Action |
| ---- | ------ |
| 1 | Copy [`packages/authentication`](./packages/authentication) into your Dioxus workspace, commonly as `packages/authentication`. |
| 2 | Add `authentication = { path = "../authentication" }` to the consuming crate, adjusting the relative path for your workspace layout. |
| 3 | Keep or adapt the package dependencies from [`packages/authentication/Cargo.toml`](./packages/authentication/Cargo.toml), including Dioxus 0.7, Dioxus Components primitives, and wasm WebAuthn bindings. |
| 4 | Render `AuthenticationComponent { expiration_hours: Some(48) }` from any routed page or shell component. |
| 5 | Replace the local demo session behavior with server-issued challenges and server-side verification before trusting passkeys in production. |

Production passkey auth should add server challenge generation, server-side verification, persistent user records, and tamper-resistant sessions. This repo keeps those concerns behind `AuthenticationService` so the component can stay stable while the backend becomes real.

## Details

The sections below separate the reusable authentication package from the demo workspace that proves it.

### Structure

#### Root Folders

| # | Name | In Git? | Purpose |
| - | ---- | ------- | ------- |
| 01 | [`.agents`](./.agents) | ✅ | Codex-facing Spec Kit skills used for specify, plan, tasks, implementation, and analysis workflows. |
| 02 | [`.codex`](./.codex) | ✅ | Repo-local Codex guidance, Dioxus rules, and project-specific skills. |
| 03 | [`.github`](./.github) | ✅ | GitHub Actions automation, including the web export workflow. |
| 04 | [`.specify`](./.specify) | ✅ | Spec Kit configuration, templates, scripts, workflows, constitution, and active feature state. |
| 05 | [`Documentation`](./Documentation) | ✅ | README screenshots, infographic images, and supporting documentation assets. |
| 06 | [`packages`](./packages) | ✅ | Rust workspace crates for reusable auth, shared demo UI, web entrypoint, and desktop entrypoint. |
| 07 | [`Scripts`](./Scripts) | ✅ | PowerShell setup, run, and test workflows. |
| 08 | [`specs`](./specs) | ✅ | Project specs for the auth-focused repo. |
| 09 | [`data`](./data) | ❌ | Local native runtime database output for desktop/non-wasm runs. |
| 10 | [`node_modules`](./node_modules) | ❌ | Local dependency cache if Node-based tooling is installed. |
| 11 | [`target`](./target) | ❌ | Cargo build output. |
| 12 | [`test-results`](./test-results) | ❌ | Local browser/test artifacts. |
| 13 | [`tmp`](./tmp) | ❌ | Local scratch output. |

#### Source Folders

| Path | Description |
| ---- | ----------- |
| [`packages/authentication`](./packages/authentication) | Reusable Dioxus auth package and the main component to copy into another project. |
| [`packages/ui`](./packages/ui) | Shared demo app shell, route, localization, toast, data cache demo, and page UI. |
| [`packages/web`](./packages/web) | Web entrypoint and web assets for trying browser passkeys. |
| [`packages/desktop`](./packages/desktop) | Desktop entrypoint and desktop assets for verifying the app still runs outside the browser. |

The shared demo app preserves the starter template behavior for routing, localization, theme state, localStorage snapshots, and native SQLite cache reads. The authentication package stays separate so the component can be evaluated in the demo and then reused without the demo shell.

### Features

#### Authentication Features

| # | Feature | In Project? | Usage |
| - | ------- | ----------- | ----- |
| 1 | Reusable Dioxus auth component | ✅ | [`AuthenticationComponent`](./packages/authentication/src/component.rs) renders the auth card, actions, status, prompt, and stylesheet link. |
| 2 | Auth service boundary | ✅ | [`AuthenticationService`](./packages/authentication/src/service.rs) owns status, login, logout, expiration, and platform support checks. |
| 3 | Browser passkey demo | ✅ | Web builds use typed WebAuthn browser APIs through `web-sys`. |
| 4 | Local demo session expiration | ✅ | Browser localStorage stores a timestamped demo session and expires it through `AuthenticationSessionConfig`. |
| 5 | Desktop support | ✅ | Desktop builds compile and run, with an explicit unsupported passkey status. |
| 6 | Production server verification | ❌ | Future work should wire Dioxus fullstack server functions to `webauthn-rs` or another production passkey backend. |
| 7 | Username/password auth | ❌ | Intentionally out of scope for this passkey-first component. |
| 8 | Social login | ❌ | Intentionally out of scope for this passkey-first component. |

## Credits

**Created By**

Samuel Asher Rivello. Over 25 years XP with game development (2025); over 10 years XP with Unity (2025).

**Contact**

| Channel | Link |
| ------- | ---- |
| Twitter | [@srivello](https://twitter.com/srivello) |
| Git | [Github.com/SamuelAsherRivello](https://github.com/SamuelAsherRivello) |
| Resume & Portfolio | [SamuelAsherRivello.com](https://www.SamuelAsherRivello.com) |
| LinkedIn | [Linkedin.com/in/SamuelAsherRivello](https://www.linkedin.com/in/SamuelAsherRivello) |

**License**

Provided as-is under [MIT License](./LICENSE).

Copyright (c) 2006 - 2026 Rivello Multimedia Consulting, LLC
