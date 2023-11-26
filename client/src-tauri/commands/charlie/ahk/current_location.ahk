; Получаем путь к папке, где находится исполняемый файл скрипта
ScriptDir := A_ScriptDir

; Указываем поддиректорию
SubDir := "Subfolder"

; Формируем полный путь к поддиректории
FullPath := ScriptDir . "\" . SubDir

; Открываем папку в проводнике
Run, explorer %FullPath%
