timeout /t 1
copy /y %1 %2
start %3
for %%I in (%1) do (
    rd /s /q "%%~dpI"
)
