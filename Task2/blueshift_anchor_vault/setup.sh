#!/bin/bash

# Blueshift Anchor Vault - 快速启动脚本
# 这个脚本会帮助你快速设置和运行项目

set -e

echo "========================================="
echo "🚀 Blueshift Anchor Vault 快速启动"
echo "========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 检查依赖
echo "📋 步骤 1: 检查依赖..."
echo ""

# 检查 Rust
if command -v rustc &> /dev/null; then
    echo -e "${GREEN}✓${NC} Rust 已安装: $(rustc --version)"
else
    echo -e "${RED}✗${NC} Rust 未安装"
    echo "请运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 检查 Solana
if command -v solana &> /dev/null; then
    echo -e "${GREEN}✓${NC} Solana CLI 已安装: $(solana --version)"
else
    echo -e "${RED}✗${NC} Solana CLI 未安装"
    echo "请运行: sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
    exit 1
fi

# 检查 Anchor
if command -v anchor &> /dev/null; then
    echo -e "${GREEN}✓${NC} Anchor 已安装: $(anchor --version)"
else
    echo -e "${RED}✗${NC} Anchor 未安装"
    echo "请运行: cargo install --git https://github.com/coral-xyz/anchor avm --locked --force"
    exit 1
fi

# 检查 Node.js
if command -v node &> /dev/null; then
    echo -e "${GREEN}✓${NC} Node.js 已安装: $(node --version)"
else
    echo -e "${RED}✗${NC} Node.js 未安装"
    echo "请访问: https://nodejs.org/"
    exit 1
fi

echo ""
echo "========================================="
echo "📦 步骤 2: 安装依赖..."
echo "========================================="
echo ""

# 安装 Node.js 依赖
if [ ! -d "node_modules" ]; then
    echo "安装 npm 包..."
    yarn install || npm install
else
    echo -e "${GREEN}✓${NC} npm 依赖已安装"
fi

echo ""
echo "========================================="
echo "🔑 步骤 3: 配置 Solana 钱包..."
echo "========================================="
echo ""

# 检查钱包
if [ ! -f ~/.config/solana/id.json ]; then
    echo -e "${YELLOW}警告:${NC} 未找到 Solana 钱包"
    echo "创建新钱包..."
    solana-keygen new --no-bip39-passphrase
else
    echo -e "${GREEN}✓${NC} Solana 钱包已存在"
fi

# 显示钱包地址
echo "钱包地址: $(solana address)"

echo ""
echo "========================================="
echo "🔧 步骤 4: 配置 Solana CLI..."
echo "========================================="
echo ""

# 配置本地网络
echo "配置为本地网络..."
solana config set --url localhost
echo -e "${GREEN}✓${NC} 已设置为 localhost"

echo ""
echo "========================================="
echo "🏗️  步骤 5: 构建程序..."
echo "========================================="
echo ""

anchor build

echo ""
echo -e "${GREEN}✓${NC} 构建成功！"

echo ""
echo "========================================="
echo "✅ 设置完成！"
echo "========================================="
echo ""
echo "接下来的步骤："
echo ""
echo "1. 在新终端启动本地验证器："
echo "   ${YELLOW}solana-test-validator${NC}"
echo ""
echo "2. 回到这个终端，运行测试："
echo "   ${YELLOW}anchor test --skip-local-validator${NC}"
echo ""
echo "   或者一键测试（会自动启动验证器）："
echo "   ${YELLOW}anchor test${NC}"
echo ""
echo "3. 部署到本地网络："
echo "   ${YELLOW}anchor deploy${NC}"
echo ""
echo "程序信息："
echo "  程序 ID: 22222222222222222222222222222222222222222222"
echo "  构建输出: target/deploy/blueshift_anchor_vault.so"
echo ""
echo "========================================="
