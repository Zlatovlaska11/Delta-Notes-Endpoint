@echo off
set /a pocitadlo=0
set /a pocet=%1
:start
echo Ahoj
set /a pocitadlo=%pocitadlo%+1
if %pocitadlo%==%pocet% (pause) else (goto :start)