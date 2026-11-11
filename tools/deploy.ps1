# Build, bump the package version, copy to D:\tools\hagizra-widget and re-register.
# The platform relaunches the provider by itself, so the copy retries until the exe is free.
param([string]$Dst = "$env:LOCALAPPDATA\HaFeed-dev")
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path -Parent $PSScriptRoot)
# A deploy kills the provider, and with it a Google sign-in in progress. Edge holds this file while the
# sign-in window is open. (Not the Edge command line: reading it goes through WMI, which hangs here.)
$lock = "$env:LOCALAPPDATA\HagizraFeed\signin\lockfile"
if (Test-Path $lock) { try { [IO.File]::Open($lock, 'Open', 'ReadWrite', 'None').Close() } catch { throw "sign-in window open: deploy later" } }
cargo build --release -q
if ($LASTEXITCODE) { throw "build failed" }
$m = Get-Content package\AppxManifest.xml -Raw
$v = [regex]::Match($m, 'Identity[^>]*Version="(\d+)\.(\d+)\.(\d+)\.0"')
$next = "$($v.Groups[1]).$($v.Groups[2]).$([int]$v.Groups[3].Value + 1).0"
$m -replace '(Identity[^>]*Version=")[\d.]+(")', "`${1}$next`$2" | Set-Content package\AppxManifest.xml -NoNewline
New-Item -ItemType Directory -Force "$Dst\Images", "$Dst\Public" | Out-Null
Copy-Item package\Images\* "$Dst\Images" -Force
for ($i = 0; $i -lt 10; $i++) {
    Get-Process hagizra-widget -ErrorAction SilentlyContinue | Stop-Process -Force
    try { Copy-Item target\release\hagizra-widget.exe $Dst -Force; break } catch { Start-Sleep -Milliseconds 400 }
}
Copy-Item package\AppxManifest.xml $Dst -Force
Add-AppxPackage -Register "$Dst\AppxManifest.xml"
"deployed $next"
