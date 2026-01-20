# Anchor 金库（Vault）

## 项目概述

保险库允许用户安全地存储他们的资产。保险库是去中心化金融（DeFi）的一个基本构建模块，其核心功能是允许用户安全地存储他们的资产（在本例中是 lamports），并且只有该用户本人可以在之后提取这些资产。

在本次挑战中，我们将构建一个简单的 lamport 保险库，展示如何使用：
- 基本账户（SystemAccount）
- 程序派生地址（PDA）
- 跨程序调用（CPI）

## 核心概念

如果您不熟悉 Anchor，建议先阅读 Anchor 入门，以了解我们将在此程序中使用的核心概念。

## 安装步骤

### 前置要求
在开始之前，请确保已安装 Rust 和 Anchor（如果需要复习，请参阅官方文档）。

### 初始化项目
在终端中运行：

```bash
anchor init blueshift_anchor_vault
cd blueshift_anchor_vault
```

本次挑战不需要额外的 crate，因此您现在可以打开新生成的文件夹，准备开始编码！

## 任务要求

### 挑战 1：实现存款（Deposit）功能
您的程序应允许用户将 SOL 存入他们自己的保险库。

**要求：**
1. 验证金库为空（lamports 为零），以防止重复存款
2. 确保存款金额超过 SystemAccount 的免租金最低限额
3. 使用 CPI 调用系统程序，将 lamports 从签名者转移到金库

### 挑战 2：实现取款（Withdraw）功能
保险库的所有者应能够从他们的保险库中提取 SOL。

**要求：**
1. 验证保险库中是否有 lamports（不为空）
2. 使用保险库的 PDA 以其自身名义签署转账
3. 将保险库中的所有 lamports 转回到签署者

## 程序结构

⚠️ **重要提示**：请记得将程序 ID 更改为 `22222222222222222222222222222222222222222222`，因为测试程序会使用这个 ID。

完整的实现代码请参考 `lib.rs` 文件。

## 技术细节

### 账户结构（VaultAction）

由于两个指令使用相同的账户，为了更简洁和易读，我们创建一个名为 `VaultAction` 的上下文，并将其用于 `deposit` 和 `withdraw`。

**VaultAction 账户结构包含：**

1. **signer**：保险库的所有者，也是创建保险库后唯一可以提取 lamports 的人
2. **vault**：一个由种子 `[b"vault", signer.key().as_ref()]` 派生的 PDA，用于存储 lamports
3. **system_program**：系统程序账户，用于执行转账指令的 CPI

**账户约束说明：**

- `signer`：使用 `mut` 约束，因为我们将在转账过程中修改其 lamports
- `vault`：
  - `mut`：因为我们将在转账过程中修改其 lamports
  - `seeds` 和 `bump`：定义如何从种子派生出有效的 PDA
- `system_program`：检查账户是否设置为可执行，并且地址是否为系统程序地址

### 错误处理

对于这个程序，我们创建两个自定义错误：

1. **VaultAlreadyExists**：用于判断账户中是否已经有 lamports，因为这意味着金库已经存在
2. **InvalidAmount**：不能存入少于基本账户最低租金的金额，或从空金库取款
### 存款（Deposit）实现

**执行步骤：**

1. 验证金库为空（lamports 为零），以防止重复存款
2. 确保存款金额超过 SystemAccount 的免租金最低限额
3. 使用 CPI 调用系统程序，将 lamports 从签名者转移到金库

**关键检查：**

- `require_eq!`：确认金库为空（防止重复存款）
- `require_gt!`：检查金额是否超过免租金阈值

**转账操作：**

使用 Anchor 的系统程序助手调用 Transfer CPI，将 lamports 从签名者账户转移到金库账户。
### 取款（Withdraw）实现

**执行步骤：**

1. 验证保险库中是否有 lamports（不为空）
2. 使用保险库的 PDA 以其自身名义签署转账
3. 将保险库中的所有 lamports 转回到签署者

**安全性保证：**

- 保险库的 PDA 是使用签署者的公钥派生的，确保只有原始存款人可以取款
- PDA 签署转账的能力通过我们提供给 `CpiContext::new_with_signer` 的种子进行验证

## 构建和测试

### 构建程序

在终端中使用以下命令构建您的程序：

```bash
anchor build
```

这将在您的 `target/deploy` 文件夹中直接生成一个 `.so` 文件。

### 测试程序

运行测试：

```bash
anchor test
```

### 部署程序

```bash
anchor deploy
```

## 完成挑战

现在点击 take challenge 按钮并将构建好的 `.so` 文件拖放到那里！

---

**代码实现请查看 `lib.rs` 文件。**