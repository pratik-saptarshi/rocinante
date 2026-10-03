$ErrorActionPreference = 'Stop'

$schemeKey = 'HKCU:\Software\Classes\rocinante'
if (Test-Path -LiteralPath $schemeKey) {
    throw "Refusing to overwrite an existing per-user rocinante protocol registration: $schemeKey"
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$installer = Join-Path $repoRoot 'src-tauri\crates\rocinante-desktop-shell\packaging\windows\install-user.ps1'
$testId = [guid]::NewGuid().ToString('N')
$testRoot = Join-Path $env:RUNNER_TEMP "rocinante-registration-$testId"
$localAppData = Join-Path $testRoot 'local-app-data'
$sourceBinary = Join-Path $testRoot 'fixture.exe'
$previousLocalAppData = $env:LOCALAPPDATA
$previousAppData = $env:APPDATA

try {
    New-Item -ItemType Directory -Path $testRoot -Force | Out-Null
    [System.IO.File]::WriteAllText($sourceBinary, 'Rocinante Windows registration fixture')
    $env:LOCALAPPDATA = $localAppData
    $env:APPDATA = Join-Path $testRoot 'roaming-app-data'

    & $installer -Executable $sourceBinary

    $installedBinary = Join-Path $localAppData 'Programs\Rocinante\rocinante-desktop-shell.exe'
    if (-not (Test-Path -LiteralPath $installedBinary -PathType Leaf)) {
        throw "Installer did not copy the executable to $installedBinary"
    }
    if ((Get-Content -LiteralPath $installedBinary -Raw) -ne 'Rocinante Windows registration fixture') {
        throw 'Installed executable contents do not match the source fixture.'
    }
    $installedIcon = Join-Path $localAppData 'Programs\Rocinante\Rocinante.ico'
    if (-not (Test-Path -LiteralPath $installedIcon -PathType Leaf)) {
        throw "Installer did not copy the application icon to $installedIcon"
    }
    $shortcutPath = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\Rocinante Repo Analyzer.lnk'
    if (-not (Test-Path -LiteralPath $shortcutPath -PathType Leaf)) {
        throw "Installer did not create the Start Menu shortcut at $shortcutPath"
    }
    $shortcut = (New-Object -ComObject WScript.Shell).CreateShortcut($shortcutPath)
    $expectedIconLocation = '"' + $installedIcon + '",0'
    if ($shortcut.TargetPath -ne $installedBinary -or $shortcut.IconLocation -ne $expectedIconLocation) {
        throw 'Start Menu shortcut does not point to the installed executable and icon.'
    }

    $protocol = Get-Item -LiteralPath $schemeKey
    if ($protocol.GetValue('') -ne 'URL:Rocinante Repository') {
        throw 'Protocol registry key has an unexpected description.'
    }
    if ($protocol.GetValueNames() -notcontains 'URL Protocol') {
        throw 'Protocol registry key is missing the URL Protocol marker.'
    }
    $defaultIcon = (Get-Item -LiteralPath (Join-Path $schemeKey 'DefaultIcon')).GetValue('')
    if ($defaultIcon -ne $expectedIconLocation) {
        throw "Unexpected protocol icon registration: $defaultIcon"
    }

    $commandKey = Join-Path $schemeKey 'shell\open\command'
    $command = (Get-Item -LiteralPath $commandKey).GetValue('')
    $expectedCommand = '"' + $installedBinary + '" "%1"'
    if ($command -ne $expectedCommand) {
        throw "Unexpected protocol launch command: $command"
    }

    Write-Output 'pass: Windows installer copied the executable and registered the current-user URI command.'
}
finally {
    if (Test-Path -LiteralPath $schemeKey) {
        Remove-Item -LiteralPath $schemeKey -Recurse -Force
    }
    if ($null -eq $previousLocalAppData) {
        Remove-Item Env:LOCALAPPDATA -ErrorAction SilentlyContinue
    }
    else {
        $env:LOCALAPPDATA = $previousLocalAppData
    }
    if ($null -eq $previousAppData) {
        Remove-Item Env:APPDATA -ErrorAction SilentlyContinue
    }
    else {
        $env:APPDATA = $previousAppData
    }
    if (Test-Path -LiteralPath $testRoot) {
        Remove-Item -LiteralPath $testRoot -Recurse -Force
    }
}
