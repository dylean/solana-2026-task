.PHONY: help install build test deploy clean setup check

help:
	@echo "========================================="
	@echo "🚀 Blueshift Anchor Escrow - Makefile"
	@echo "========================================="
	@echo ""
	@echo "可用命令："
	@echo ""
	@echo "  make setup      - 自动设置项目"
	@echo "  make install    - 安装依赖"
	@echo "  make build      - 构建程序"
	@echo "  make test       - 运行测试"
	@echo "  make deploy     - 部署程序"
	@echo "  make clean      - 清理构建产物"
	@echo "  make check      - 检查依赖"
	@echo ""
	@echo "========================================="

check:
	@echo "📋 检查依赖..."
	@command -v rustc >/dev/null 2>&1 || { echo "❌ Rust 未安装"; exit 1; }
	@command -v solana >/dev/null 2>&1 || { echo "❌ Solana CLI 未安装"; exit 1; }
	@command -v anchor >/dev/null 2>&1 || { echo "❌ Anchor 未安装"; exit 1; }
	@command -v node >/dev/null 2>&1 || { echo "❌ Node.js 未安装"; exit 1; }
	@echo "✅ 所有依赖已安装"

setup:
	@./setup.sh

install:
	@echo "📦 安装依赖..."
	@yarn install || npm install
	@echo "✅ 依赖安装完成"

build:
	@echo "🏗️  构建程序..."
	@anchor build
	@echo "✅ 构建完成"

test:
	@echo "🧪 运行测试..."
	@anchor test
	@echo "✅ 测试完成"

test-skip-validator:
	@echo "🧪 运行测试（跳过验证器）..."
	@anchor test --skip-local-validator
	@echo "✅ 测试完成"

deploy:
	@echo "📤 部署程序..."
	@anchor deploy
	@echo "✅ 部署完成"

clean:
	@echo "🧹 清理构建产物..."
	@anchor clean
	@rm -rf node_modules
	@echo "✅ 清理完成"

validator:
	@echo "🔧 启动本地验证器..."
	@solana-test-validator

logs:
	@echo "📝 查看程序日志..."
	@solana logs

wallet:
	@echo "👛 钱包信息："
	@echo "地址: $$(solana address)"
	@echo "余额: $$(solana balance)"

info:
	@echo "ℹ️  程序信息："
	@echo "程序 ID: 22222222222222222222222222222222222222222222"
	@echo "构建产物: target/deploy/blueshift_anchor_escrow.so"
	@if [ -f target/deploy/blueshift_anchor_escrow.so ]; then \
		echo "程序文件大小: $$(ls -lh target/deploy/blueshift_anchor_escrow.so | awk '{print $$5}')"; \
	fi
