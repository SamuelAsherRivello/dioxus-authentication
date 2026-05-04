---
name: create-project-from-template
description: "Create a new local git repository from this Dioxus Authentication. Use when the user wants to copy this template into a new repo, confirm the destination repo name and display project name, update project naming, and commit once as Initial Commit."
---

# Create Project From Template

## Overview

Use this skill to create a sibling local git repository from the current `dioxus-authentication` checkout, apply the Generated Project payload, rename it to the requested project identity, and make exactly one initial commit.

## Required Confirmation

Before creating anything, require the user to confirm both values in one message:

- New git repository name, for example `dioxus-bitcoin-lightning-game`.
- New display project name, for example `Dioxus Bitcoin Lightning Game`.

If either value is missing or ambiguous, ask for it and stop. Do not infer the display name from the repo name without confirmation.

## Workflow

1. Read `AGENTS.md`, `.codex/project-identity.md`, `.specs/template/README.md`, and `.specs/generated/README.md`.
2. Confirm the current directory is the template repo.
3. Run `git status --short`. If the source template has uncommitted changes, stop and ask whether to continue after the user commits or stashes them.
4. Derive the destination as a sibling folder named with the confirmed repo name unless the user explicitly names a different path.
5. If the destination exists and is not empty, stop. Do not delete, clean, overwrite, or reuse it.
6. Copy tracked template contents without history:

```powershell
New-Item -ItemType Directory -Path <destination> -Force | Out-Null
git archive --format=tar HEAD | tar -xf - -C <destination>
```

7. Initialize the new repository:

```powershell
git -C <destination> init
```

8. Run the bundled helper from the new repository. It copies `.specs/generated/` over the repo root, replaces root `specs/` when `.specs/generated/specs/` exists, renames project identity, and removes `.specs/` from the new repo:

```powershell
.\.codex\skills\create-project-from-template\scripts\Update-ProjectIdentity.ps1 `
  -RepoRoot <destination> `
  -NewRepoName "<repo-name>" `
  -NewProjectName "<project-name>"
```

9. Inspect key rename results with `rg`:

```powershell
rg -n "dioxus-authentication|Dioxus Authentication" <destination> --glob '!target/**' --glob '!node_modules/**' --glob '!.git/**'
```

Expected matches should be limited to intentional old-name references in the rename helper defaults. Update any live project surfaces that still use the old identity.

10. Run focused validation in the new repo:

```powershell
cargo check -p ui --target wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
```

11. Commit once:

```powershell
git -C <destination> add -A
git -C <destination> commit -m "Initial Commit: <Project Name>"
```

## Safety Rules

- Never run `git reset --hard`, `git clean`, force checkout, rebase, squash, amend, force-push, or history rewrite commands.
- Never delete or overwrite an existing destination repository.
- The helper may remove only the copied `.specs/` directory and copied root `specs/` before replacing it with `.specs/generated/specs/`; it must verify those paths are under the destination root first.
- Keep temporary files inside the destination repo or this template repo.
- Do not create a GitHub repository unless the user explicitly asks for GitHub publishing in the same request.
