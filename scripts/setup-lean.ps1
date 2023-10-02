# Lean 4 / elan helpers for Windows.
# elan often fails to download toolchains behind slow or proxied networks.
# Use -InstallFromArchive when you have already fetched lean-<version>-windows.tar.zst.

param(
    [string]$InstallFromArchive,
    [string]$Version = "v4.16.0"
)

$ErrorActionPreference = "Stop"

$elanBin = Join-Path $env:USERPROFILE ".elan\bin"
if (-not (Test-Path (Join-Path $elanBin "elan.exe"))) {
    Write-Error "elan not found at $elanBin. Install elan first: https://github.com/leanprover/elan"
    exit 1
}
$env:Path = "$elanBin;$env:Path"

function Install-LeanToolchainFromArchive {
    param(
        [Parameter(Mandatory = $true)][string]$ArchivePath,
        [Parameter(Mandatory = $true)][string]$ToolchainVersion
    )

    if (-not (Test-Path -LiteralPath $ArchivePath)) {
        Write-Error "Archive not found: $ArchivePath"
        exit 1
    }

    $toolchainId = "leanprover--lean4---$ToolchainVersion"
    $dest = Join-Path $env:USERPROFILE ".elan\toolchains\$toolchainId"

    Write-Host "Installing $ToolchainVersion from $ArchivePath"
    Write-Host "Destination: $dest"

    if (Test-Path -LiteralPath $dest) {
        Write-Host "Removing existing toolchain directory..."
        Remove-Item -LiteralPath $dest -Recurse -Force
    }
    New-Item -ItemType Directory -Path $dest -Force | Out-Null

    tar -xf $ArchivePath --strip-components=1 -C $dest

    if (-not (Test-Path (Join-Path $dest "bin\lean.exe"))) {
        Write-Error "Extract failed: bin\lean.exe missing under $dest"
        exit 1
    }

    Write-Host "Installed. Registered toolchains:"
    elan toolchain list
}

if ($InstallFromArchive) {
    Install-LeanToolchainFromArchive -ArchivePath $InstallFromArchive -ToolchainVersion $Version
}

Write-Host "elan: $(elan --version)"
Write-Host "lean: $(lean --version)"
Write-Host "lake: $(lake --version)"
