[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$RepoRoot,

    [Parameter(Mandatory = $true)]
    [ValidatePattern('^[a-z0-9]+(-[a-z0-9]+)*$')]
    [string]$NewRepoName,

    [Parameter(Mandatory = $true)]
    [ValidateNotNullOrEmpty()]
    [string]$NewProjectName,

    [string]$OldRepoName = 'dioxus-authentication',
    [string]$OldProjectName = 'Dioxus Authentication'
)

$ErrorActionPreference = 'Stop'

$resolvedRoot = (Resolve-Path -LiteralPath $RepoRoot).Path
$gitDir = Join-Path $resolvedRoot '.git'

if (-not (Test-Path -LiteralPath $resolvedRoot -PathType Container)) {
    throw "RepoRoot does not exist: $RepoRoot"
}

function Test-UnderRoot {
    param([string]$Path)
    $resolved = (Resolve-Path -LiteralPath $Path).Path
    if ($resolved.Equals($resolvedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        return $true
    }

    $rootWithSeparator = $resolvedRoot.TrimEnd([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar) + [System.IO.Path]::DirectorySeparatorChar
    return $resolved.StartsWith($rootWithSeparator, [System.StringComparison]::OrdinalIgnoreCase)
}

$specsRoot = Join-Path $resolvedRoot '.specs'
$generatedRoot = Join-Path $specsRoot 'generated'
if (Test-Path -LiteralPath $generatedRoot -PathType Container) {
    if (-not (Test-UnderRoot -Path $generatedRoot)) {
        throw "Refusing to read a generated payload outside RepoRoot: $generatedRoot"
    }

    $generatedSpecs = Join-Path $generatedRoot 'specs'
    $rootSpecs = Join-Path $resolvedRoot 'specs'
    if ((Test-Path -LiteralPath $generatedSpecs -PathType Container) -and (Test-Path -LiteralPath $rootSpecs -PathType Container)) {
        if (-not (Test-UnderRoot -Path $rootSpecs)) {
            throw "Refusing to replace a specs directory outside RepoRoot: $rootSpecs"
        }
        Remove-Item -LiteralPath $rootSpecs -Recurse -Force
    }

    Get-ChildItem -LiteralPath $generatedRoot -Force | ForEach-Object {
        $destination = Join-Path $resolvedRoot $_.Name
        Copy-Item -LiteralPath $_.FullName -Destination $destination -Recurse -Force
    }
}

$excludedDirs = @(
    '.git',
    'target',
    'node_modules',
    'data',
    'test-results',
    '.codex/tmp',
    '.specs'
)

$textExtensions = @(
    '.css', '.ftl', '.html', '.js', '.json', '.lock', '.md', '.ps1', '.rs', '.toml', '.txt', '.yaml', '.yml'
)

$files = Get-ChildItem -LiteralPath $resolvedRoot -Recurse -File -Force | Where-Object {
    $relative = [System.IO.Path]::GetRelativePath($resolvedRoot, $_.FullName).Replace('\', '/')
    $isExcluded = $false
    foreach ($dir in $excludedDirs) {
        if ($relative -eq $dir -or $relative.StartsWith("$dir/", [System.StringComparison]::OrdinalIgnoreCase)) {
            $isExcluded = $true
            break
        }
    }

    (-not $isExcluded) -and ($textExtensions -contains $_.Extension.ToLowerInvariant())
}

foreach ($file in $files) {
    if (-not (Test-UnderRoot -Path $file.FullName)) {
        throw "Refusing to edit a file outside RepoRoot: $($file.FullName)"
    }

    $content = Get-Content -LiteralPath $file.FullName -Raw
    $updated = $content.Replace($OldProjectName, $NewProjectName).
        Replace($OldRepoName, $NewRepoName)

    if ($updated -ne $content) {
        Set-Content -LiteralPath $file.FullName -Value $updated -NoNewline
    }
}

$oldSkillDir = Join-Path $resolvedRoot ".codex/skills/$OldRepoName"
$newSkillDir = Join-Path $resolvedRoot ".codex/skills/$NewRepoName"
if ((Test-Path -LiteralPath $oldSkillDir -PathType Container) -and -not (Test-Path -LiteralPath $newSkillDir)) {
    if (-not (Test-UnderRoot -Path $oldSkillDir)) {
        throw "Refusing to move a skill directory outside RepoRoot: $oldSkillDir"
    }
    Move-Item -LiteralPath $oldSkillDir -Destination $newSkillDir
}

$templateOnlyDirs = @(
    $specsRoot
)

foreach ($templateOnlyDir in $templateOnlyDirs) {
    if (Test-Path -LiteralPath $templateOnlyDir -PathType Container) {
        if (-not (Test-UnderRoot -Path $templateOnlyDir)) {
            throw "Refusing to remove a directory outside RepoRoot: $templateOnlyDir"
        }
        Remove-Item -LiteralPath $templateOnlyDir -Recurse -Force
    }
}

if (Test-Path -LiteralPath $gitDir -PathType Container) {
    git -C $resolvedRoot status --short | Out-Host
}
