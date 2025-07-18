# PowerShell version of pre-commit hook for Windows users
# This script performs the same checks as the git pre-commit hook

param(
    [switch]$Fix = $false
)

Write-Host "Running pre-commit checks..." -ForegroundColor Blue

# Check formatting
Write-Host "Checking code formatting..." -ForegroundColor Yellow
$formatResult = cargo fmt --check 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Code is not formatted properly!" -ForegroundColor Red
    if ($Fix) {
        Write-Host "Fixing formatting..." -ForegroundColor Yellow
        cargo fmt --all
        Write-Host "Code has been formatted. Please review and commit the changes." -ForegroundColor Green
    } else {
        Write-Host "Run 'cargo fmt' or '.\scripts\pre-commit.ps1 -Fix' to fix formatting." -ForegroundColor Yellow
        exit 1
    }
}
Write-Host "✓ Code formatting is correct" -ForegroundColor Green

# Run clippy
Write-Host "Running clippy checks..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "Clippy found issues!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Clippy checks passed" -ForegroundColor Green

Write-Host "All pre-commit checks passed!" -ForegroundColor Green
