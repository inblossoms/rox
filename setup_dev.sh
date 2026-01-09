#!/bin/bash


set -e # 遇到错误立即退出

# --- 颜色定义 ---
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

info() { echo -e "${BLUE}ℹ️  [INFO]${NC} $1"; }
success() { echo -e "${GREEN}✅ [SUCCESS]${NC} $1"; }
warn() { echo -e "${YELLOW}⚠️  [WARN]${NC} $1"; }
error() { echo -e "${RED}❌ [ERROR]${NC} $1"; }

# --- 0. 基础检查 ---
echo "正在检查基础环境..."
if ! command -v cargo &> /dev/null; then
    error "未检测到 Rust 工具链。请先安装: https://rustup.rs/"
    exit 1
fi

# --- 1. 安装 Rust 官方组件 ---
info "正在安装 rustfmt 和 clippy..."
rustup component add rustfmt clippy
success "Rust 官方组件已就绪。"

# --- 2. 智能安装 cargo-binstall ---
# 逻辑：如果没有 binstall，询问用户是否安装。如果用户拒绝，后续将使用编译安装。
HAS_BINSTALL=false

if command -v cargo-binstall &> /dev/null; then
    HAS_BINSTALL=true
    success "检测到 cargo-binstall，将用于加速安装。"
else
    echo ""
    warn "未检测到 cargo-binstall。"
    echo "cargo-binstall 可以直接下载预编译的二进制文件，将安装速度从'分钟级'提升到'秒级'。"
    read -p "❓ 是否允许安装 cargo-binstall? (y/N): " choice
    case "$choice" in
        y|Y )
            info "正在安装 cargo-binstall..."
            # 使用官方脚本安装
            curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
            HAS_BINSTALL=true
            success "cargo-binstall 安装成功！"
            ;;
        * )
            warn "已跳过 cargo-binstall。后续工具将通过源码编译安装（速度较慢）。"
            ;;
    esac
fi

# --- 3. 安装工具函数 ---
install_tool() {
    local cmd_name=$1
    local pkg_name=$2

    if command -v $cmd_name &> /dev/null; then
        success "$cmd_name 已安装。"
        return
    fi

    info "$cmd_name 未找到，正在安装..."

    if [ "$HAS_BINSTALL" = true ]; then
        cargo binstall -y --locked $pkg_name || cargo install --locked $pkg_name
    else
        cargo install --locked $pkg_name
    fi

    success "$pkg_name 安装完成。"
}

# --- 4. 安装依赖工具 ---
install_tool "cargo-deny" "cargo-deny"
install_tool "cargo-nextest" "cargo-nextest"
install_tool "typos" "typos-cli"

# --- 5. 配置 Pre-commit ---
info "配置 Git Hooks..."

if ! command -v pre-commit &> /dev/null; then
    warn "未检测到 pre-commit。"
    if [[ "$OSTYPE" == "darwin"* ]] && command -v brew &> /dev/null; then
        info "正在使用 Homebrew 安装..."
        brew install pre-commit
    elif command -v pip3 &> /dev/null; then
        info "正在使用 pip3 安装..."
        pip3 install pre-commit
    else
        error "无法自动安装 pre-commit。请手动安装: https://pre-commit.com/"
        exit 1
    fi
fi

pre-commit install
pre-commit install --hook-type commit-msg
success "Git hooks 配置完成！"

echo ""
echo -e "${GREEN}🎉 开发环境初始化完成！${NC}"
