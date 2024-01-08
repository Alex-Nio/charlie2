import subprocess
import sys
import os

def create_virtualenv():
    # Создание виртуального окружения в папке src-tauri/venv
    subprocess.run([sys.executable, '-m', 'venv', os.path.join('src', 'venv')])

def activate_virtualenv():
    # Активация виртуального окружения
    if sys.platform.startswith('win'):
        activate_script = os.path.join('src', 'venv', 'Scripts', 'activate')
    else:
        activate_script = os.path.join('src', 'venv', 'bin', 'activate')
    activate_command = f'call {activate_script}' if sys.platform.startswith('win') else f'source {activate_script}'
    subprocess.run(activate_command, shell=True)

def install_requirements():
    # Установка библиотек из requirements.txt
    subprocess.run([sys.executable, '-m', 'pip', 'install', '-r', os.path.join('src', 'tts', 'requirements.txt')])

def main():
    create_virtualenv()
    activate_virtualenv()
    install_requirements()

if __name__ == "__main__":
    main()
