# Dioxus 0.7 Workflow Rule

Use this rule for any Dioxus implementation, debugging, routing, state, asset, async loading, cache, or cross-platform work in this repository.

Primary docs: https://dioxuslabs.com/learn/0.7/

## First Pass

- Read `AGENTS.md`, this file, and the files directly involved in the requested behavior before editing.
- Respect the repo's two layers: Template Project files maintain the reusable template, while `.specs/generated/` files seed the copied Generated Project.
- Keep reusable authentication UI and business logic in `packages/authentication`; keep target entrypoints in `packages/web` and `packages/desktop`.
- Preserve both web and desktop unless the request is explicitly platform-specific.
- Prefer existing app patterns before adding new abstractions. This project keeps the reusable authentication component separate from the web and desktop demo entrypoints.
- Treat Dioxus 0.7 docs as authoritative. Do not use older Dioxus APIs such as `cx`, `Scope`, `use_state`, old router setup, or borrowed component props.

## Components And Props

- Components return `Element` and use `#[component]` when they are rendered from RSX.
- Component names must start with a capital letter or contain an underscore.
- Props must be owned, `Clone`, and `PartialEq`. Use `String`, `Vec<T>`, `Option<T>`, `ReadOnlySignal<T>`, or local model types instead of `&str` or borrowed slices.
- Use component syntax inside RSX, for example `TemplatePage { title, body_01, body_02, body_03, data_text }`; do not call component functions directly.
- For repeated UI, prefer direct `for` loops in RSX when the output is straightforward and easier to read.
- Keep target-specific platform code behind modules or `cfg` gates. Shared components should compile for both `wasm32-unknown-unknown` and desktop.

## State And Reactivity

- Use `use_signal` for local mutable state and context signals for shared app state.
- Read signal values with call syntax for cheap clones, `.read()` for borrowed reads, `.peek()` for non-subscribing reads, and write with `.set()`, `.write()`, or `.with_mut()`.
- Use `.peek()` inside background persistence or prompt-deduplication logic when a reactive subscription would create a feedback loop.
- Use `use_memo` for derived values that should recalculate only when their dependencies change.
- Use `use_context_provider` in layout/root components and `use_context::<Signal<T>>()` in descendants when shared state is needed.

## Async Loading

- Use `use_resource` for async data that should rerun when the signals it reads change.
- `Resource` reads return `None` while loading and `Some(value)` after completion. Preserve a visible loading or toast state for `None`.
- Use async event handlers or `spawn` for user-triggered work and timers.
- Avoid overlapping user-triggered loads unless the existing flow supports them. Gate refresh behavior with existing request signals or explicit loading state.
- Never block the first meaningful render on optional cache work when snapshot data can render first.

## Layout

- The web and desktop crates render the authentication demo directly from their entrypoints.
- If routes are reintroduced, keep router-aware navigation under `Router::<Route> {}`.

## Assets And Styles

- Use `asset!("/assets/...")` for local files relative to the package root. Do not use absolute machine paths.
- Keep reusable component CSS under `packages/authentication/assets`; keep target shell CSS under `packages/web/assets` and `packages/desktop/assets`.
- Inject styles with Dioxus document components already used in the repo.
- For browser-visible styling or asset changes, run the real web app and inspect it instead of trusting compile success alone.

## Authentication State Behavior

- Preserve visible loading or prompt-style feedback for authentication status checks, login, logout, and errors.
- Browser builds use localStorage for demo passkey credential/session state.
- Browser builds must not introduce SQLite or OPFS worker startup.
- Desktop builds must not use fake authentication. Windows desktop uses native Win32 WebAuthn support; unsupported native targets must report passkey login as unavailable instead of pretending to authenticate.

## Documentation

- For Template Project workflow changes, update root `README.md`, root `AGENTS.md`, `.specs/template/`, `.codex/`, and the creation skill as needed.
- For Generated Project starter changes, update `.specs/generated/` and keep the creation skill aligned.
- Keep the root `README.md` Dioxus Features section updated whenever current Dioxus feature usage or planned extension points change.

| Markdown pattern | Preferred format |
| ---------------- | ---------------- |
| Multiple related items | Use a table instead of bullets or numbered lists unless the content is procedural code guidance or a short nested explanation. |
| Boolean table cells | Use ✅ for yes and ❌ for no instead of spelling out `Yes` or `No`. |

## Verification

Use the smallest check that proves the change, then broaden when the edit crosses packages or runtime surfaces:

```powershell
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
.\Scripts\Other\RunTests.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Other\RunDesktop.ps1
```

Use a concrete local IPv4 address instead of `0.0.0.0` for web testing on Windows. If port `8080` is already serving an older build, stop that server and restart this project before trusting browser results.
