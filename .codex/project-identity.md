# Project Identity

Use this file as the first rename checklist when turning this Template Project into a Generated Project.

| Purpose | Current Value | Format |
| ------- | ------------- | ------ |
| Repository slug | `dioxus-authentication` | lowercase kebab-case |
| Display project name | `Dioxus Authentication` | title case |

## Rename Surfaces

- `README.md`, `AGENTS.md`, `.codex/README.md`, `.codex/rules/`, `.codex/skills/`, `.agents/skills/`, and `.specify/`
- `.specs/template/` for Template Project maintenance docs and `.specs/generated/` for Generated Project root replacements
- `Dioxus.toml`, `package.json`, `package-lock.json`, and `.github/workflows/`
- `packages/ui/assets/i18n/`
- `packages/ui/src/client/components/`, `packages/ui/src/client/pages/`, and `packages/ui/src/client/services/`
- `packages/web/README.md`
- `.specs/template/specs/archive/` for archived Template Project specs
- `.specs/generated/specs/` for starter Generated Project specs

Prefer the bundled rename helper in `.codex/skills/create-project-from-template/scripts/Update-ProjectIdentity.ps1` for broad template rename work. It applies `.specs/generated/` before renaming so generated root files are renamed in the same pass.
