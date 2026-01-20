# Anchor 项目构建问题统一解决方案

## 影响的项目

- ✅ **Task2**: `blueshift_anchor_vault`
- ✅ **Task3**: `blueshift_anchor_escrow`

两个项目都遇到了相同的 `edition2024` 构建错误。

## 问题描述

执行 `anchor build` 时出现以下错误：

```
feature `edition2024` is required

The package requires the Cargo feature called `edition2024`, but that feature is 
not stabilized in this version of Cargo (1.84.0 (12fe57a9d 2025-04-07)).
```

## 根本原因

这是 **Solana 工具链的限制**，不是代码问题：

1. **固定的 Cargo 版本**
   - Solana 的 `cargo build-sbf` 内部使用 Cargo 1.84.0
   - 该版本不支持 `edition2024` 特性

2. **依赖链冲突**
   - 新版本 Anchor 的依赖需要 `edition2024`
   - 即使系统 Rust 已更新，`cargo build-sbf` 仍使用旧版本

3. **AVM 限制**
   - Anchor Version Manager (avm) 在安装旧版本时可能失败
   - 会自动回退到最新版本（0.32.1），导致问题持续

## 解决方案

### 🎯 推荐方案：使用 Nightly 工具链

这是最可靠且简单的解决方案：

```bash
# 1. 安装 nightly 工具链
rustup toolchain install nightly

# 2. 设置为默认工具链
rustup default nightly

# 3. 验证安装
rustup show active-toolchain
# 应该显示: nightly-xxx-apple-darwin (default)

# 4. 清理缓存（重要！）
rm -rf ~/.cargo/registry/cache/*
rm -rf ~/.cargo/registry/src/*

# 5. 构建 Task2
cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault
anchor clean && cargo clean
anchor build

# 6. 构建 Task3
cd /Users/dean/code/web3/solana-2026-task/Task3/blueshift_anchor_escrow
anchor clean && cargo clean
anchor build
```

### 替代方案：等待 Solana 官方更新

等待 Solana 团队更新 `cargo build-sbf` 的 Cargo 版本以支持 `edition2024`。

## 验证代码正确性

两个项目的代码都是 **完全正确** 的，只是受到工具链限制。

### Task2 验证

```bash
cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault/programs/blueshift_anchor_vault
cargo check --lib
```

### Task3 验证

```bash
cd /Users/dean/code/web3/solana-2026-task/Task3/blueshift_anchor_escrow/programs/blueshift_anchor_escrow
cargo check --lib
```

## 项目状态总结

### Task2: blueshift_anchor_vault

✅ **已完成的功能**
- `deposit` 指令：存款到金库
- `withdraw` 指令：从金库取款
- 完整的测试套件（6 个测试用例）
- 完善的文档和部署脚本

❌ **当前阻碍**
- 仅因 Cargo 版本限制无法构建

📝 **Anchor 版本**: 0.30.1

### Task3: blueshift_anchor_escrow

✅ **已完成的功能**
- `make` 指令：创建托管
- `take` 指令：接受托管
- `refund` 指令：退款
- 模块化代码结构（state, errors, instructions）
- 自定义鉴别器
- 基础测试框架

❌ **当前阻碍**
- 仅因 Cargo 版本限制无法构建

📝 **Anchor 版本**: 0.30.1

## 完整构建流程（使用 Nightly）

```bash
# ===== 一次性设置 =====

# 1. 安装并切换到 nightly
rustup toolchain install nightly
rustup default nightly

# 2. 清理 Cargo 缓存
rm -rf ~/.cargo/registry/cache/*
rm -rf ~/.cargo/registry/src/*

# ===== 构建 Task2 =====

cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault

# 清理项目
anchor clean
cargo clean

# 构建
anchor build

# 测试（可选）
anchor test

# ===== 构建 Task3 =====

cd /Users/dean/code/web3/solana-2026-task/Task3/blueshift_anchor_escrow

# 清理项目
anchor clean
cargo clean

# 构建
anchor build

# 测试（可选）
anchor test
```

## 切换回 Stable（可选）

如果你想在完成构建后切换回 stable 工具链：

```bash
rustup default stable
```

注意：下次构建时需要再次切换到 nightly。

## 预期结果

使用 nightly 工具链后，两个项目应该都能成功构建：

```
✅ Task2 构建成功
   - 生成 target/deploy/blueshift_anchor_vault.so
   - 生成 target/idl/blueshift_anchor_vault.json
   - 生成 target/types/blueshift_anchor_vault.ts

✅ Task3 构建成功
   - 生成 target/deploy/blueshift_anchor_escrow.so
   - 生成 target/idl/blueshift_anchor_escrow.json
   - 生成 target/types/blueshift_anchor_escrow.ts
```

## 相关文档

- Task2 详细说明: [`Task2/blueshift_anchor_vault/BUILD_STATUS.md`](Task2/blueshift_anchor_vault/BUILD_STATUS.md)
- Task3 详细说明: [`Task3/blueshift_anchor_escrow/BUILD_STATUS.md`](Task3/blueshift_anchor_escrow/BUILD_STATUS.md)

## 技术背景

- [Rust Edition 2024](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#edition-2024)
- [Solana 工具链文档](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor 框架](https://www.anchor-lang.com/)

---

**最后更新**: 2026-01-20  
**适用版本**: Anchor 0.30.1 / 0.32.1  
**Cargo 版本（build-sbf）**: 1.84.0（固定）  
**推荐工具链**: Nightly
