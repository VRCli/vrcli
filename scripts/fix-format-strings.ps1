#!/usr/bin/env pwsh

# Script to fix uninlined format args in Rust code
# This script fixes the common clippy::uninlined_format_args warnings

Write-Host "Fixing uninlined format args..." -ForegroundColor Yellow

# Get all Rust files
$rustFiles = Get-ChildItem -Path "src" -Filter "*.rs" -Recurse

foreach ($file in $rustFiles) {
    Write-Host "Processing $($file.FullName)..." -ForegroundColor Gray
    
    $content = Get-Content $file.FullName -Raw
    $originalContent = $content
    
    # Fix common format! patterns
    $content = $content -replace 'format!\("([^"]*)\{\}([^"]*)", ([^)]+)\)', 'format!("$1{$3}$2")'
    $content = $content -replace 'println!\("([^"]*)\{\}([^"]*)", ([^)]+)\)', 'println!("$1{$3}$2")'
    $content = $content -replace 'print!\("([^"]*)\{\}([^"]*)", ([^)]+)\)', 'print!("$1{$3}$2")'
    
    # Write back if changed
    if ($content -ne $originalContent) {
        Write-Host "  Updated!" -ForegroundColor Green
        $content | Out-File -FilePath $file.FullName -Encoding UTF8 -NoNewline
    }
}

Write-Host "Done! Run 'cargo clippy' to check for remaining issues." -ForegroundColor Green
