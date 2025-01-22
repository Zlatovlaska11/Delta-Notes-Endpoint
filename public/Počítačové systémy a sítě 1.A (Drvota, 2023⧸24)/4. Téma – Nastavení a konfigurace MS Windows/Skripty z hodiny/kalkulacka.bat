@echo off
set /a operand1=%1
set /a operand2=%2
set /a operator=%3
set /a vysledek=0
if %operator%==1 (set /a vysledek = %operand1%+%operand2%)
if %operator%==2 (set /a vysledek = %operand1%-%operand2%)
echo %vysledek%