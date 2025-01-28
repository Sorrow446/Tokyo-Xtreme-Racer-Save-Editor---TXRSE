@echo off
setlocal enabledelayedexpansion

if "%~1" == "" (
    echo Drag and drop a save file onto this batch file.
    pause >nul
    exit
)

cd "%~dp0"

echo Input nothing to retain any values.
set /p cp=Input CP value: 
set /p bp=Input BP value: 

echo Calling txrse...

set "cmd="%~dp0txrse.exe""
if defined cp set "cmd=!cmd! --cp !cp!"
if defined bp set "cmd=!cmd! --bp !bp!"
set "cmd=!cmd! --in-path "!%~1!""

%cmd%

if %errorlevel% neq 0 (
    echo Not OK. Press enter to exit.
    pause >nul
    exit
)

echo OK. Press enter to exit.
pause >nul