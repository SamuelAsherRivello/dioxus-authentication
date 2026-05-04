# Dioxus Authentication

Use this folder for Codex-specific project context that should travel with this repository. Spec Kit workflow skills live in `.agents/skills`; keep `.codex` focused on project-specific rules for the Dioxus Authentication.

## Project

`dioxus-authentication` is a Rust workspace for a Dioxus 0.7 template app with shared UI code and separate web and desktop entrypoints.

Use `.codex/project-identity.md` as the first checklist when changing this template into a new named project.

This repo has two layers:

| Layer | Purpose |
| ----- | ------- |
| Template Project | Maintains and documents the reusable template. Root `README.md`, root `AGENTS.md`, `.specs/template/`, and template-focused skills belong to this layer. |
| Generated Project | Supplies files for the new repo after copying. `.specs/generated/` contains root replacements and starter specs. |

| Path | Purpose |
| ---- | ------- |
| `packages/ui` | Shared Dioxus code split between pages, components, models, client services, assets, and tests. |
| `packages/web` | Web app entrypoint and web assets. |
| `packages/desktop` | Desktop app entrypoint and desktop assets. |
| `Scripts` | Windows PowerShell workflows for setup, web run, desktop run, and tests. |
| `Documentation/Images` | README screenshots and infographic assets that can be replaced in-place. |
| `.specs/template` | Template Project guidance and specs. |
| `.specs/generated` | Generated Project replacement docs and starter specs. |

## Project Rules

Use `.codex/rules/dioxus-0.7-workflow.md` for Dioxus implementation, debugging, routing, state, asset, async loading, cache, or cross-platform work.

Use `.codex/rules/frontend-design.md` when changing user-visible UI, CSS, layout, component composition, route presentation, loading states, toasts, empty/error states, or visual assets.

Keep the root `README.md` Dioxus Features section updated when the template's Dioxus feature usage or suggested next work changes.

## Default Commands

Prefer the repository scripts when possible:

```powershell
.\Scripts\Common\InstallDependencies.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Other\RunDesktop.ps1
.\Scripts\Other\RunTests.ps1
```

For direct Dioxus web work, use:

```powershell
.\Scripts\Common\RunWeb.ps1
```

The web script stops an older `dx serve` process on the requested port before starting a new one. If you run `dx serve` directly and port `8080` is already occupied, stop the old server and restart it so browser testing uses the latest build for this checkout.

## Runtime Notes

- The app supports web and desktop paths.
- Browser template data uses localStorage snapshots.
- Native template data uses SQLite under local `data/`.
- First-time database/schema/seed creation belongs in `create_database_if_missing()`.
- Browser behavior should be validated in a real served app when UI, routing, cache loading, or asset behavior changes.

## Skill Work

Use `.codex/skills/dioxus-authentication/SKILL.md` as the starting point for a project-specific custom skill.

Use `.codex/skills/validate-specs/SKILL.md` to compare Spec Kit specs against the current codebase truth before choosing whether to update code, specs, or both.

Archived Template Project Codex/Spec Kit task specs live under `.specs/template/specs/archive/`; `.specify/feature.json` stays empty when there is no active feature task.

Use `.codex/skills/create-project-from-template/SKILL.md` when creating a new repository from this template. That skill applies `.specs/generated/` over the copied root, renames the project identity, and removes template-only scaffolding from the generated repo.
