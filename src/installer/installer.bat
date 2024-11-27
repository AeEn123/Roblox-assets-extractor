timeout /t 1
move /y %1 %2
rd /s /q %1
for %%I in (%2) do cd %%~dpI
start %2
