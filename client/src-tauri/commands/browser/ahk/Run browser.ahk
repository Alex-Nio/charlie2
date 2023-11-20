#NoEnv
; #Warn
SendMode Input
SetWorkingDir %A_ScriptDir%

RegRead, BrowserKeyName, HKEY_CURRENT_USER, Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.html\UserChoice, Progid
RegRead, BrowserFullCommand, HKEY_CLASSES_ROOT, %BrowserKeyName%\shell\open\command

Run, opera.exe "https://yandex.ru/" " --new-window "
