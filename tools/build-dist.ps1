# Builds dist\out\HaFeed-Setup.cmd: the single-file installer for other machines.
# Only the feed and its widgets go in. Own package identity and CLSID, so it can sit next to the dev package.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
$Clsid = 'FBACA4FF-FB4B-4136-BBFA-7BA94B052D65'
$WClsid = '1E52DA20-3303-4DC3-909D-0510F029E6C3'   # the widgets class (widgets.rs)
$out = "$root\dist\out"; $stage = "$out\stage"

# 1. Self-check of the policy patch, against the real file from this machine.
$ast = [Management.Automation.Language.Parser]::ParseFile("$root\dist\Setup.template.cmd", [ref]$null, [ref]$null)
foreach ($fn in $ast.FindAll({ $args[0] -is [Management.Automation.Language.FunctionDefinitionAst] -and $args[0].Name -eq 'Get-PatchedPolicy' }, $true)) { . ([scriptblock]::Create($fn.Extent.Text)) }
foreach ($v in $ast.FindAll({ $args[0] -is [Management.Automation.Language.AssignmentStatementAst] -and "$($args[0].Left)" -eq '$Rules' }, $true)) { . ([scriptblock]::Create($v.Extent.Text)) }
if ($Rules.Count -lt 1) { throw '$Rules not found in the template' }
$orig = [IO.File]::ReadAllText("$root\region-policy\original.json")
$patched = Get-PatchedPolicy $orig 'IL'
if ($patched.Replace('"IL", ', '') -ne $orig) { throw 'policy patch changed more than the inserted region codes' }
if ((Get-PatchedPolicy $patched 'IL') -ne $patched) { throw 'policy patch is not idempotent' }
if (([regex]::Matches($patched, '"IL"')).Count -ne $Rules.Count) { throw 'policy patch missed a rule' }

# 2. Exe without local paths in its panic strings.
$env:HZ_CLSID = $Clsid
$env:HZ_WCLSID = $WClsid
$env:CARGO_TARGET_DIR = "$root\target\dist"
$env:RUSTFLAGS = "--remap-path-prefix=$env:USERPROFILE\.cargo\registry\src=crates --remap-path-prefix=$root=src"
cargo build --release -q
if ($LASTEXITCODE) { throw 'build failed' }
Remove-Item Env:HZ_CLSID, Env:HZ_WCLSID, Env:CARGO_TARGET_DIR, Env:RUSTFLAGS
$exe = "$root\target\dist\release\hagizra-widget.exe"
if ([Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($exe)) -match [regex]::Escape($env:USERNAME)) { throw 'exe still contains a local path' }

# 3. Package: dev manifest with the distribution identity.
Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$stage\Images", "$stage\Public" | Out-Null
Copy-Item $exe -Destination $stage
Copy-Item package\Images\logo.png, package\Images\w-*.png -Destination "$stage\Images"
Set-Content "$stage\Public\readme.txt" 'HaFeed' -Encoding ascii   # makeappx drops empty folders
$m = Get-Content package\AppxManifest.xml -Raw
$version = [regex]::Match($m, 'Identity[^>]*Version="([\d.]+)"').Groups[1].Value
$m = $m -replace 'Name="HagizraWidget"', 'Name="HagizraFeed"' `
        -replace 'Publisher="CN=HaFeed Dev"', 'Publisher="CN=Hagizra Feed"' `
        -replace '<PublisherDisplayName>[^<]*', '<PublisherDisplayName>יאנג בוי' `
        -replace '5D2F8A3E-7C41-4B9E-9A63-2E8F0C7D1B54', $Clsid `
        -replace '53219DDD-D127-43D1-8F2A-E0137971AFAF', $WClsid
if ($m -match 'HaFeed Dev') { throw 'manifest still names the dev publisher' }
if ($m -notmatch $WClsid -or $m -match '53219DDD') { throw 'manifest widgets class not replaced' }
[IO.File]::WriteAllText("$stage\AppxManifest.xml", $m, [Text.UTF8Encoding]::new($false))
$makeappx = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\makeappx.exe" | Sort-Object FullName | Select-Object -Last 1
& $makeappx.FullName pack /o /d $stage /p "$out\HagizraFeed.msix" | Out-Null
if ($LASTEXITCODE) { throw 'makeappx failed' }

# 4. Sign. Windows refuses unsigned packages that run an exe, so the installer trusts this certificate
#    (LocalMachine\TrustedPeople only). The private key stays in this user's store; a new one is made if it is lost.
$cert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -eq 'CN=Hagizra Feed' -and $_.NotAfter -gt (Get-Date).AddYears(1) } | Select-Object -First 1
if (-not $cert) {
    $cert = New-SelfSignedCertificate -Type Custom -Subject 'CN=Hagizra Feed' -KeyUsage DigitalSignature -FriendlyName 'Hagizra Feed package signing' `
        -CertStoreLocation Cert:\CurrentUser\My -NotAfter (Get-Date).AddYears(10) -KeyExportPolicy NonExportable `
        -TextExtension @('2.5.29.37={text}1.3.6.1.5.5.7.3.3', '2.5.29.19={text}')
    $cert = Get-Item "Cert:\CurrentUser\My\$($cert.Thumbprint)"   # pwsh 7 returns a deserialized object from the PKI module
}
$signtool = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe" | Sort-Object FullName | Select-Object -Last 1
& $signtool.FullName sign /q /fd SHA256 /sha1 $cert.Thumbprint /s My "$out\HagizraFeed.msix"
if ($LASTEXITCODE) { throw 'signtool failed' }

# 5. One CMD: the template, then the package, the certificate and the logo as base64 inside a PowerShell comment.
$v = $version -replace '\.0$', ''
$cmd = [IO.File]::ReadAllText("$root\dist\Setup.template.cmd").Replace('{{VERSION}}', $v).Replace('{{CERT}}', $cert.Thumbprint) -replace "`r?`n", "`r`n"
$cmd += "<#`r`nCERT:" + [Convert]::ToBase64String($cert.RawData) +
        "`r`nMSIX:" + [Convert]::ToBase64String([IO.File]::ReadAllBytes("$out\HagizraFeed.msix")) +
        "`r`nLOGO:" + [Convert]::ToBase64String([IO.File]::ReadAllBytes("$root\package\Images\logo.png")) + "`r`n#>`r`n"
[IO.File]::WriteAllText("$out\HaFeed-Setup.cmd", $cmd, [Text.UTF8Encoding]::new($false))
Remove-Item $stage -Recurse -Force
"built $out\HaFeed-Setup.cmd ($v, $([int]((Get-Item "$out\HaFeed-Setup.cmd").Length / 1KB)) KB)"
