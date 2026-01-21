# Solana 2026 开发任务集

> Solana 智能合约开发实战项目集合

## 📋 项目概述

本项目包含 6 个 Solana 智能合约开发任务，涵盖从基础的 SPL Token 操作到高级的 Anchor 和 Pinocchio 框架程序开发。

## 🎯 任务列表

| 任务 | 名称 | 框架 | 状态 | 说明 |
|------|------|------|------|------|
| Task1 | SPL Token 铸造 | TypeScript | ✅ 完成 | 使用 Web3.js 铸造 SPL 代币 |
| Task2 | Anchor Vault | Anchor | ✅ 完成 | SOL 金库存取程序 |
| Task3 | Anchor Escrow | Anchor | ✅ 完成 | 代币托管交换程序（307KB）|
| Task4 | Pinocchio Vault | Pinocchio | ✅ 完成 | SOL 金库（底层实现，13KB）|
| Task5 | Pinocchio Escrow | Pinocchio | ✅ 完成 | 代币托管（底层实现，22KB）|
| Task6 | Pinocchio AMM | Pinocchio | ✅ 完成 | 自动做市商 AMM（17KB）|

## 📁 项目结构

```
solana-2026-task/
├── README.md                         # 项目总览
├── DOCKER.md                         # Docker 使用说明
├── .gitignore                        # Git 忽略配置
│
├── Task1/                            # Task1 项目（TypeScript）
│   ├── Task1.md                      # 需求文档
│   └── mint-spl-token.ts             # SPL Token 铸造代码
│
├── Task2/                            # Task2 Anchor Vault 项目
│   ├── Task2.md                      # 需求文档
│   └── blueshift_anchor_vault/       # Anchor 项目
│       ├── programs/                 # 程序代码
│       ├── tests/                    # 测试代码
│       ├── target/deploy/            # 编译产物 (.so)
│       └── README.md                 # 项目文档
│
├── Task3/                            # Task3 Anchor Escrow 项目
│   ├── Task3.md                      # 需求文档
│   ├── BUILD_FINAL_STATUS.md         # 构建问题说明
│   └── blueshift_anchor_escrow/      # Anchor 项目
│       └── README.md                 # 项目文档
│
├── Task4/                            # Task4 Pinocchio Vault 项目
│   ├── Task4.md                      # 需求文档
│   └── blueshift_vault/              # Pinocchio 项目
│       ├── src/                      # 源代码
│       └── target/deploy/            # 编译产物 (.so, 13KB)
│
├── Task5/                            # Task5 Pinocchio Escrow 项目
│   ├── Task5.md                      # 需求文档
│   └── blueshift_escrow/             # Pinocchio 项目
│       ├── src/                      # 源代码
│       ├── target/deploy/            # 编译产物 (.so, 14KB)
│       ├── README.md                 # 项目文档
│       └── IMPLEMENTATION_GUIDE.md   # 实现指南
│
└── Task6/                            # Task6 Pinocchio AMM 项目
    ├── Task6.md                      # 需求文档
    └── blueshift_native_amm/         # Pinocchio 项目
        ├── src/                      # 源代码
        ├── target/deploy/            # 编译产物 (.so, 17KB)
        └── README.md                 # 项目文档
```

## 🚀 快速开始

### 环境要求

- **Node.js**: >= 18.0.0
- **Rust**: 1.92.0 (nightly)
- **Solana CLI**: >= 1.18.0
- **Anchor**: 0.32.1
- **Cargo**: 1.84.0+

### 安装依赖

```bash
# 安装 Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# 安装 Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

# 安装 Node 依赖（针对 Anchor 项目）
cd Task2/blueshift_anchor_vault && yarn install
```

### 构建项目

#### Task1 (TypeScript)
```bash
cd Task1
# 在 Blueshift 沙盒环境中运行
# 或使用 ts-node 本地运行
```

#### Task2 (Anchor Vault) ✅
```bash
cd Task2/blueshift_anchor_vault
anchor build
anchor test
```

#### Task3 (Anchor Escrow) ✅
```bash
cd Task3/blueshift_anchor_escrow
# 首先降级 blake3 以解决 edition2024 问题
cargo update -p blake3 --precise 1.8.2
# 然后构建
cargo build-sbf
# 输出：target/deploy/blueshift_anchor_escrow.so (307KB)
```

#### Task4 (Pinocchio Vault) ✅
```bash
cd Task4/blueshift_vault
cargo build-sbf
# 输出：target/deploy/blueshift_vault.so (13KB)
```

#### Task5 (Pinocchio Escrow) ✅
```bash
cd Task5/blueshift_escrow
cargo build-sbf
# 输出：target/deploy/blueshift_escrow.so (22KB)
```

#### Task6 (Pinocchio AMM) ✅
```bash
cd Task6/blueshift_native_amm
cargo build-sbf
# 输出：target/deploy/blueshift_native_amm.so (17KB)
```

## ⚠️ 已知问题

### ~~Task3 构建失败 (edition2024)~~ ✅ 已解决！

**问题描述**：
Anchor 0.32.1 的依赖链中包含需要 `edition2024` 特性的 crate（`constant_time_eq 0.4.2`、`blake3 1.8.3`），但 Solana 工具链内置的 Cargo 版本（1.84.0）不支持此特性。

**✅ 解决方案（已实施）**：
```bash
# 降级 blake3 到 1.8.2（不需要 edition2024）
cargo update -p blake3 --precise 1.8.2
```

这个命令会自动降级：
- `blake3`: 1.8.3 → 1.8.2
- `constant_time_eq`: 0.4.2 → 0.3.1

**结果**：
- ✅ Task3 现在可以成功构建！
- ✅ 生成 `blueshift_anchor_escrow.so` (286KB)

详细说明请参阅：`Task3/EDITION2024_FIX.md`

## 📚 技术栈对比

### Anchor vs Pinocchio

| 特性 | Anchor | Pinocchio |
|------|--------|-----------|
| **抽象层级** | 高级框架 | 底层 API |
| **开发难度** | 简单 | 困难 |
| **代码量** | 少 | 多 |
| **性能** | 一般 | 优秀 |
| **程序大小** | 较大 | 极小 |
| **类型安全** | 强 | 需手动保证 |
| **构建问题** | edition2024 ❌ | 无问题 ✅ |
| **适用场景** | 快速开发 | 性能优化 |

### 框架选择建议

- **Anchor**：适合快速原型开发、标准业务逻辑
- **Pinocchio**：适合性能敏感场景、极致优化需求

## 🔧 开发工具

### 推荐 VSCode 插件

- **Rust Analyzer** - Rust 语言支持
- **Anchor** - Anchor 框架支持
- **Solana** - Solana 开发工具

### 有用的命令

```bash
# 查看 Solana 配置
solana config get

# 查看钱包余额
solana balance

# 空投测试 SOL（devnet）
solana airdrop 2

# 查看程序日志
solana logs

# 部署程序
solana program deploy target/deploy/program.so

# Anchor 特定命令
anchor build          # 构建程序
anchor test           # 运行测试
anchor deploy         # 部署程序
anchor idl init       # 初始化 IDL
```

## 📖 学习资源

### 官方文档

- [Solana 文档](https://docs.solana.com/)
- [Anchor 文档](https://www.anchor-lang.com/)
- [Pinocchio 文档](https://docs.rs/pinocchio/)
- [SPL Token 文档](https://spl.solana.com/token)

### 社区资源

- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Stack Exchange](https://solana.stackexchange.com/)

## 🎓 任务详解

### Task1: SPL Token 铸造

**目标**：使用 TypeScript 和 Web3.js 铸造 SPL 代币

**核心概念**：
- Mint Account 创建和初始化
- Associated Token Account (ATA)
- Token 铸造操作
- 交易构建和发送

**关键代码**：
```typescript
// 创建 Mint 账户
const mintKeypair = Keypair.generate();
// 初始化 Mint（6 位小数）
createInitializeMint2Instruction(...)
// 创建 ATA
createAssociatedTokenAccountInstruction(...)
// 铸造 21,000,000 代币
createMintToInstruction(...)
```

### Task2: Anchor Vault

**目标**：实现一个简单的 SOL 金库程序

**核心功能**：
- `deposit`: 存入 SOL
- `withdraw`: 取出 SOL

**核心概念**：
- PDA (Program Derived Address)
- CPI (Cross-Program Invocation)
- Anchor 账户约束
- 租金豁免

**程序 ID**: `11111111111111111111111111111111`

### Task3: Anchor Escrow

**目标**：实现代币托管交换程序

**核心功能**：
- `make`: 创建托管，Maker 存入代币 A，期望换取代币 B
- `take`: Taker 接受托管，提供代币 B，换取代币 A
- `refund`: Maker 取消托管，退回代币 A

**核心概念**：
- 多代币交换
- PDA 控制的代币金库
- Anchor CPI 调用
- ATA 操作
- 安全约束验证

**技术特点**：
- 使用 `anchor-spl` 进行 Token 操作
- Anchor 约束自动验证账户
- 完整的 PDA 和 CPI 实现
- 通过降级 `blake3` 解决 edition2024 问题
- 程序体积 307KB

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: ✅ 已完成并成功构建（通过降级 blake3）

### Task4: Pinocchio Vault

**目标**：使用 Pinocchio 实现 SOL 金库

**核心功能**：
- `Deposit(amount)`: 存入指定数量的 SOL 到 PDA 金库
- `Withdraw`: 取出金库中的所有 SOL

**技术特点**：
- `no_std` 环境
- PDA 账户管理
- 手动 CPI 调用（`pinocchio-system`）
- 最小化程序大小（13KB）
- 零拷贝优化

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: ✅ 已完成并成功构建

### Task5: Pinocchio Escrow

**目标**：使用 Pinocchio 实现代币托管交换

**核心功能**：
- `Make`: 创建托管，Maker 存入代币 A，期望换取代币 B
- `Take`: Taker 接受托管，提供代币 B，换取代币 A
- `Refund`: Maker 取消托管，退回代币 A

**技术特点**：
- 底层 Token 程序 CPI
- 完整的账户验证和安全检查
- Escrow 状态序列化/反序列化
- 账户数据清零（标记为已关闭）
- 性能优化
- 极小程序体积（22KB）

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: ✅ 已完成并成功构建

### Task6: Pinocchio AMM

**目标**：使用 Pinocchio 实现自动做市商（Automated Market Maker）

**核心功能**：
- `Initialize`: 初始化 AMM 配置和 LP Token Mint
- `Deposit`: 存入流动性，获得 LP Token
- `Withdraw`: 销毁 LP Token，取回流动性
- `Swap`: 交换代币（X ↔ Y），使用恒定乘积公式

**技术特点**：
- 恒定乘积曲线（x * y = k）
- LP Token 铸造和销毁
- 手动 Token Mint 初始化
- PDA 权限管理
- 交易手续费机制
- 程序体积 17KB

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: ✅ 已完成并成功构建

## 🐛 故障排除

### 构建失败

```bash
# 清理构建缓存
cargo clean
anchor clean

# 更新依赖
cargo update
yarn install

# 检查 Rust 工具链
rustc --version
cargo --version
```

### 测试失败

```bash
# 启动本地验证器
solana-test-validator

# 检查日志
solana logs

# 重置测试环境
anchor test --skip-local-validator
```

### 部署失败

```bash
# 检查余额
solana balance

# 切换到 devnet
solana config set --url devnet

# 空投 SOL
solana airdrop 2
```

## 🤝 贡献

本项目为学习项目，欢迎提出问题和建议。

## 📄 许可证

MIT License

## 📞 联系方式

如有问题，请创建 Issue 或联系项目维护者。

---

**最后更新**: 2026-01-20

**项目状态**: 
- ✅ **全部 6 个任务完成！**
- ✅ Task3 edition2024 问题已通过降级 blake3 解决

**构建产物**:
- Task2: `blueshift_anchor_vault.so` (Anchor)
- Task3: `blueshift_anchor_escrow.so` (307KB, Anchor, 标准实现)
- Task4: `blueshift_vault.so` (13KB, Pinocchio)
- Task5: `blueshift_escrow.so` (22KB, Pinocchio, 标准实现)
- Task6: `blueshift_native_amm.so` (17KB, Pinocchio)

**技术亮点**:
- 成功解决 edition2024 依赖问题
- 实现了 Anchor 和 Pinocchio 两种 Escrow 方案
- 完成了高级 AMM (自动做市商) 程序
