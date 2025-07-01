# PowerShell script for vrcli development tasks

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

function Format-Code {
    Write-Host "Formatting code..." -ForegroundColor Green
    cargo fmt --all
    Write-Host "Code formatted successfully!" -ForegroundColor Green
}

function Check-Format {
    Write-Host "Checking code formatting..." -ForegroundColor Yellow
    cargo fmt --all -- --check
}

function Run-Clippy {
    Write-Host "Running clippy..." -ForegroundColor Yellow
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::unnecessary-unwrap
}

function Fix-Clippy {
    Write-Host "Running clippy with auto-fix..." -ForegroundColor Yellow
    cargo clippy --fix --allow-dirty --all-targets --all-features -- -D warnings -D clippy::unnecessary-unwrap
}

function Fix-All {
    Write-Host "Fixing all common issues..." -ForegroundColor Cyan
    Format-Code
    Fix-Clippy
    Write-Host "Auto-fix completed!" -ForegroundColor Green
}

function Run-Tests {
    Write-Host "Running tests..." -ForegroundColor Yellow
    cargo test --verbose
}

function Run-All-Checks {
    Write-Host "Running all checks..." -ForegroundColor Cyan
    Format-Code
    Run-Clippy
    Run-Tests
    Write-Host "All checks completed successfully!" -ForegroundColor Green
}

function Run-CI-Local {
    Write-Host "Running CI workflow locally (same as GitHub Actions)..." -ForegroundColor Cyan
    
    Write-Host "Step 1: Format check (strict)" -ForegroundColor Yellow
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Format check failed! Run 'cargo fmt --all' to fix." -ForegroundColor Red
        exit $LASTEXITCODE
    }
    
    Write-Host "Step 2: Run clippy (strict)" -ForegroundColor Yellow
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::unnecessary-unwrap
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Clippy checks failed!" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    
    Write-Host "Step 3: Run tests" -ForegroundColor Yellow
    cargo test --verbose
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Tests failed!" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    
    Write-Host "Local CI checks completed successfully!" -ForegroundColor Green
}

function Build-Project {
    Write-Host "Building project..." -ForegroundColor Yellow
    cargo build
}

function Build-Release {
    Write-Host "Building release..." -ForegroundColor Yellow
    cargo build --release
}

function Clean-Project {
    Write-Host "Cleaning project..." -ForegroundColor Yellow
    cargo clean
}

function Install-Binary {
    Write-Host "Installing binary..." -ForegroundColor Yellow
    cargo install --path .
}

function Setup-GitHooks {
    Write-Host "Setting up git hooks..." -ForegroundColor Cyan
    
    if (!(Test-Path ".git/hooks")) {
        New-Item -ItemType Directory -Path ".git/hooks" -Force
    }
    
    $hookContent = @'
#!/bin/sh
pwsh -File scripts/dev.ps1 pre-commit
'@
    
    $hookContent | Out-File -FilePath ".git/hooks/pre-commit" -Encoding ASCII
    Write-Host "Git pre-commit hook installed!" -ForegroundColor Green
}

function Pre-Commit {
    Write-Host "Running pre-commit checks..." -ForegroundColor Cyan
    Format-Code
    Run-Clippy
    Write-Host "Pre-commit checks completed successfully!" -ForegroundColor Green
}

function Show-Help {
    Write-Host "vrcli Development Script" -ForegroundColor Cyan
    Write-Host "Usage: .\scripts\dev.ps1 <command>" -ForegroundColor White
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Yellow
    Write-Host "  format        - Format code with cargo fmt"
    Write-Host "  check-format  - Check code formatting"
    Write-Host "  clippy        - Run clippy linter"
    Write-Host "  clippy-fix    - Run clippy with auto-fix"
    Write-Host "  fix           - Fix all common issues (format + clippy-fix)"
    Write-Host "  test          - Run tests"
    Write-Host "  check         - Run all checks (format + clippy + test)"
    Write-Host "  ci-local      - Run CI workflow locally (same as GitHub Actions)"
    Write-Host "  build         - Build project"
    Write-Host "  build-release - Build release version"
    Write-Host "  clean         - Clean build artifacts"
    Write-Host "  install       - Install binary"
    Write-Host "  setup-hooks   - Setup git pre-commit hooks"
    Write-Host "  pre-commit    - Run pre-commit checks"
    Write-Host "  help          - Show this help"
}

switch ($Command.ToLower()) {
    "format" { Format-Code }
    "check-format" { Check-Format }
    "clippy" { Run-Clippy }
    "clippy-fix" { Fix-Clippy }
    "fix" { Fix-All }
    "test" { Run-Tests }
    "check" { Run-All-Checks }
    "ci-local" { Run-CI-Local }
    "build" { Build-Project }
    "build-release" { Build-Release }
    "clean" { Clean-Project }
    "install" { Install-Binary }
    "setup-hooks" { Setup-GitHooks }
    "pre-commit" { Pre-Commit }
    "help" { Show-Help }
    default { 
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Show-Help
    }
}
