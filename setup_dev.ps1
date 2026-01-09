$ErrorActionPreference = "Stop"

function Write-Info($msg) { Write-Host "ℹ️  [INFO] $msg" -ForegroundColor Cyan }
function Write-Success($msg) { Write-Host "✅ [SUCCESS] $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "⚠️  [WARN] $msg" -ForegroundColor Yellow }
function Write-ErrorMsg($msg) { Write-Host "❌ [ERROR] $msg" -ForegroundColor Red }

# --- 0. 基础检查 ---
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-ErrorMsg "未检测到 Rust 工具链。请访问 https://rustup.rs/ 安装。"
    exit 1
}

# --- 1. 安装 Rust 官方组件 ---
Write-Info "正在安装 rustfmt 和 clippy..."
rustup component add rustfmt clippy
Write-Success "Rust 官方组件已就绪。"

# --- 2. 智能安装 cargo-binstall ---
$UseBinstall = $false

if (Get-Command cargo-binstall -ErrorAction SilentlyContinue) {
    $UseBinstall = $true
    Write-Success "检测到 cargo-binstall，将用于加速安装。"
} else {
    Write-Host ""
    Write-Warn "未检测到 cargo-binstall。"
    Write-Host "cargo-binstall 可以直接下载预编译的二进制文件，极大提升安装速度。"
    $confirmation = Read-Host "❓ 是否允许安装 cargo-binstall? (y/N)"
    if ($confirmation -match "^[yY]") {
        Write-Info "正在安装 cargo-binstall..."
        # PowerShell 安装命令
        Set-ExecutionPolicy Unrestricted -Scope Process -Force
        iex (new-object net.webclient).downloadstring('https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1')
        $UseBinstall = $true
        Write-Success "cargo-binstall 安装成功！"
    } else {
        Write-Warn "已跳过。后续工具将通过源码编译安装（速度较慢）。"
    }
}

# --- 3. 安装工具函数 ---
function Install-Tool {
    param ($CmdName, $PkgName)

    if (Get-Command $CmdName -ErrorAction SilentlyContinue) {
        Write-Success "$CmdName 已安装。"
        return
    }

    Write-Info "$CmdName 未找到，正在安装..."

    if ($UseBinstall) {
        cargo binstall -y --locked $PkgName
        if ($LASTEXITCODE -ne 0) {
            Write-Warn "Binstall 失败，尝试源码编译..."
            cargo install --locked $PkgName
        }
    } else {
        cargo install --locked $PkgName
    }

    Write-Success "$PkgName 安装完成。"
}

# --- 4. 安装依赖工具 ---
Install-Tool "cargo-deny" "cargo-deny"
Install-Tool "cargo-nextest" "cargo-nextest"
Install-Tool "typos" "typos-cli"

# --- 5. 配置 Pre-commit ---
Write-Info "配置 Git Hooks..."

if (-not (Get-Command pre-commit -ErrorAction SilentlyContinue)) {
    Write-Warn "未检测到 pre-commit。"
    if (Get-Command pip -ErrorAction SilentlyContinue) {
        Write-Info "正在使用 pip 安装..."
        pip install pre-commit
    } else {
        Write-ErrorMsg "无法自动安装 pre-commit。请手动安装 Python 和 pip，然后运行 'pip install pre-commit'。"
        exit 1
    }
}

pre-commit install
pre-commit install --hook-type commit-msg
Write-Success "Git hooks 配置完成！"

Write-Host ""
Write-Host "🎉 开发环境初始化完成！" -ForegroundColor Green
