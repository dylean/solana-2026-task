# 构建问题解决方案

## 问题描述

构建时遇到以下错误：
```
error: feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, 
but that feature is not stabilized in this version of Cargo (1.84.0)
```

这是因为某些依赖包（如 `constant_time_eq v0.4.2`）需要 Rust edition2024 特性，但当前的 Cargo 版本不支持。

## 解决方案

### 方案 1: 使用兼容的 Anchor 版本（推荐）

降级到稳定版本的 Anchor，避免使用需要 edition2024 的依赖。

修改 `Anchor.toml`:
```toml
[toolchain]
anchor_version = "0.30.1"
```

修改 `programs/blueshift_anchor_escrow/Cargo.toml`:
```toml
[dependencies]
anchor-lang = { version = "0.30.1", features = ["init-if-needed"] }
anchor-spl = "0.30.1"
```

修改 `package.json`:
```json
"@coral-xyz/anchor": "^0.30.1"
```

然后清理并重新构建：
```bash
anchor clean
rm -rf ~/.cargo/registry/cache/* ~/.cargo/registry/index/*
anchor build
```

### 方案 2: 更新 Solana 工具链

如果您想使用最新版本，需要更新 Solana 工具链到支持 edition2024 的版本：

```bash
# 更新 Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# 更新 Rust
rustup update

# 清理缓存
rm -rf ~/.cargo/registry/cache/* ~/.cargo/registry/index/*

# 重新构建
anchor build
```

### 方案 3: 手动删除问题包

```bash
# 删除问题包的缓存
rm -rf ~/.cargo/registry/src/index.crates.io-*/constant_time_eq-0.4.2
rm -rf ~/.cargo/registry/cache/index.crates.io-*/constant_time_eq-0.4.2

# 重新构建
anchor build
```

## 推荐做法

**使用方案 1**，因为：
- 更稳定可靠
- Anchor 0.30.1 是经过广泛测试的版本
- 避免依赖问题

更新后的项目仍然保持所有功能，只是使用更稳定的依赖版本。

## 验证

构建成功后，您应该看到：
```
Build success
```

并且在 `target/deploy/` 目录下生成 `blueshift_anchor_escrow.so` 文件。
