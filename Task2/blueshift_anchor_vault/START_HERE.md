# 🚀 快速启动指南

## 一键启动（推荐）

```bash
cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault
./setup.sh
```

## 手动启动步骤

### 1. 进入项目目录
```bash
cd /Users/dean/code/web3/solana-2026-task/Task2/blueshift_anchor_vault
```

### 2. 安装依赖
```bash
yarn install
# 或
npm install
```

### 3. 构建程序
```bash
anchor build
```

### 4. 启动测试验证器（新终端窗口）
```bash
solana-test-validator
```

### 5. 运行测试（原终端）
```bash
anchor test --skip-local-validator
```

### 6. 部署程序
```bash
anchor deploy
```

## 验证安装

确保已安装以下工具：

```bash
# 检查 Rust
rustc --version

# 检查 Solana CLI
solana --version

# 检查 Anchor
anchor --version

# 检查 Node.js
node --version
```

如果缺少任何工具，请参考 README.md 中的安装说明。

## 常见问题

### Q: 如何安装 Anchor？
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Q: 测试失败怎么办？
1. 确保本地验证器正在运行
2. 检查钱包是否有足够余额
3. 清理并重新构建：`anchor clean && anchor build`

### Q: 如何查看程序日志？
```bash
solana logs
```

## 项目已包含

✅ 完整的 Anchor 程序代码（带详细中文注释）  
✅ 6 个完整的测试用例  
✅ 自动化设置脚本  
✅ 部署脚本（支持多网络）  
✅ 完整的配置文件  
✅ 详细的 README 文档  

## 下一步

程序构建成功后，您可以：

1. **查看构建产物**：`target/deploy/blueshift_anchor_vault.so`
2. **查看 IDL**：`target/idl/blueshift_anchor_vault.json`
3. **运行测试**：验证所有功能
4. **部署到 devnet**：`./deploy.sh` 选择选项 2

---

祝您开发愉快！🎉
