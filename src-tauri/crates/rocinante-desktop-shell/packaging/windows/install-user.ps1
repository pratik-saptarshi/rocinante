[CmdletBinding()]
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string] $Executable
)

$ErrorActionPreference = 'Stop'

if (-not $env:LOCALAPPDATA) {
    throw 'LOCALAPPDATA must be set for a per-user installation.'
}
if (-not $env:APPDATA) {
    throw 'APPDATA must be set for a per-user Start Menu shortcut.'
}
$source = (Resolve-Path -LiteralPath $Executable).Path
if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
    throw "Desktop shell executable was not found: $Executable"
}
$sourceDirectory = Split-Path -Parent $source
$duckdbLibrary = Join-Path $sourceDirectory 'deps\duckdb.dll'
if (-not (Test-Path -LiteralPath $duckdbLibrary -PathType Leaf)) {
    throw "Verified prebuilt DuckDB runtime was not found: $duckdbLibrary"
}

$installDirectory = Join-Path $env:LOCALAPPDATA 'Programs\Rocinante'
$installedBinary = Join-Path $installDirectory 'rocinante-desktop-shell.exe'
$iconSource = Join-Path $PSScriptRoot '..\icons\Rocinante.ico'
$iconSource = (Resolve-Path -LiteralPath $iconSource).Path
$installedIcon = Join-Path $installDirectory 'Rocinante.ico'
New-Item -ItemType Directory -Path $installDirectory -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $installedBinary -Force
Copy-Item -LiteralPath $duckdbLibrary -Destination (Join-Path $installDirectory 'duckdb.dll') -Force
Copy-Item -LiteralPath $iconSource -Destination $installedIcon -Force

$programsDirectory = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'
$shortcutPath = Join-Path $programsDirectory 'Rocinante Repo Analyzer.lnk'
New-Item -ItemType Directory -Path $programsDirectory -Force | Out-Null
$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = $installedBinary
$shortcut.WorkingDirectory = $installDirectory
$iconLocation = '"' + $installedIcon + '",0'
$shortcut.IconLocation = $iconLocation
$shortcut.Save()

$schemeKey = 'HKCU:\Software\Classes\rocinante'
New-Item -Path $schemeKey -Force | Out-Null
Set-Item -Path $schemeKey -Value 'URL:Rocinante Repository' -Force
New-ItemProperty -Path $schemeKey -Name 'URL Protocol' -PropertyType String -Value '' -Force | Out-Null
$iconKey = Join-Path $schemeKey 'DefaultIcon'
New-Item -Path $iconKey -Force | Out-Null
Set-Item -Path $iconKey -Value $iconLocation -Force

$commandKey = Join-Path $schemeKey 'shell\open\command'
New-Item -Path $commandKey -Force | Out-Null
$command = '"' + $installedBinary + '" "%1"'
Set-Item -Path $commandKey -Value $command -Force

Write-Output "Installed Rocinante desktop shell for the current user at $installedBinary."
Write-Output 'Registered the rocinante:// URL handler in the current user registry.'
