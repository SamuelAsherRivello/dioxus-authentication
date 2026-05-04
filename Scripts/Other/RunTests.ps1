$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $repoRoot

. (Join-Path $repoRoot "Scripts\Common\RunWeb.ps1")

function Assert-Equal {
    param(
        [object]$Actual,
        [object]$Expected,
        [string]$Message
    )

    if ($Actual -ne $Expected) {
        throw "$Message Expected '$Expected' but got '$Actual'."
    }
}

function Assert-Contains {
    param(
        [string[]]$Values,
        [string]$Expected,
        [string]$Message
    )

    if ($Values -notcontains $Expected) {
        throw "$Message Missing '$Expected'."
    }
}

$defaultAddress = Resolve-RunWebAddress -Address "" -WifiAddress "192.168.1.44"
Assert-Equal $defaultAddress.Address "127.0.0.1" "Default web address should bind to Dioxus-supported loopback IP."
Assert-Equal $defaultAddress.IsLan $false "Default web address should not be LAN mode."

$defaultLines = Get-RunWebDisplayLines -Address $defaultAddress.Address -Port 8080 -IsLan $defaultAddress.IsLan
Assert-Contains $defaultLines "Passkeys: http://localhost:8080" "Default output should show localhost passkey URL."
$defaultOpenUrl = Get-RunWebOpenUrl -Address $defaultAddress.Address -Port 8080 -IsLan $defaultAddress.IsLan
Assert-Equal $defaultOpenUrl "http://localhost:8080/" "Default web browser should open localhost."

$lanAddress = Resolve-RunWebAddress -Address "" -WifiAddress "192.168.1.44" -Lan
Assert-Equal $lanAddress.Address "192.168.1.44" "LAN mode should use Wi-Fi address when available."
Assert-Equal $lanAddress.IsLan $true "LAN mode should mark Wi-Fi address as LAN."

$lanLines = Get-RunWebDisplayLines -Address $lanAddress.Address -Port 8080 -IsLan $lanAddress.IsLan
Assert-Contains $lanLines "Passkeys: use localhost mode without -Lan for WebAuthn testing." "LAN output should warn that passkey testing needs localhost mode."
$lanOpenUrl = Get-RunWebOpenUrl -Address $lanAddress.Address -Port 8080 -IsLan $lanAddress.IsLan
Assert-Equal $lanOpenUrl "http://192.168.1.44:8080/" "LAN web browser should open the LAN URL."

$zeroAddress = Resolve-RunWebAddress -Address "0.0.0.0" -WifiAddress "192.168.1.44"
Assert-Equal $zeroAddress.Address "127.0.0.1" "0.0.0.0 should be replaced with loopback IP."
Assert-Equal $zeroAddress.Warning "Fullstack backend readiness can fail on Windows when using 0.0.0.0. Using localhost URL mode instead." "0.0.0.0 should explain the replacement."

Write-Host "RunWeb script tests passed."
cargo test -p authentication --lib
cargo test -p ui --lib --test tests
