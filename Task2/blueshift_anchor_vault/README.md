# Blueshift Anchor Vault

一个使用 Anchor 框架开发的 Solana 金库程序。

## ⚠️ 重要提示：构建问题

当前项目由于 Solana 工具链的 Cargo 版本限制，可能遇到 `edition2024` 构建错误。

**解决方案**：请查看 [`BUILD_STATUS.md`](./BUILD_STATUS.md) 获取详细说明和解决方案。

**快速修复**：
```bash
rustup toolchain install nightly
rustup default nightly
anchor clean && cargo clean
anchor build
```

## 🚀 快速开始

### 方法 1: 使用自动化脚本（推荐）

```bash
cd blueshift_anchor_vault
./setup.sh
```

这个脚本会自动：
- ✅ 检查所有依赖
- ✅ 安装 npm 包
- ✅ 配置 Solana 钱包
- ✅ 构建程序

### 方法 2: 手动设置

#### 1. 安装依赖

```bash
cd blueshift_anchor_vault
yarn install
# 或
npm install
```

#### 2. 构建程序

```bash
anchor build
```

#### 3. 启动本地验证器（新终端）

```bash
solana-test-validator
```

#### 4. 运行测试

```bash
# 使用已运行的验证器
anchor test --skip-local-validator

# 或自动启动验证器
anchor test
```

#### 5. 部署程序

```bash
# 部署到本地
anchor deploy

# 或使用部署脚本（支持多网络）
./deploy.sh
```

## 📁 项目结构

```
blueshift_anchor_vault/
├── Anchor.toml                 # Anchor 配置
├── Cargo.toml                  # Rust 工作空间配置
├── package.json                # Node.js 依赖
├── tsconfig.json               # TypeScript 配置
├── setup.sh                    # 自动设置脚本
├── deploy.sh                   # 部署脚本
├── programs/
│   └── blueshift_anchor_vault/
│       ├── Cargo.toml          # 程序依赖
│       ├── Xargo.toml          # Xargo 配置
│       └── src/
│           └── lib.rs          # 程序源代码 ⭐
└── tests/
    └── blueshift_anchor_vault.ts  # 测试文件 ⭐
```

## 🎯 程序功能

### 1. Deposit（存款）
将 SOL 存入个人金库。

**验证：**
- 金库必须为空（防止重复存款）
- 金额必须大于免租金最低限额

### 2. Withdraw（取款）
从个人金库取出所有 SOL。

**验证：**
- 金库必须有余额

## 🔧 常用命令

```bash
# 构建
anchor build

# 测试（会自动启动和停止验证器）
anchor test

# 测试（使用已运行的验证器）
anchor test --skip-local-validator

# 部署到本地
anchor deploy

# 部署到 devnet
anchor deploy --provider.cluster devnet

# 查看程序日志
solana logs

# 检查钱包余额
solana balance

# 查看配置
solana config get
```

## 📝 测试用例

测试文件包含 6 个测试用例：

1. ✅ 成功存款到金库
2. ✅ 重复存款应该失败
3. ✅ 小额存款应该失败
4. ✅ 成功从金库取款
5. ✅ 从空金库取款应该失败
6. ✅ 多次存取循环

运行测试：
```bash
anchor test
```

## 🔐 安全特性

- **PDA 控制**：使用程序派生地址确保安全
- **所有者验证**：只有金库所有者可以操作
- **余额检查**：防止非法操作
- **租金豁免**：确保账户存活

## ⚙️ 配置说明

### 程序 ID
程序 ID 已固定为测试要求的值：
```
22222222222222222222222222222222222222222222
```

这在以下文件中配置：
- `Anchor.toml`
- `programs/blueshift_anchor_vault/src/lib.rs`

### 网络配置

在 `Anchor.toml` 中可以配置不同网络：
- `localnet`: 本地测试网络
- `devnet`: 开发网络
- `testnet`: 测试网络

## 🐛 故障排查

### 问题：找不到钱包
```bash
# 创建新钱包
solana-keygen new
```

### 问题：余额不足
```bash
# 本地网络自动有余额
# devnet/testnet 申请空投
solana airdrop 2
```

### 问题：程序构建失败
```bash
# 清理并重新构建
anchor clean
anchor build
```

### 问题：测试超时
```bash
# 确保本地验证器正在运行
solana-test-validator

# 在另一个终端运行测试
anchor test --skip-local-validator
```

### 问题：端口被占用
```bash
# 查找并终止占用端口的进程
lsof -ti:8899 | xargs kill -9
```

## 📚 学习资源

- [Anchor 官方文档](https://www.anchor-lang.com/)
- [Solana 官方文档](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)

## 💻 开发建议

### VS Code 扩展推荐
- rust-analyzer
- Better TOML
- Solana IDE

### 代码检查
```bash
# Rust 格式化
cargo fmt

# Rust linting
cargo clippy

# TypeScript 格式化
yarn lint:fix
```

## 📄 许可证

MIT License

---

**作者**：Solana 2026 Task  
**日期**：2026-01  
**版本**：0.1.0
