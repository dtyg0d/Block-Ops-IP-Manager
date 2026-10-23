@echo off
setlocal
title BlockOps IP Report Relay

set "RELAY_EXE=%~dp0BlockOps_IP_Report_Relay.exe"
if not exist "%RELAY_EXE%" set "RELAY_EXE=%~dp0target\release\blockops_ip_report_relay.exe"

if not exist "%RELAY_EXE%" (
    echo BlockOps_IP_Report_Relay.exe was not found.
    echo Run build_release.bat first.
    pause
    exit /b 1
)

echo This must run on a Windows PC connected to the rack LAN and the VPN.
echo.
set /p "MANAGER_IP=Manager laptop VPN IP: "
if "%MANAGER_IP%"=="" (
    echo A manager VPN IP is required.
    pause
    exit /b 1
)

echo.
echo Starting relay to %MANAGER_IP% on UDP 14235...
echo Allow private-network access if Windows Firewall asks.
echo.
"%RELAY_EXE%" "%MANAGER_IP%"

echo.
echo Relay stopped.
pause
