# 自动安装 Rust 工具链（仅开发/打包时需要，玩家安装 exe 后无需 Rust）
$ErrorActionPreference = "Stop"

$cargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"

if (Test-Path $cargo) {
    Write-Host "Rust 已安装: $cargo" -ForegroundColor Green
    & $cargo --version
    & (Join-Path $env:USERPROFILE ".cargo\bin\rustc.exe") --version
    exit 0
}

Write-Host "正在下载 Rust 安装器..." -ForegroundColor Cyan
$rustup = Join-Path $env:TEMP "rustup-init.exe"
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustup -UseBasicParsing

Write-Host "正在安装 Rust（首次约 3-5 分钟）..." -ForegroundColor Cyan
& $rustup -y --default-toolchain stable

if (-not (Test-Path $cargo)) {
    Write-Error "Rust 安装失败，请手动访问 https://rustup.rs 安装"
    exit 1
}

# 当前会话加入 PATH
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

Write-Host "Rust 安装完成！" -ForegroundColor Green
& $cargo --version
Write-Host ""
Write-Host "说明：Rust 仅用于开发打包。玩家安装的 exe 已包含全部运行时，无需 Rust。" -ForegroundColor Yellow
