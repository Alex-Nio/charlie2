; Получаем текущую рабочую директорию (папку, в которой запущен скрипт)
WorkingDir := A_WorkingDir

; Указываем поддиректорию
SubDir := "Subfolder"

; Формируем полный путь к поддиректории
FullPath := WorkingDir . "\" . SubDir

; Открываем папку в проводнике
Run, explorer %FullPath%
