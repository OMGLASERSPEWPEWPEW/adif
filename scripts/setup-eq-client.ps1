<#
.SYNOPSIS
    Sets up an EverQuest Titanium client directory for use with EQ emulators.

.DESCRIPTION
    Downloads a pre-installed EQ Titanium client from Internet Archive and
    extracts it to reference/eq-client/. The ISOs in reference/Everquest Titanium/
    use InstallShield cabinets that standard extraction tools can't handle, so
    this script uses a community-maintained pre-installed archive instead.

.EXAMPLE
    .\setup-eq-client.ps1
    .\setup-eq-client.ps1 -Force
    .\setup-eq-client.ps1 -ClientZip "C:\path\to\local\client.zip"
#>
[CmdletBinding()]
param(
    [string]$OutputDir  = (Join-Path $PSScriptRoot "..\reference\eq-client"),
    [string]$ClientZip  = "",
    [switch]$KeepZip,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

$ArchiveUrl = "https://archive.org/download/EQP99V46/EQ%20P99%20v46.zip"

# --- Helpers ---

function Write-Step {
    param([string]$Tag, [string]$Message, [string]$Color = "Cyan")
    Write-Host "[$Tag] " -ForegroundColor $Color -NoNewline
    Write-Host $Message
}

function Write-Ok {
    param([string]$Tag, [string]$Message)
    Write-Step $Tag $Message "Green"
}

function Write-Warn {
    param([string]$Tag, [string]$Message)
    Write-Step $Tag $Message "Yellow"
}

function Write-Fail {
    param([string]$Tag, [string]$Message)
    Write-Step $Tag $Message "Red"
}

function Format-Size {
    param([long]$Bytes)
    if ($Bytes -ge 1GB) { return "{0:N2} GB" -f ($Bytes / 1GB) }
    if ($Bytes -ge 1MB) { return "{0:N1} MB" -f ($Bytes / 1MB) }
    return "{0:N0} KB" -f ($Bytes / 1KB)
}

# --- Banner ---

Write-Host ""
Write-Host "=====================================" -ForegroundColor White
Write-Host "  EQ Titanium Client Setup" -ForegroundColor White
Write-Host "=====================================" -ForegroundColor White
Write-Host ""

# --- Resolve paths ---

$OutputDir = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutputDir)
Write-Step "prereq" "Output: $OutputDir"

# --- Idempotency check ---

if ((Test-Path $OutputDir) -and -not $Force) {
    $existing = Get-ChildItem $OutputDir -Recurse -File
    $totalSize = ($existing | Measure-Object -Property Length -Sum).Sum
    Write-Ok "skip" "eq-client/ already exists ($($existing.Count) files, $(Format-Size $totalSize))."
    Write-Host "       Run with -Force to rebuild from scratch." -ForegroundColor DarkGray
    exit 0
}

# --- Disk space check ---

$drive = (Split-Path $OutputDir -Qualifier)
$freeSpace = (Get-PSDrive ($drive -replace ':','')).Free
$requiredSpace = 3GB
if ($freeSpace -lt $requiredSpace) {
    Write-Fail "prereq" "Need ~3 GB free on $drive but only $(Format-Size $freeSpace) available."
    exit 1
}
Write-Step "prereq" "Disk space: $(Format-Size $freeSpace) free on $drive"
Write-Host ""

# --- Get the client zip ---

$downloaded = $false

if ($ClientZip -and (Test-Path $ClientZip)) {
    Write-Ok "source" "Using local client zip: $ClientZip"
}
else {
    $defaultZipPath = Join-Path (Split-Path $OutputDir -Parent) "EQ_P99_v46.zip"

    if (Test-Path $defaultZipPath) {
        Write-Ok "source" "Found existing download: $defaultZipPath"
        $ClientZip = $defaultZipPath
    }
    else {
        Write-Step "download" "Downloading pre-installed EQ Titanium client (1.3 GB)..."
        Write-Step "download" "From: $ArchiveUrl"
        $ClientZip = $defaultZipPath

        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        $wc = New-Object Net.WebClient
        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        try {
            $wc.DownloadFile($ArchiveUrl, $ClientZip)
        }
        catch {
            Write-Fail "download" "Download failed: $_"
            Write-Host ""
            Write-Host "  You can download manually from:" -ForegroundColor Yellow
            Write-Host "    https://archive.org/details/EQP99V46" -ForegroundColor White
            Write-Host ""
            Write-Host "  Then run:" -ForegroundColor DarkGray
            Write-Host "    .\setup-eq-client.ps1 -ClientZip `"path\to\EQ P99 v46.zip`"" -ForegroundColor DarkGray
            Write-Host ""
            exit 1
        }
        finally {
            $wc.Dispose()
        }
        $sw.Stop()

        $dlSize = (Get-Item $ClientZip).Length
        Write-Ok "download" "Downloaded $(Format-Size $dlSize) in $([math]::Round($sw.Elapsed.TotalSeconds))s"
        $downloaded = $true
    }
}

# --- Extract ---

Write-Host ""

if ((Test-Path $OutputDir) -and $Force) {
    Write-Warn "extract" "Removing existing output directory..."
    Remove-Item $OutputDir -Recurse -Force -Confirm:$false
}

$tempExtract = Join-Path (Split-Path $OutputDir -Parent) "eq-client-extract-temp"
if (Test-Path $tempExtract) {
    Remove-Item $tempExtract -Recurse -Force -Confirm:$false
}

Write-Step "extract" "Extracting client files..."

Add-Type -AssemblyName System.IO.Compression.FileSystem
try {
    [System.IO.Compression.ZipFile]::ExtractToDirectory($ClientZip, $tempExtract)
}
catch {
    Write-Fail "extract" "Extraction failed: $_"
    if (Test-Path $tempExtract) {
        Remove-Item $tempExtract -Recurse -Force -Confirm:$false
    }
    exit 1
}

$extractedDirs = Get-ChildItem $tempExtract -Directory
if ($extractedDirs.Count -eq 1) {
    Move-Item $extractedDirs[0].FullName $OutputDir
    Remove-Item $tempExtract -Recurse -Force -Confirm:$false
    Write-Ok "extract" "Extracted $($extractedDirs[0].Name)/ -> eq-client/"
}
else {
    Move-Item $tempExtract $OutputDir
    Write-Ok "extract" "Extracted to eq-client/"
}

# --- Verify ---

Write-Host ""

$keyFiles = @(
    'eqgame.exe',
    'eqmain.dll',
    'spells_us.txt',
    'dbstr_us.txt',
    'eqstr_us.txt',
    'defaults.ini'
)

$verified = 0
foreach ($kf in $keyFiles) {
    $found = Get-ChildItem $OutputDir -Recurse -Filter $kf -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($found) {
        Write-Ok "verify" "$kf"
        $verified++
    }
    else {
        Write-Warn "verify" "$kf -- not found"
    }
}

# --- Cleanup ---

if (-not $KeepZip -and $downloaded -and (Test-Path $ClientZip)) {
    Write-Step "cleanup" "Removing downloaded zip..."
    Remove-Item $ClientZip -Force -Confirm:$false
}
elseif ($downloaded -and $KeepZip) {
    Write-Step "cleanup" "Keeping downloaded zip at: $ClientZip"
}

Write-Host ""

$outputFiles = Get-ChildItem $OutputDir -Recurse -File
$outputSize = ($outputFiles | Measure-Object -Property Length -Sum).Sum

Write-Host "=====================================" -ForegroundColor White
Write-Host "  Complete!" -ForegroundColor Green
Write-Host "  $($outputFiles.Count) files, $(Format-Size $outputSize)" -ForegroundColor White
Write-Host "  $verified/$($keyFiles.Count) key files verified" -ForegroundColor $(if ($verified -eq $keyFiles.Count) { "Green" } else { "Yellow" })
Write-Host "  Output: $OutputDir" -ForegroundColor White
Write-Host "=====================================" -ForegroundColor White
Write-Host ""
