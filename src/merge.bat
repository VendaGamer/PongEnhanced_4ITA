@echo off
setlocal enabledelayedexpansion

REM Output file
set "OUTPUT=merged.rs"

REM Remove existing output file
if exist "%OUTPUT%" del "%OUTPUT%"

REM Find, sort, and merge all .rs files
for /f "delims=" %%F in ('dir /s /b *.rs ^| sort') do (
    echo // --- %%F --- >> "%OUTPUT%"
    type "%%F" >> "%OUTPUT%"
    echo. >> "%OUTPUT%"
)

echo Merged all .rs files into %OUTPUT%
