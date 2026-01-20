# Blueshift Anchor Escrow

一个使用 Anchor 框架开发的 Solana 托管程序，实现无需信任的代币交换。

## 🎯 项目概述

托管服务是一种强大的金融工具，可以在两方之间实现安全的代币交换。本程序实现了一个数字保险箱，用户可以锁定代币 A，等待另一方用户存入代币 B，然后完成交换。

### 核心功能

1. **Make（创建托管）** - Discriminator: 0
   - 创建者发起托管报价
   - 存入代币 A 到金库
   - 设定期望接收的代币 B 数量

2. **Take（接受托管）** - Discriminator: 1
   - 接受者接受托管报价
   - 提供代币 B 给创建者
   - 获得代币 A 从金库

3. **Refund（退款）** - Discriminator: 2
   - 创建者取消托管报价
   - 退回代币 A 给创建者
   - 关闭托管账户

## 🚀 快速开始

### 方式 1: 自动化脚本（推荐）

```bash
cd blueshift_anchor_escrow
./setup.sh
```

### 方式 2: Makefile

```bash
make setup
```

### 方式 3: 手动步骤

```bash
# 1. 安装依赖
yarn install

# 2. 构建程序
anchor build

# 3. 运行测试
anchor test
```

## 📁 项目结构

```
blueshift_anchor_escrow/
├── programs/
│   └── blueshift_anchor_escrow/
│       └── src/
│           ├── lib.rs              # 程序入口
│           ├── state.rs            # 状态定义
│           ├── errors.rs           # 错误定义
│           └── instructions/
│               ├── mod.rs          # 模块导出
│               ├── make.rs         # 创建托管
│               ├── take.rs         # 接受托管
│               └── refund.rs       # 退款
├── tests/
│   └── blueshift_anchor_escrow.ts  # 测试文件
├── Anchor.toml                     # Anchor 配置
├── Cargo.toml                      # Rust 配置
├── package.json                    # Node.js 配置
└── README.md                       # 本文件
```

## 🎓 代码说明

### State (state.rs)

定义了 Escrow 账户结构：
- `seed`: 随机数种子（允许多个托管）
- `maker`: 创建者地址
- `mint_a`: 代币 A 铸币地址
- `mint_b`: 代币 B 铸币地址
- `receive`: 期望接收的代币 B 数量
- `bump`: PDA bump seed

### Make 指令

创建托管的流程：
1. 验证金额有效性
2. 初始化 Escrow 账户
3. 创建金库（Vault）
4. 转移代币 A 到金库

### Take 指令

接受托管的流程：
1. 转移代币 B 给创建者
2. 从金库转移代币 A 给接受者
3. 关闭金库账户
4. 关闭 Escrow 账户

### Refund 指令

退款的流程：
1. 验证只有创建者可以退款
2. 从金库退回代币 A
3. 关闭金库账户
4. 关闭 Escrow 账户

## 🔧 常用命令

```bash
# 构建
make build
# 或
anchor build

# 测试
make test
# 或
anchor test

# 部署
make deploy
# 或
anchor deploy

# 清理
make clean

# 查看帮助
make help
```

## 📊 测试

测试文件包含三个完整的测试用例：

1. ✅ 创建托管
2. ✅ 接受托管
3. ✅ 退款

运行测试：
```bash
anchor test
```

## 🛡️ 安全特性

1. **PDA 控制**
   - 金库由 Escrow PDA 控制
   - 只有程序可以代表 PDA 签署

2. **约束验证**
   - has_one 约束验证账户所有权
   - 防止未授权操作

3. **原子性交易**
   - 所有操作在一个交易中完成
   - 要么全部成功，要么全部失败

4. **租金返还**
   - 关闭账户时返还租金
   - 降低用户成本

## ⚠️ 安全警告

SPL Token-2022 的某些扩展功能可能引入漏洞：

- **转账钩子**：可能阻止转账或锁定资金
- **保密转账**：可能导致余额不一致
- **默认账户状态**：可能冻结新创建的账户

**最佳实践：**
- 确保 mint_a 和 mint_b 由同一个代币程序拥有
- 使用经过充分审计的代币（如 USDC、wSOL）
- 避免使用未经验证的 Token-2022 铸币

## 📝 依赖要求

- **Anchor**: ^0.32.1
- **Rust**: 1.92.0 或更高版本
- **anchor-lang**: ^0.32.1 (with init-if-needed feature)
- **anchor-spl**: ^0.32.1

## 💡 使用示例

### 创建托管

```typescript
await program.methods
  .make(
    seed,           // 随机数种子
    receiveAmount,  // 期望接收的代币 B 数量
    depositAmount   // 存入的代币 A 数量
  )
  .accounts({
    maker: makerPublicKey,
    mintA: mintAPublicKey,
    mintB: mintBPublicKey,
    makerAtaA: makerTokenAAccount,
    vault: vaultPDA,
    escrow: escrowPDA,
  })
  .rpc();
```

### 接受托管

```typescript
await program.methods
  .take()
  .accounts({
    taker: takerPublicKey,
    maker: makerPublicKey,
    mintA: mintAPublicKey,
    mintB: mintBPublicKey,
    takerAtaA: takerTokenAAccount,
    takerAtaB: takerTokenBAccount,
    makerAtaB: makerTokenBAccount,
    escrow: escrowPDA,
    vault: vaultPDA,
  })
  .rpc();
```

### 退款

```typescript
await program.methods
  .refund()
  .accounts({
    maker: makerPublicKey,
    mintA: mintAPublicKey,
    makerAtaA: makerTokenAAccount,
    escrow: escrowPDA,
    vault: vaultPDA,
  })
  .rpc();
```

## 🐛 故障排查

### 构建失败
```bash
anchor clean
anchor build
```

### 测试失败
```bash
# 确保验证器正在运行
solana-test-validator

# 在另一个终端运行测试
anchor test --skip-local-validator
```

### 找不到钱包
```bash
solana-keygen new --no-bip39-passphrase
```

## 📚 学习资源

- [Anchor 官方文档](https://www.anchor-lang.com/)
- [Solana 官方文档](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)

## 📄 许可证

MIT License

---

**项目状态**：✅ 完整可用  
**程序 ID**：22222222222222222222222222222222222222222222  
**创建日期**：2026-01-20

祝您使用愉快！🚀
