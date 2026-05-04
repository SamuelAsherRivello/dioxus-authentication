param(
    [string]$Address = "",
    [int]$Port = 8080,
    [switch]$Lan
)

$ErrorActionPreference = "Stop"

function Resolve-RunWebAddress {
    param(
        [string]$Address = "",
        [string]$WifiAddress = "",
        [switch]$Lan
    )

    if ([string]::IsNullOrWhiteSpace($Address)) {
        if ($Lan -and -not [string]::IsNullOrWhiteSpace($WifiAddress)) {
            return [pscustomobject]@{
                Address = $WifiAddress
                IsLan = $true
                Warning = ""
            }
        }

        return [pscustomobject]@{
            Address = "127.0.0.1"
            IsLan = $false
            Warning = ""
        }
    }

    if ($Address -eq "0.0.0.0") {
        return [pscustomobject]@{
            Address = "127.0.0.1"
            IsLan = $false
            Warning = "Fullstack backend readiness can fail on Windows when using 0.0.0.0. Using localhost URL mode instead."
        }
    }

    return [pscustomobject]@{
        Address = $Address
        IsLan = ($Address -eq $WifiAddress -and -not [string]::IsNullOrWhiteSpace($WifiAddress))
        Warning = ""
    }
}

function Get-RunWebDisplayLines {
    param(
        [string]$Address,
        [int]$Port,
        [bool]$IsLan
    )

    $lines = [System.Collections.Generic.List[string]]::new()

    if ($IsLan) {
        $lines.Add("Starting web app for laptop and phone testing on the same Wi-Fi.")
        $lines.Add("Laptop: http://$Address`:$Port")
        $lines.Add("Phone:  http://$Address`:$Port")
        $lines.Add("Passkeys: use localhost mode without -Lan for WebAuthn testing.")
    } else {
        $lines.Add("Starting web app for passkey testing on localhost.")
        $lines.Add("Passkeys: http://localhost:$Port")
        $lines.Add("Laptop:   http://localhost:$Port")
        $lines.Add("Phone:    not available unless you pass -Lan or this laptop's Wi-Fi IPv4 address with -Address.")
    }

    return $lines
}

function Get-RunWebOpenUrl {
    param(
        [string]$Address,
        [int]$Port,
        [bool]$IsLan
    )

    if ($IsLan) {
        return "http://$Address`:$Port/"
    }

    return "http://localhost:$Port/"
}

function Open-RunWebBrowser {
    param(
        [string]$Url
    )

    $command = "Start-Sleep -Seconds 2; Start-Process '$Url'"
    $encodedCommand = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($command))
    Start-Process -FilePath "powershell.exe" -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-EncodedCommand", $encodedCommand) -WindowStyle Hidden | Out-Null
}

function Stop-ProcessById {
    param(
        [int]$ProcessId,
        [string]$Reason
    )

    if ($ProcessId -le 0 -or $ProcessId -eq $PID) {
        return
    }

    $process = Get-CimInstance Win32_Process -Filter "ProcessId = $ProcessId" -ErrorAction SilentlyContinue
    if (-not $process) {
        return
    }

    Write-Host "Stopping $($process.Name) process $ProcessId ($Reason)."
    Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
    Wait-Process -Id $ProcessId -Timeout 5 -ErrorAction SilentlyContinue
}

function Invoke-RunWeb {
    param(
        [string]$Address = "",
        [int]$Port = 8080,
        [switch]$Lan
    )

    $repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
    Set-Location $repoRoot

    Get-CimInstance Win32_Process -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -ieq "dx.exe" -and $_.CommandLine -match "\bserve\b" } |
        ForEach-Object {
            Stop-ProcessById -ProcessId $_.ProcessId -Reason "existing Dioxus server"
        }

    Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue |
        Select-Object -ExpandProperty OwningProcess -Unique |
        ForEach-Object {
            Stop-ProcessById -ProcessId $_ -Reason "port $Port listener"
        }

    $wifiAddress = Get-NetIPConfiguration |
        Where-Object { $_.IPv4DefaultGateway -and $_.NetAdapter.Status -eq "Up" } |
        Select-Object -ExpandProperty IPv4Address -First 1 |
        Select-Object -ExpandProperty IPAddress

    $addressResult = Resolve-RunWebAddress -Address $Address -WifiAddress $wifiAddress -Lan:$Lan
    $Address = $addressResult.Address

    if (-not [string]::IsNullOrWhiteSpace($addressResult.Warning)) {
        Write-Host $addressResult.Warning
    }

    Get-RunWebDisplayLines -Address $Address -Port $Port -IsLan $addressResult.IsLan |
        ForEach-Object { Write-Host $_ }
    Write-Host ""

    Open-RunWebBrowser -Url (Get-RunWebOpenUrl -Address $Address -Port $Port -IsLan $addressResult.IsLan)

    dx serve --platform web --addr $Address --port $Port --open false
}

if ($MyInvocation.InvocationName -ne ".") {
    Invoke-RunWeb -Address $Address -Port $Port -Lan:$Lan
}
