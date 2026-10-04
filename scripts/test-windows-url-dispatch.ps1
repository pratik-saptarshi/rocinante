$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$manifest = Join-Path $repoRoot 'src-tauri\crates\rocinante-desktop-shell\Cargo.toml'
$sourceBinary = Join-Path $repoRoot 'src-tauri\target\debug\rocinante-desktop-shell.exe'
$provisioner = Join-Path $repoRoot 'scripts\provision_duckdb.py'
$targetDirectory = if ($env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR
} else {
    Join-Path $repoRoot 'src-tauri\target'
}
$installer = Join-Path $repoRoot 'src-tauri\crates\rocinante-desktop-shell\packaging\windows\install-user.ps1'
$testId = [guid]::NewGuid().ToString('N')
$testRoot = Join-Path $env:RUNNER_TEMP "rocinante-url-acceptance-$testId"
$appData = Join-Path $testRoot 'app-data'
$stateFile = Join-Path $appData 'shell-state.ron'
$stateJsonFile = Join-Path $appData 'shell-state.json'
$witness = Join-Path $testRoot 'applied-state.txt'
$forwardWitness = Join-Path $testRoot 'forward-attempt.txt'
$quitFile = Join-Path $testRoot 'request-clean-quit'
$schemeKey = 'HKCU:\Software\Classes\rocinante'
$localAppData = Join-Path $testRoot 'local-app-data'
$roamingAppData = Join-Path $testRoot 'roaming-app-data'
$installedBinary = Join-Path $localAppData 'Programs\Rocinante\rocinante-desktop-shell.exe'
$previousWitness = $env:ROCINANTE_ACCEPTANCE_WITNESS
$previousDataDir = $env:ROCINANTE_ACCEPTANCE_DATA_DIR
$previousQuitFile = $env:ROCINANTE_ACCEPTANCE_QUIT_FILE
$previousForwardWitness = $env:ROCINANTE_ACCEPTANCE_FORWARD_WITNESS
$previousLocalAppData = $env:LOCALAPPDATA
$previousAppData = $env:APPDATA
$script:warmLaunch = $null
$testPids = [System.Collections.Generic.HashSet[int]]::new()

if (Test-Path -LiteralPath $schemeKey) {
    throw "Refusing to overwrite an existing per-user rocinante protocol registration: $schemeKey"
}
if (Get-Process -Name 'rocinante-desktop-shell' -ErrorAction SilentlyContinue) {
    throw 'Close any running Rocinante desktop shell before this acceptance test.'
}

function Get-TestProcesses {
    if (-not (Test-Path -LiteralPath $installedBinary)) { return @() }
    $expected = [System.IO.Path]::GetFullPath($installedBinary)
    return @(Get-CimInstance Win32_Process -Filter "Name = 'rocinante-desktop-shell.exe'" |
        Where-Object { $_.ExecutablePath -and [System.IO.Path]::GetFullPath($_.ExecutablePath) -eq $expected })
}

function Wait-ForAppliedPath([string] $expectedPath) {
    for ($attempt = 0; $attempt -lt 60; $attempt++) {
        if (Test-Path -LiteralPath $witness) {
            $lines = Get-Content -LiteralPath $witness
            $pidLine = $lines | Where-Object { $_ -like 'pid=*' } | Select-Object -First 1
            if ($pidLine) { [void]$testPids.Add([int]$pidLine.Substring(4)) }
            if (($lines -contains 'page=Repositories') -and ($lines -contains "path=$expectedPath")) { return }
        }
        Start-Sleep -Seconds 1
    }
    $snapshot = if (Test-Path -LiteralPath $witness) { Get-Content -LiteralPath $witness -Raw } else { '<no witness>' }
    $forwardSnapshot = if (Test-Path -LiteralPath $forwardWitness) { Get-Content -LiteralPath $forwardWitness -Raw } else { '<no secondary process forwarding witness>' }
    if ($script:warmLaunch) {
        $script:warmLaunch.Refresh()
        $warmStatus = if ($script:warmLaunch.HasExited) { "pid=$($script:warmLaunch.Id) exit=$($script:warmLaunch.ExitCode)" } else { "pid=$($script:warmLaunch.Id) still-running" }
    } else {
        $warmStatus = '<not-started>'
    }
    throw "The bundled app did not apply URI target '$expectedPath'. Witness: $snapshot Forwarding: $forwardSnapshot Warm launch: $warmStatus"
}

function New-RepositoryUri([string] $path) {
    return 'rocinante://repository/open?path=' + [uri]::EscapeDataString($path)
}

try {
    New-Item -ItemType Directory -Path $testRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $appData -Force | Out-Null
    $env:LOCALAPPDATA = $localAppData
    $env:APPDATA = $roamingAppData
    $env:ROCINANTE_ACCEPTANCE_WITNESS = $witness
    $env:ROCINANTE_ACCEPTANCE_DATA_DIR = $appData
    $env:ROCINANTE_ACCEPTANCE_QUIT_FILE = $quitFile
    $env:ROCINANTE_ACCEPTANCE_FORWARD_WITNESS = $forwardWitness

    & cargo build --manifest-path $manifest --bin rocinante-desktop-shell --features acceptance-witness --locked
    if ($LASTEXITCODE -ne 0) { throw "Cargo build failed with exit code $LASTEXITCODE." }
    if (-not (Test-Path -LiteralPath $sourceBinary -PathType Leaf)) { throw "Build did not produce $sourceBinary" }
    & python $provisioner --target x86_64-pc-windows-msvc --target-dir $targetDirectory --stage-runtime-for-binary $sourceBinary
    if ($LASTEXITCODE -ne 0) { throw "Could not stage the verified DuckDB runtime beside $sourceBinary." }
    & $installer -Executable $sourceBinary
    $installedDuckdb = Join-Path (Split-Path -Parent $installedBinary) 'duckdb.dll'
    if (-not (Test-Path -LiteralPath $installedDuckdb -PathType Leaf)) {
        throw "The user installation did not include prebuilt DuckDB: $installedDuckdb"
    }

    $coldPath = Join-Path $testRoot 'cold repository'
    $warmPath = Join-Path $testRoot 'warm repository'
    New-Item -ItemType Directory -Path $coldPath, $warmPath -Force | Out-Null

    Start-Process -FilePath (New-RepositoryUri $coldPath) | Out-Null
    Wait-ForAppliedPath $coldPath
    $primaryPid = [int]((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'pid=*' }) -replace '^pid=', '')
    $primaryInstance = ((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'instance=*' }) -replace '^instance=', '')
    if (-not $primaryPid) { throw 'Cold launch did not report a process id.' }
    if (-not $primaryInstance) { throw 'Cold launch did not report an instance id.' }

    if (Test-Path -LiteralPath $forwardWitness) { Remove-Item -LiteralPath $forwardWitness -Force }
    $script:warmLaunch = Start-Process -FilePath $installedBinary -ArgumentList (New-RepositoryUri $warmPath) -PassThru
    [void]$testPids.Add([int]$script:warmLaunch.Id)
    Wait-ForAppliedPath $warmPath
    $warmPid = [int]((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'pid=*' }) -replace '^pid=', '')
    if ($warmPid -ne $primaryPid) { throw "Warm URI started or reached a different process ($warmPid; expected $primaryPid)." }

    Set-Content -LiteralPath $quitFile -Value 'quit' -NoNewline
    $primary = Get-Process -Id $primaryPid -ErrorAction SilentlyContinue
    if ($primary -and -not $primary.WaitForExit(60000)) { throw 'The app did not exit cleanly after the acceptance quit request.' }
    Remove-Item -LiteralPath $quitFile -Force -ErrorAction SilentlyContinue
    if (-not (Test-Path -LiteralPath $stateFile -PathType Leaf)) {
        throw "The app did not persist eframe state before exit. Forwarding witness: $(if (Test-Path -LiteralPath $forwardWitness) { Get-Content -LiteralPath $forwardWitness -Raw } else { '<no secondary process forwarding witness>' })"
    }
    if (-not (Test-Path -LiteralPath $stateJsonFile -PathType Leaf)) {
        throw 'The app did not persist its restart state before exit.'
    }
    Write-Output "Persisted shell state: $(Get-Content -LiteralPath $stateFile -Raw)"

    Clear-Content -LiteralPath $witness
    Start-Process -FilePath $installedBinary | Out-Null
    Wait-ForAppliedPath $warmPath
    $restartedPid = [int]((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'pid=*' }) -replace '^pid=', '')
    $restartedInstance = ((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'instance=*' }) -replace '^instance=', '')
    if (-not $restartedPid -or -not $restartedInstance -or $restartedInstance -eq $primaryInstance) { throw 'Restart did not restore state in a new process.' }

    Write-Output "Windows cold/warm URL delivery and saved-state restart passed (pid $restartedPid)."
}
finally {
    foreach ($process in (Get-TestProcesses)) {
        [void]$testPids.Add([int]$process.ProcessId)
    }
    if ($testPids.Count -gt 0) {
        Set-Content -LiteralPath $quitFile -Value 'quit' -NoNewline -ErrorAction SilentlyContinue
        foreach ($processId in $testPids) {
            $process = Get-Process -Id $processId -ErrorAction SilentlyContinue
            if ($process) { [void]$process.WaitForExit(10000) }
        }
    }
    foreach ($processId in $testPids) {
        $process = Get-CimInstance Win32_Process -Filter "ProcessId = $processId" -ErrorAction SilentlyContinue
        if ($process -and $process.ExecutablePath -and (Test-Path -LiteralPath $installedBinary) -and
            [System.IO.Path]::GetFullPath($process.ExecutablePath) -eq [System.IO.Path]::GetFullPath($installedBinary)) {
            Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
        }
    }
    if (Test-Path -LiteralPath $schemeKey) { Remove-Item -LiteralPath $schemeKey -Recurse -Force }
    $shortcutPath = Join-Path $roamingAppData 'Microsoft\Windows\Start Menu\Programs\Rocinante Repo Analyzer.lnk'
    if (Test-Path -LiteralPath $shortcutPath) { Remove-Item -LiteralPath $shortcutPath -Force }
    if ($null -eq $previousWitness) { Remove-Item Env:ROCINANTE_ACCEPTANCE_WITNESS -ErrorAction SilentlyContinue } else { $env:ROCINANTE_ACCEPTANCE_WITNESS = $previousWitness }
    if ($null -eq $previousDataDir) { Remove-Item Env:ROCINANTE_ACCEPTANCE_DATA_DIR -ErrorAction SilentlyContinue } else { $env:ROCINANTE_ACCEPTANCE_DATA_DIR = $previousDataDir }
    if ($null -eq $previousQuitFile) { Remove-Item Env:ROCINANTE_ACCEPTANCE_QUIT_FILE -ErrorAction SilentlyContinue } else { $env:ROCINANTE_ACCEPTANCE_QUIT_FILE = $previousQuitFile }
    if ($null -eq $previousForwardWitness) { Remove-Item Env:ROCINANTE_ACCEPTANCE_FORWARD_WITNESS -ErrorAction SilentlyContinue } else { $env:ROCINANTE_ACCEPTANCE_FORWARD_WITNESS = $previousForwardWitness }
    if ($null -eq $previousLocalAppData) { Remove-Item Env:LOCALAPPDATA -ErrorAction SilentlyContinue } else { $env:LOCALAPPDATA = $previousLocalAppData }
    if ($null -eq $previousAppData) { Remove-Item Env:APPDATA -ErrorAction SilentlyContinue } else { $env:APPDATA = $previousAppData }
    if (Test-Path -LiteralPath $testRoot) { Remove-Item -LiteralPath $testRoot -Recurse -Force -ErrorAction SilentlyContinue }
}
