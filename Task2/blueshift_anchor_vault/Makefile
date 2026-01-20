.PHONY: help install build test deploy clean setup check

# 默认目标
help:
	@echo "========================================="
	@echo "🚀 Blueshift Anchor Vault - Makefile"
	@echo "========================================="
	@echo ""
	@echo "可用命令："
	@echo ""
	@echo "  make setup      - 自动设置项目（检查依赖、安装包、构建）"
	@echo "  make install    - 安装 Node.js 依赖"
	@echo "  make build      - 构建程序"
	@echo "  make test       - 运行测试"
	@echo "  make deploy     - 部署到本地网络"
	@echo "  make check      - 检查依赖是否已安装"
	@echo "  make clean      - 清理构建产物"
	@echo ""
	@echo "快速开始："
	@echo "  make setup      # 第一次使用"
	@echo "  make test       # 运行测试"
	@echo ""
	@echo "========================================="

# 检查依赖
check:
	@echo "📋 检查依赖..."
	@command -v rustc >/dev/null 2>&1 || { echo "❌ Rust 未安装"; exit 1; }
	@command -v solana >/dev/null 2>&1 || { echo "❌ Solana CLI 未安装"; exit 1; }
	@command -v anchor >/dev/null 2>&1 || { echo "❌ Anchor 未安装"; exit 1; }
	@command -v node >/dev/null 2>&1 || { echo "❌ Node.js 未安装"; exit 1; }
	@echo "✅ 所有依赖已安装"

# 自动设置项目
setup:
	@echo "🚀 开始自动设置..."
	@./setup.sh

# 安装 Node.js 依赖
install:
	@echo "📦 安装 Node.js 依赖..."
	@yarn install || npm install
	@echo "✅ 依赖安装完成"

# 构建程序
build:
	@echo "🏗️  构建程序..."
	@anchor build
	@echo "✅ 构建完成"
	@echo ""
	@echo "构建产物："
	@ls -lh target/deploy/*.so

# 运行测试
test:
	@echo "🧪 运行测试..."
	@anchor test
	@echo "✅ 测试完成"

# 运行测试（使用已运行的验证器）
test-skip-validator:
	@echo "🧪 运行测试（跳过启动验证器）..."
	@anchor test --skip-local-validator
	@echo "✅ 测试完成"

# 部署程序
deploy:
	@echo "📤 部署程序到本地网络..."
	@anchor deploy
	@echo "✅ 部署完成"

# 部署到不同网络（使用脚本）
deploy-interactive:
	@./deploy.sh

# 清理构建产物
clean:
	@echo "🧹 清理构建产物..."
	@anchor clean
	@rm -rf node_modules
	@echo "✅ 清理完成"

# 格式化代码
fmt:
	@echo "💅 格式化代码..."
	@cargo fmt
	@yarn lint:fix || npm run lint:fix
	@echo "✅ 格式化完成"

# 启动本地验证器
validator:
	@echo "🔧 启动本地验证器..."
	@solana-test-validator

# 查看日志
logs:
	@echo "📝 查看程序日志..."
	@solana logs

# 查看钱包信息
wallet:
	@echo "👛 钱包信息："
	@echo "地址: $$(solana address)"
	@echo "余额: $$(solana balance)"
	@echo "网络: $$(solana config get | grep 'RPC URL' | awk '{print $$3}')"

# 查看程序信息
info:
	@echo "ℹ️  程序信息："
	@echo "程序 ID: 22222222222222222222222222222222222222222222"
	@echo "构建产物: target/deploy/blueshift_anchor_vault.so"
	@echo "IDL 文件: target/idl/blueshift_anchor_vault.json"
	@echo ""
	@if [ -f target/deploy/blueshift_anchor_vault.so ]; then \
		echo "程序文件大小: $$(ls -lh target/deploy/blueshift_anchor_vault.so | awk '{print $$5}')"; \
	else \
		echo "⚠️  程序未构建，运行 'make build'"; \
	fi

# 完整的开发流程
dev: clean install build test
	@echo "✅ 开发流程完成！"

# 快速重建和测试
quick: build test-skip-validator
	@echo "✅ 快速测试完成！"
