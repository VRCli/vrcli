[Console]::OutputEncoding = [Console]::InputEncoding = $OutputEncoding = [Text.Encoding]::UTF8
$PSDefaultParameterValues['*:Encoding'] = 'utf8'

Write-Host "Fetching online friends..." -ForegroundColor Green

# Create StartProcess helper to hide console windows
$startProcess = {
    param($path, $arguments)
    $psi = [Diagnostics.ProcessStartInfo]::new($path)
    $psi.Arguments = $arguments
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $proc = [Diagnostics.Process]::new()
    $proc.StartInfo = $psi
    $proc.Start() | Out-Null
    $stdout = $proc.StandardOutput.ReadToEnd()
    $stderr = $proc.StandardError.ReadToEnd()
    $proc.WaitForExit()
    return @{ExitCode=$proc.ExitCode;Output=$stdout;Error=$stderr}
}

$vrcli = Get-Command vrcli -ErrorAction Stop | ForEach-Object Source

# Get online friends with location data
$result = & $startProcess $vrcli 'friends list --online --json --show-location'
if ($result.ExitCode) {
    Write-Error "Friend fetch failed (Error $($result.ExitCode))`n$($result.Error)"
    Write-Host "Run 'vrcli auth login' and ensure vrcli in PATH" -f Yellow
    exit $result.ExitCode
}

try {
    $friends = $result.Output | ConvertFrom-Json | Where-Object {
        (-not $IncludePrivate) -bor
        ($_.location -notmatch 'private|offline' -and 
         $_.display_name -and $_.display_name.Trim())
    }
} 
catch { 
    Write-Error "JSON parse failed: $_"
    Write-Host "Raw output:`n$($result.Output)" -f Yellow
    exit 1
}

if (-not $friends) {
    Write-Host "No friends found online${('',' in non-private instances')[$(-not $IncludePrivate)]}." -f Yellow
    exit
}

# Select random friend and sanitize name
$name = ($friends | Get-Random).display_name.Trim() -replace '[^\p{L}\p{N}\p{P}\p{S}\s]'
Write-Host "Sending invite request to: $name" -f Green

# Send invite using the same helper
$inviteResult = & $startProcess $vrcli "invite request `"$name`""
if ($inviteResult.ExitCode) {
    Write-Error "Invite failed (Error $($inviteResult.ExitCode))`n$($inviteResult.Error)"
    exit $inviteResult.ExitCode
} 
elseif ($inviteResult.Output) {
    Write-Host $inviteResult.Output
}