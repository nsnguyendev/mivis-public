@echo off
:: BatchGotAdmin
:-------------------------------------
REM  --> Check for permissions
    IF "%PROCESSOR_ARCHITECTURE%" EQU "amd64" (
>nul 2>&1 "%SYSTEMROOT%\SysWOW64\cacls.exe" "%SYSTEMROOT%\SysWOW64\config\system"
) ELSE (
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"
)

REM --> If error flag set, we do not have admin.
if '%errorlevel%' NEQ '0' (
    echo Requesting administrative privileges...
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    set params = %*:"=""
    echo UAC.ShellExecute "cmd.exe", "/c ""%~s0"" %params%", "", "runas", 1 >> "%temp%\getadmin.vbs"

    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    rem Now we have admin privileges.
    rem Ensure the script operates from its own directory (where Tauri copies it).
    pushd "%CD%"
    CD /D "%~dp0"
:--------------------------------------    
    
    SET LOG_FILE=D:\_project\mivis\packages\stt\src\stt_service_run_log.txt
    echo Batch script started with admin privileges at %TIME% > %LOG_FILE%
    echo Current directory (where .bat is running): %CD% >> %LOG_FILE%

    echo --- >> %LOG_FILE%
    echo Attempting to change to Python script directory... >> %LOG_FILE%
    cd /D "D:\_project\mivis\packages\stt\src\" >> %LOG_FILE% 2>&1
    echo Changed to directory: %CD% >> %LOG_FILE%

    echo --- >> %LOG_FILE%
    echo Attempting to run Python STT service (stt_service.py) in Conda env 'whisper-cuda'... >> %LOG_FILE%
    call "D:\anaconda3\condabin\conda.bat" run -n whisper-cuda python stt_service.py >> %LOG_FILE% 2>&1
    if errorlevel 1 (
        echo FAILED to run Python STT service with conda run. Errorlevel: %errorlevel% >> %LOG_FILE%
        exit /b 1
    )
    echo Python STT service script execution finished/started via conda run. >> %LOG_FILE%

exit /b 0
