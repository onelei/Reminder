@echo off
chcp 65001 >nul
setlocal

echo ========================================
echo   Reminder - 一键打包（含环境检查）
echo ========================================
echo.

cd /d "%~dp0.."

:: 检查 Node.js
where node >nul 2>&1
if errorlevel 1 (
    echo [错误] 未找到 Node.js，请先安装 https://nodejs.org/
    exit /b 1
)

:: 自动安装 Rust（若未安装）
powershell -ExecutionPolicy Bypass -File "%~dp0setup-rust.ps1"
if errorlevel 1 exit /b 1

:: 加入 PATH
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

:: 安装依赖
if not exist "node_modules\" (
    echo 正在安装 npm 依赖...
    call npm install
    if errorlevel 1 exit /b 1
)

:: 打包 exe 安装程序
echo.
echo 正在打包，首次编译约 10-20 分钟，请耐心等待...
call npm run tauri build
if errorlevel 1 (
    echo [错误] 打包失败
    exit /b 1
)

echo.
echo ========================================
echo   打包完成！
echo   安装包位置:
echo   src-tauri\target\release\bundle\nsis\
echo.
echo   玩家双击安装即可使用，无需安装 Rust。
echo ========================================

endlocal
