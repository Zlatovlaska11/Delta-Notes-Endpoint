@echo off
set /a pocitadlo=%1
:start
echo %pocitadlo%
set /a pocitadlo = %pocitadlo%-1
if %pocitadlo%==-1 (pause) else (goto :start)