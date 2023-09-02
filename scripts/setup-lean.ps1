# Add elan (Lean 4) to the current PowerShell session PATH.
$elanBin = Join-Path $env:USERPROFILE ".elan\bin"
if (-not (Test-Path (Join-Path $elanBin "elan.exe"))) {
    Write-Error "elan not found at $elanBin. Install elan (Lean toolchain manager) first."
    exit 1
}
$env:Path = "$elanBin;$env:Path"
Write-Host "elan: $(elan --version)"
Write-Host "lean: $(lean --version)"
Write-Host "lake: $(lake --version)"
