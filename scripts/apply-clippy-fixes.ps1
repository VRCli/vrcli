#!/usr/bin/env pwsh

# Script to apply the specific fixes needed for clippy::uninlined_format_args

$files = @(
    "src\commands\friends\handlers.rs",
    "src\commands\users\handlers.rs", 
    "src\common\command_utils.rs",
    "src\common\formatter.rs"
)

foreach ($file in $files) {
    if (Test-Path $file) {
        Write-Host "Fixing $file..." -ForegroundColor Yellow
        
        $content = Get-Content $file -Raw
        
        # Apply specific fixes for each file
        $content = $content -replace 'println!\("Successfully unfriended user \{\}", user_id\)', 'println!("Successfully unfriended user {user_id}")'
        $content = $content -replace 'println!\("Successfully cancelled friend request to \{\}", user_id\)', 'println!("Successfully cancelled friend request to {user_id}")'
        $content = $content -replace 'println!\(\s*"No friendship or outgoing friend request found with user \{\}",\s*user_id\s*\);', 'println!("No friendship or outgoing friend request found with user {user_id}");'
        $content = $content -replace 'println!\("Friend status with user \{\}:", user_id\)', 'println!("Friend status with user {user_id}:")'
        
        $content = $content -replace 'println!\("Username: \{\}", username\)', 'println!("Username: {username}")'
        $content = $content -replace 'println!\("Status: \{\}", colored_status\)', 'println!("Status: {colored_status}")'
        $content = $content -replace 'println!\("Platform: \{\}", formatted_platform\)', 'println!("Platform: {formatted_platform}")'
        $content = $content -replace 'println!\("No note found for user: \{\}", identifier\)', 'println!("No note found for user: {identifier}")'
        $content = $content -replace 'println!\("No feedback found for user: \{\}", identifier\)', 'println!("No feedback found for user: {identifier}")'
        $content = $content -replace 'println!\("Feedback for user \{\}:", identifier\)', 'println!("Feedback for user {identifier}:")'
        
        $content = $content -replace 'println!\("\{\}", context_message\)', 'println!("{context_message}")'
        $content = $content -replace 'print!\("\{\}", table_output\)', 'print!("{table_output}")'
        
        $content | Out-File -FilePath $file -Encoding UTF8 -NoNewline
        Write-Host "  Fixed!" -ForegroundColor Green
    }
}

Write-Host "All fixes applied!" -ForegroundColor Green
