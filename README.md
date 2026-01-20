# Solana 2026 开发任务集

> Solana 智能合约开发实战项目集合

## 📋 项目概述

本项目包含 6 个 Solana 智能合约开发任务，涵盖从基础的 SPL Token 操作到高级的 Anchor 和 Pinocchio 框架程序开发。

## 🎯 任务列表

| 任务 | 名称 | 框架 | 状态 | 说明 |
|------|------|------|------|------|
| Task1 | SPL Token 铸造 | TypeScript | ✅ 完成 | 使用 Web3.js 铸造 SPL 代币 |
| Task2 | Anchor Vault | Anchor | ✅ 完成 | SOL 金库存取程序 |
| Task3 | Anchor Escrow | Anchor | ⚠️ 无法构建 | 代币托管交换程序（edition2024 问题）|
| Task4 | Pinocchio Vault | Pinocchio | 🚧 框架完成 | SOL 金库（底层实现）|
| Task5 | Pinocchio Escrow | Pinocchio | 🚧 框架完成 | 代币托管（底层实现）|
| Task6 | Pinocchio 高级 | Pinocchio | ⏳ 待开发 | 高级 Pinocchio 程序 |

## 📁 项目结构

```
solana-2026-task/
├── Task1.md                          # Task1 需求文档
├── Task1/                            # Task1 代码（TypeScript）
│   └── mint-spl-token.ts
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
│   ├── blueshift_anchor_escrow/      # Anchor 项目
│   └── BUILD_IMPOSSIBLE.md           # 构建问题说明
│
├── Task4/                            # Task4 Pinocchio Vault 项目
│   ├── Task4.md                      # 需求文档
│   └── blueshift_vault/              # Pinocchio 项目
│       └── target/deploy/            # 编译产物 (.so)
│
├── Task5/                            # Task5 Pinocchio Escrow 项目
│   ├── Task5.md                      # 需求文档
│   └── blueshift_escrow/             # Pinocchio 项目
│       ├── src/                      # 源代码
│       ├── target/deploy/            # 编译产物 (.so)
│       └── README.md                 # 项目文档
│
├── Task6/                            # Task6 项目
│   └── Task6.md                      # 需求文档
│
├── ANCHOR_BUILD_SOLUTION.md          # Anchor 构建问题统一说明
└── README.md                         # 本文件
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

#### Task3 (Anchor Escrow) ⚠️
```bash
cd Task3/blueshift_anchor_escrow
# 注意：由于 edition2024 问题，当前无法构建
# 详见 BUILD_IMPOSSIBLE.md
```

#### Task4 (Pinocchio Vault) 🚧
```bash
cd Task4/blueshift_vault
cargo build-sbf
# 输出：target/deploy/blueshift_vault.so
```

#### Task5 (Pinocchio Escrow) 🚧
```bash
cd Task5/blueshift_escrow
cargo build-sbf
# 输出：target/deploy/blueshift_escrow.so (3.4KB)
```

## ⚠️ 已知问题

### Task3 构建失败 (edition2024)

**问题描述**：
Anchor 0.32.1 的依赖链中包含需要 `edition2024` 特性的 crate（`constant_time_eq 0.4.2`、`blake3 1.8.3`），但 Solana 工具链内置的 Cargo 版本（1.84.0）不支持此特性。

**影响范围**：
- Task3 (Anchor Escrow) 无法构建

**解决方案**：
1. ✅ **已采用**：使用 Pinocchio 重写（Task5）
2. ⏳ **等待**：Solana 官方更新工具链（预计 1-3 个月）
3. 🔧 **高级**：自定义编译支持 edition2024 的 Solana 工具链

详细说明请参阅：
- `Task3/blueshift_anchor_escrow/BUILD_IMPOSSIBLE.md`
- `ANCHOR_BUILD_SOLUTION.md`

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

### Task3: Anchor Escrow (⚠️ 无法构建)

**目标**：实现代币托管交换程序

**核心功能**：
- `make`: 创建托管
- `take`: 接受托管
- `refund`: 退款

**核心概念**：
- 多代币交换
- 托管状态管理
- ATA 操作
- 安全检查

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: 由于 edition2024 问题无法构建，已用 Pinocchio 重写为 Task5

### Task4: Pinocchio Vault

**目标**：使用 Pinocchio 实现 SOL 金库

**核心功能**：
- `Deposit`: 存入 SOL
- `Withdraw`: 取出 SOL

**技术特点**：
- `no_std` 环境
- 手动 CPI 调用
- 最小化程序大小
- 零拷贝优化

**程序 ID**: `11111111111111111111111111111111`

### Task5: Pinocchio Escrow

**目标**：使用 Pinocchio 实现代币托管

**核心功能**：
- `Make`: 创建托管
- `Take`: 接受托管
- `Refund`: 退款

**技术特点**：
- 底层 Token 程序 CPI
- 手动账户验证
- 性能优化
- 极小程序体积 (3.4KB)

**程序 ID**: `22222222222222222222222222222222222222222222`

**状态**: ✅ 基础框架完成，成功构建 .so 文件

### Task6: Pinocchio 高级

**目标**：高级 Pinocchio 程序开发

**状态**: ⏳ 待开发

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
- ✅ Task1, Task2 完全可用
- 🚧 Task4, Task5 框架完成
- ⚠️ Task3 因工具链问题暂时无法构建
- ⏳ Task6 待开发
