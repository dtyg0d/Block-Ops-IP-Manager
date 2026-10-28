@echo off
setlocal
title Build BlockOps Static IP Manager
set "VERSION=3.0.1"
set "DIST=dist"
set "MANAGER=%DIST%\BlockOps_Static_IP_Manager.exe"
set "MANAGER_VERSIONED=%DIST%\BlockOps_Static_IP_Manager_v%VERSION%.exe"
set "RELAY=%DIST%\BlockOps_IP_Report_Relay.exe"
set "ARCHIVE=%DIST%\BlockOps_Static_IP_Manager_v%VERSION%_Windows.zip"

where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust/Cargo was not found. Install Rust from https://rustup.rs/ and try again.
    exit /b 1
)

where tar >nul 2>nul
if errorlevel 1 (
    echo Windows tar.exe was not found, so the release ZIP cannot be created.
    exit /b 1
)

echo Building BlockOps Static IP Manager %VERSION%...
cargo test --all-targets
if errorlevel 1 exit /b 1

cargo build --release --bins
if errorlevel 1 exit /b 1

if not exist "%DIST%" mkdir "%DIST%"
if errorlevel 1 exit /b 1
copy /Y "target\release\blockops_static_ip_tool.exe" "%MANAGER%" >nul
if errorlevel 1 exit /b 1
copy /Y "target\release\blockops_static_ip_tool.exe" "%MANAGER_VERSIONED%" >nul
if errorlevel 1 exit /b 1
copy /Y "target\release\blockops_ip_report_relay.exe" "%RELAY%" >nul
if errorlevel 1 exit /b 1
copy /Y "start_ip_report_relay.bat" "%DIST%\start_ip_report_relay.bat" >nul
if errorlevel 1 exit /b 1
copy /Y "README_V2.md" "%DIST%\README_V2.md" >nul
if errorlevel 1 exit /b 1
copy /Y "README_IP_REPORT_RELAY.md" "%DIST%\README_IP_REPORT_RELAY.md" >nul
if errorlevel 1 exit /b 1

if exist "%ARCHIVE%" del /Q "%ARCHIVE%"
if errorlevel 1 exit /b 1
tar -a -c -f "%ARCHIVE%" -C "%DIST%" "BlockOps_Static_IP_Manager_v%VERSION%.exe" "BlockOps_IP_Report_Relay.exe" "start_ip_report_relay.bat" "README_V2.md" "README_IP_REPORT_RELAY.md"
if errorlevel 1 exit /b 1

echo.
echo Build complete:
echo   %MANAGER_VERSIONED%
echo   %ARCHIVE%
if /I not "%~1"=="--no-pause" pause
