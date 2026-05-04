param(
    [ValidateSet("paid","free")]
    [string]$Preset,
    [string]$ConfigPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-ConfigPath {
    param([string]$InputPath)

    if ($InputPath) {
        return (Resolve-Path $InputPath).Path
    }

    $candidate = Join-Path (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path ".codex\config.toml"
    return $candidate
}

function Read-Preset {
    param([string]$Selection)

    switch ($Selection) {
        "paid" { return @{ model = "gpt-5.5"; reasoning_effort = "medium"; reasoning_speed = "standard" } }
        "free" { return @{ model = "gpt-4o-mini"; reasoning_effort = "medium"; reasoning_speed = "standard" } }
        default { throw "Unknown preset '$Selection'." }
    }
}

if (-not $Preset) {
    Write-Host "Select Codex preset:"
    Write-Host "1) Standard Medium Latest Paid"
    Write-Host "2) Standard Medium Free"
    $choice = Read-Host "Enter 1 or 2"
    switch ($choice.Trim()) {
        "1" { $Preset = "paid" }
        "2" { $Preset = "free" }
        default { throw "Invalid selection '$choice'. Use 1 or 2." }
    }
}

$selected = Read-Preset -Selection $Preset
$path = Resolve-ConfigPath -InputPath $ConfigPath

if (-not (Test-Path $path)) {
    throw "Config file not found: $path"
}

$content = Get-Content -Raw -Path $path

$content = [regex]::Replace($content, '(?ms)^model\s*=\s*".*?"\r?\n?', '', 1)
$content = [regex]::Replace($content, '(?ms)^model_reasoning_effort\s*=\s*".*?"\r?\n?', '', 1)
$content = [regex]::Replace($content, '(?ms)^model_reasoning_speed\s*=\s*".*?"\r?\n?', '', 1)

$trimmed = $content.Trim()
$updatedLines = @(
    "model = ""$($selected.model)"""
    "model_reasoning_effort = ""$($selected.reasoning_effort)"""
    "model_reasoning_speed = ""$($selected.reasoning_speed)"""
)

if ($trimmed) {
    $updatedLines += ""
    $updatedLines += $trimmed
}

$updated = $updatedLines -join [Environment]::NewLine

Set-Content -Path $path -Value $updated
$statusScript = Join-Path $PSScriptRoot "Get-CodexModelPresetStatus.ps1"
& $statusScript -ConfigPath $path -Refresh -AsJson | Out-Null

Write-Host "Updated: $path"
Write-Host "Preset: $Preset"
Write-Host "  model = $($selected.model)"
Write-Host "  model_reasoning_effort = $($selected.reasoning_effort)"
Write-Host "  model_reasoning_speed = $($selected.reasoning_speed)"
Write-Host "Cache: .codex\cache\codex-model-presets\current-setting.json"
Write-Host "If Codex is already running, restart your shell/IDE session to load the new defaults."
