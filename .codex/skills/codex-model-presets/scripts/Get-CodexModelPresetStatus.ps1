param(
    [string]$ConfigPath,
    [switch]$Refresh,
    [switch]$AsJson
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-RepoRoot {
    return (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
}

function Resolve-ConfigPath {
    param([string]$InputPath)

    if ($InputPath) {
        return (Resolve-Path $InputPath).Path
    }

    return (Join-Path (Resolve-RepoRoot) ".codex\config.toml")
}

function Read-TomlStringValue {
    param(
        [string]$Content,
        [string]$Key
    )

    $match = [regex]::Match($Content, "(?m)^\s*$([regex]::Escape($Key))\s*=\s*""([^""]*)""\s*$")
    if (-not $match.Success) {
        throw "Missing '$Key' in config."
    }

    return $match.Groups[1].Value
}

function Get-PresetInfo {
    param(
        [string]$Model,
        [string]$ReasoningEffort,
        [string]$ReasoningSpeed
    )

    if ($Model -eq "gpt-5.5" -and $ReasoningEffort -eq "medium" -and $ReasoningSpeed -eq "standard") {
        return @{ preset = "paid"; display_name = "Standard Medium Latest Paid" }
    }

    if ($Model -eq "gpt-4o-mini" -and $ReasoningEffort -eq "medium" -and $ReasoningSpeed -eq "standard") {
        return @{ preset = "free"; display_name = "Standard Medium Free" }
    }

    return @{ preset = "custom"; display_name = "Custom Codex Model Setting" }
}

function New-StatusFromConfig {
    param([string]$Path)

    $content = Get-Content -Raw -Path $Path
    $model = Read-TomlStringValue -Content $content -Key "model"
    $reasoningEffort = Read-TomlStringValue -Content $content -Key "model_reasoning_effort"
    $reasoningSpeed = Read-TomlStringValue -Content $content -Key "model_reasoning_speed"
    $preset = Get-PresetInfo -Model $model -ReasoningEffort $reasoningEffort -ReasoningSpeed $reasoningSpeed
    $configItem = Get-Item -Path $Path

    return [ordered]@{
        preset = $preset.preset
        display_name = $preset.display_name
        model = $model
        model_reasoning_effort = $reasoningEffort
        model_reasoning_speed = $reasoningSpeed
        config_path = $configItem.FullName
        config_last_write_time_utc = $configItem.LastWriteTimeUtc.ToString("o")
        cache_created_at_utc = (Get-Date).ToUniversalTime().ToString("o")
        source = "config"
    }
}

$path = Resolve-ConfigPath -InputPath $ConfigPath
if (-not (Test-Path $path)) {
    throw "Config file not found: $path"
}

$repoRoot = Resolve-RepoRoot
$cacheDirectory = Join-Path $repoRoot ".codex\cache\codex-model-presets"
$cachePath = Join-Path $cacheDirectory "current-setting.json"
New-Item -ItemType Directory -Force -Path $cacheDirectory | Out-Null

$configItem = Get-Item -Path $path
$useCache = $false
if ((-not $Refresh) -and (Test-Path $cachePath)) {
    $cacheItem = Get-Item -Path $cachePath
    $useCache = $cacheItem.LastWriteTimeUtc -gt $configItem.LastWriteTimeUtc
}

if ($useCache) {
    $status = Get-Content -Raw -Path $cachePath | ConvertFrom-Json -AsHashtable
    $status["source"] = "cache"
} else {
    $status = New-StatusFromConfig -Path $path
    $status | ConvertTo-Json -Compress | Set-Content -Path $cachePath
}

if ($AsJson) {
    $status | ConvertTo-Json -Compress
    return
}

Write-Host "Preset: $($status.display_name)"
Write-Host "Model: $($status.model)"
Write-Host "Reasoning: $($status.model_reasoning_effort), $($status.model_reasoning_speed)"
Write-Host "Source: $($status.source)"
