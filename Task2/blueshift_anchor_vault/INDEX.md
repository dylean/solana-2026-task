# 📖 Blueshift Anchor Vault - 文档索引

欢迎！这是完整的 Anchor 金库项目。本文档帮助您快速找到所需信息。

## 🚀 我该从哪里开始？

### 如果您是第一次使用
👉 **阅读：** [`START_HERE.md`](START_HERE.md)  
👉 **运行：** `./setup.sh`

### 如果您想了解完整功能
👉 **阅读：** [`README.md`](README.md)

### 如果您想查看项目交付情况
👉 **阅读：** [`DELIVERY_SUMMARY.md`](DELIVERY_SUMMARY.md)

---

## 📚 文档导航

### 1. 快速开始
- 📄 [`START_HERE.md`](START_HERE.md) - **从这里开始！**
  - 一键启动命令
  - 手动安装步骤
  - 常见问题解答

### 2. 完整指南
- 📄 [`README.md`](README.md) - 详细使用文档
  - 项目简介
  - 完整安装流程
  - 功能说明
  - 测试说明
  - 命令参考
  - 故障排查

- 📄 [`USAGE_GUIDE.md`](USAGE_GUIDE.md) - 使用指南
  - 详细使用步骤
  - 构建验证方法
  - 代码亮点说明
  - 学习建议

### 3. 项目信息
- 📄 [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md) - 项目结构
  - 可视化目录树
  - 文件统计信息
  - 重要文件说明
  - 使用流程图

- 📄 [`PROJECT_COMPLETE.md`](PROJECT_COMPLETE.md) - 完成总结
  - 完整文件清单
  - 三种启动方式
  - 快速命令参考
  - 下一步建议

- 📄 [`DELIVERY_SUMMARY.md`](DELIVERY_SUMMARY.md) - 交付总结
  - 项目统计数据
  - 完整性验证
  - 需求完成度
  - 质量评估

---

## 💻 核心代码

### 程序源代码
📁 `programs/blueshift_anchor_vault/src/lib.rs` - **231 行，150+ 行注释**
- Deposit（存款）函数
- Withdraw（取款）函数
- VaultAction 账户结构
- VaultError 错误枚举
- 详细的中文注释

### 测试文件
📁 `tests/blueshift_anchor_vault.ts` - **294 行，6 个测试用例**
- 成功存款测试
- 重复存款失败测试
- 小额存款失败测试
- 成功取款测试
- 空金库取款失败测试
- 多次存取循环测试

---

## ⚙️ 配置文件

### 核心配置
- 📄 `Anchor.toml` - Anchor 项目配置（程序 ID、网络等）
- 📄 `Cargo.toml` - Rust 工作空间配置
- 📄 `package.json` - Node.js 依赖配置
- 📄 `tsconfig.json` - TypeScript 编译配置

### 程序配置
- 📄 `programs/blueshift_anchor_vault/Cargo.toml` - 程序依赖
- 📄 `programs/blueshift_anchor_vault/Xargo.toml` - Xargo 配置

### 其他配置
- 📄 `.gitignore` - Git 忽略文件
- 📄 `.prettierignore` - Prettier 忽略文件

---

## 🛠️ 辅助工具

### Shell 脚本
- 📄 `setup.sh` - **自动化设置脚本（推荐使用）**
  - 检查所有依赖
  - 安装 npm 包
  - 配置钱包
  - 构建程序

- 📄 `deploy.sh` - **多网络部署脚本**
  - 支持 localhost、devnet、testnet
  - 交互式选择
  - 自动检查余额

### Makefile
- 📄 `Makefile` - **Make 命令配置**
  - `make setup` - 自动设置
  - `make build` - 构建程序
  - `make test` - 运行测试
  - `make deploy` - 部署程序
  - `make help` - 查看所有命令

---

## 🎯 快速命令参考

### 第一次使用
```bash
# 方式 1: 自动化脚本（最简单）
./setup.sh

# 方式 2: Makefile（推荐）
make setup

# 方式 3: 手动步骤
yarn install && anchor build && anchor test
```

### 日常开发
```bash
make build      # 构建程序
make test       # 运行测试
make deploy     # 部署程序
make clean      # 清理构建产物
make info       # 查看程序信息
```

### 查看帮助
```bash
make help       # 查看所有 Make 命令
./setup.sh      # 运行设置脚本（包含依赖检查）
./deploy.sh     # 运行部署脚本（交互式选择网络）
```

---

## 📖 按场景查找文档

### 我想快速开始
→ [`START_HERE.md`](START_HERE.md)

### 我想了解如何使用
→ [`USAGE_GUIDE.md`](USAGE_GUIDE.md)

### 我想查看所有功能
→ [`README.md`](README.md)

### 我想了解项目结构
→ [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md)

### 我想查看项目完成情况
→ [`DELIVERY_SUMMARY.md`](DELIVERY_SUMMARY.md)

### 我想学习代码
→ `programs/blueshift_anchor_vault/src/lib.rs`（含详细注释）

### 我想学习测试
→ `tests/blueshift_anchor_vault.ts`（含 6 个测试用例）

### 我遇到了问题
→ [`README.md`](README.md) 的"故障排查"部分

---

## 🔍 按主题查找

### 安装和设置
- [`START_HERE.md`](START_HERE.md) - 快速开始
- [`README.md`](README.md) - 详细安装步骤
- `setup.sh` - 自动化安装脚本

### 构建和测试
- [`README.md`](README.md) - 测试说明
- [`USAGE_GUIDE.md`](USAGE_GUIDE.md) - 验证方法
- `Makefile` - 构建和测试命令

### 部署
- `deploy.sh` - 多网络部署脚本
- [`README.md`](README.md) - 部署说明

### 学习代码
- `programs/.../src/lib.rs` - 程序源码（详细注释）
- `tests/blueshift_anchor_vault.ts` - 测试用例
- [`USAGE_GUIDE.md`](USAGE_GUIDE.md) - 代码亮点

### 故障排查
- [`README.md`](README.md) - 故障排查部分
- [`USAGE_GUIDE.md`](USAGE_GUIDE.md) - 常见问题

---

## 📊 项目统计

| 项目 | 数量/大小 |
|------|----------|
| 总文件数 | 19 个 |
| 配置文件 | 8 个 |
| 程序代码 | 231 行 |
| 测试代码 | 294 行 |
| 核心代码 | 525 行 |
| 文档文件 | 6 个 |
| 文档总量 | 1000+ 行 |
| 脚本文件 | 3 个 |
| 中文注释 | 300+ 行 |

---

## 🎯 常见任务快速查找

| 我想... | 查看文档 | 运行命令 |
|---------|----------|----------|
| 快速开始 | [`START_HERE.md`](START_HERE.md) | `./setup.sh` |
| 构建程序 | [`README.md`](README.md) | `make build` |
| 运行测试 | [`README.md`](README.md) | `make test` |
| 部署程序 | [`README.md`](README.md) | `./deploy.sh` |
| 查看命令 | 终端运行 | `make help` |
| 学习代码 | `lib.rs` | - |
| 学习测试 | `tests/*.ts` | - |
| 解决问题 | [`README.md`](README.md) | - |

---

## 💡 提示

### 新手用户
1. 先阅读 [`START_HERE.md`](START_HERE.md)
2. 运行 `./setup.sh`
3. 查看 [`README.md`](README.md) 了解详情

### 有经验用户
1. 查看 [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md) 了解结构
2. 直接运行 `make setup`
3. 查看 [`USAGE_GUIDE.md`](USAGE_GUIDE.md) 深入了解

### 代码学习者
1. 打开 `programs/.../src/lib.rs` 查看源码
2. 阅读详细的中文注释
3. 查看 `tests/blueshift_anchor_vault.ts` 学习测试

---

## 📞 需要帮助？

1. 查看 [`README.md`](README.md) 的故障排查部分
2. 运行 `make help` 查看所有可用命令
3. 查看 [`USAGE_GUIDE.md`](USAGE_GUIDE.md) 的常见问题

---

## 🎉 开始使用

**最快速的方式：**
```bash
cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault
./setup.sh
```

**查看所有文档：**
```bash
ls -1 *.md
```

**查看项目结构：**
```bash
tree -L 2
# 或
ls -R
```

---

**项目状态：✅ 100% 完成，可立即使用**

祝您开发愉快！🚀
