#!/bin/bash

# 部署脚本 - 部署到不同的网络

set -e

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "========================================="
echo "🚀 Blueshift Anchor Vault 部署脚本"
echo "========================================="
echo ""

# 询问部署目标
echo "请选择部署目标："
echo "  1) 本地网络 (localhost)"
echo "  2) 开发网络 (devnet)"
echo "  3) 测试网络 (testnet)"
echo ""
read -p "请输入选项 (1-3): " choice

case $choice in
    1)
        NETWORK="localhost"
        CLUSTER="localnet"
        ;;
    2)
        NETWORK="https://api.devnet.solana.com"
        CLUSTER="devnet"
        ;;
    3)
        NETWORK="https://api.testnet.solana.com"
        CLUSTER="testnet"
        ;;
    *)
        echo -e "${RED}无效的选项${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "目标网络: ${YELLOW}${CLUSTER}${NC}"
echo ""

# 配置网络
echo "配置 Solana CLI..."
solana config set --url $NETWORK

# 检查余额
echo ""
echo "检查钱包余额..."
BALANCE=$(solana balance)
echo "当前余额: $BALANCE"

if [[ "$CLUSTER" != "localnet" ]]; then
    # 如果不是本地网络，检查是否需要空投
    BALANCE_NUM=$(echo $BALANCE | awk '{print $1}')
    if (( $(echo "$BALANCE_NUM < 1" | bc -l) )); then
        echo ""
        echo -e "${YELLOW}余额不足，正在申请空投...${NC}"
        solana airdrop 2
        echo -e "${GREEN}✓${NC} 空投成功"
    fi
fi

# 构建程序
echo ""
echo "========================================="
echo "🏗️  构建程序..."
echo "========================================="
echo ""
anchor build

# 部署程序
echo ""
echo "========================================="
echo "📤 部署程序..."
echo "========================================="
echo ""
anchor deploy --provider.cluster $CLUSTER

echo ""
echo "========================================="
echo "✅ 部署成功！"
echo "========================================="
echo ""
echo "程序 ID: 22222222222222222222222222222222222222222222"
echo "网络: $CLUSTER"
echo "钱包地址: $(solana address)"
echo "剩余余额: $(solana balance)"
echo ""
echo "========================================="
