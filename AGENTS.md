You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.7 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone.

Provide concise code examples with detailed descriptions.

# Project Workflow

* Inspect the current files before editing; keep changes scoped to the requested behavior.
* Preserve the workspace split between `packages/ui`, `packages/web`, and `packages/desktop`.
* Prefer repository scripts for repeat workflows:
	* `.\Scripts\Common\InstallDependencies.ps1`
	* `.\Scripts\Common\RunWeb.ps1`
	* `.\Scripts\Other\RunDesktop.ps1`
	* `.\Scripts\Other\RunTests.ps1`
* Keep both web and desktop support intact unless the request is explicitly platform-specific.
* For browser-visible changes, serve the real web app and verify the page when practical.
* Preserve visible loading or toast-style status feedback during data loading, cache reads, cache writes, and database creation.
* Browser builds use localStorage snapshots, and non-wasm builds use native SQLite unless the project intentionally changes the cache architecture.
* Keep first-time native database/schema/seed setup in the clearly named `create_database_if_missing()` service method. Normal reads should not recreate or reseed an existing database.
* Do not introduce browser SQLite or OPFS worker startup unless the project explicitly adopts that architecture.
* Keep the root `README.md` Dioxus Features section updated whenever Dioxus feature usage, Dioxus Components usage, routes, cache behavior, platform support, or suggested future work changes.

# Markdown Format Rules

| Rule | Requirement |
| ---- | ----------- |
| Listed information | When Markdown content lists multiple related items, prefer a table over bullets or numbered lists unless the content is procedural code guidance or a short nested explanation. |
| Boolean table cells | When a table cell would otherwise say `Yes` or `No`, use ✅ for yes and ❌ for no. |

# AI Agent Safety Rules

* Do not read, create, edit, move, rename, delete, or otherwise operate on files outside this project repository unless the user explicitly names another repository as part of the current request.
* Keep all generated files, scratch files, downloaded assets, caches, and temporary outputs inside this project repository.
* Do not run destructive git operations. This includes `git reset --hard`, `git clean`, forced checkout that discards work, deleting branches or tags, force-pushing, rewriting history, or deleting commits.
* Normal git commits are allowed when the user asks for them. Do not rebase, squash, amend existing commits, or rewrite commit history; if the user wants rebase or squash work, the user must do it himself.
* Do not delete a repository or remove an existing commit from any repository. If a request requires history rewriting or destructive cleanup, stop and explain that the user must perform that operation manually.
