---
name: codex-model-presets
description: Switch Codex model defaults between predefined presets in the current repo (`Standard Medium Latest Paid` and `Standard Medium Free`) by running the provided script and confirm any runtime restart requirement.
---

# Codex model presets

## Use when
- You need to switch the active Codex model configuration for this repository.
- You want to choose between a paid and free preset quickly.

## Start here

Before reading `.codex/config.toml`, run the status script from the repo root:

```powershell
.\.codex\skills\codex-model-presets\scripts\Get-CodexModelPresetStatus.ps1
```

The script checks `.codex/cache/codex-model-presets/current-setting.json` first. If that cache file is newer than `.codex/config.toml`, it returns the cached model setting. If `.codex/config.toml` is newer, it recreates the cache from the TOML and returns the refreshed setting.

## Presets

| Preset | Model | Reasoning Effort | Reasoning Speed |
|---|---|---|---|
| Standard Medium Latest Paid | `gpt-5.5` | `medium` | `standard` |
| Standard Medium Free | `gpt-4o-mini` | `medium` | `standard` |

## Quick command

Run from repo root:

```powershell
.\.codex\skills\codex-model-presets\scripts\Set-CodexModelPreset.ps1
```

You can skip the prompt by passing `-Preset`:

```powershell
.\.codex\skills\codex-model-presets\scripts\Set-CodexModelPreset.ps1 -Preset paid
.\.codex\skills\codex-model-presets\scripts\Set-CodexModelPreset.ps1 -Preset free
```

Optional:

```powershell
.\.codex\skills\codex-model-presets\scripts\Set-CodexModelPreset.ps1 -Preset paid -ConfigPath <path>\.codex\config.toml
```

## Expected behavior
| Script | Behavior |
| ------ | -------- |
| `Get-CodexModelPresetStatus.ps1` | Returns the active model setting from `.codex/cache/codex-model-presets/current-setting.json` when the cache is newer than `.codex/config.toml`; otherwise rebuilds the cache from `.codex/config.toml`. |
| `Set-CodexModelPreset.ps1` | Offers `1) Standard Medium Latest Paid` and `2) Standard Medium Free` when `-Preset` is omitted, updates `model`, `model_reasoning_effort`, and `model_reasoning_speed`, refreshes the cache, and prints whether a terminal/CLI restart is needed. |
