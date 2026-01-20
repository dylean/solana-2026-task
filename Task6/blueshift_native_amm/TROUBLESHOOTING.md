# Task6 AMM 问题诊断指南

## 错误 1: `invalid account data for instruction`

### 症状
- 计算单元消耗很少（< 10000）
- 在指令早期就失败

### 可能原因

#### 1. 账户顺序错误（最常见！）

**错误示例**:
```typescript
// ❌ 错误的顺序
const accounts = [
  { pubkey: user.publicKey, ... },
  { pubkey: mintLpPDA, ... },      // ❌ 第1个应该是 config！
  { pubkey: configPDA, ... },      // ❌ 这应该是第1个
  ...
];
```

**正确示例**:
```typescript
// ✅ 正确的顺序
const accounts = [
  { pubkey: user.publicKey, ... },       // 0
  { pubkey: configPDA, ... },            // 1 ✅
  { pubkey: mintLpPDA, ... },            // 2
  ...
];
```

**解决方案**:
1. 查看 [ACCOUNT_ORDER.md](./ACCOUNT_ORDER.md)
2. 逐个检查账户顺序
3. **重点检查第 0、1、2 个账户**

#### 2. PDA 计算错误

**检查 Config PDA**:
```typescript
const [configPDA, bump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("config"),
    seedBuffer,          // ✅ 8 字节 u64
    mintX.toBuffer(),    // ✅ 32 字节
    mintY.toBuffer(),    // ✅ 32 字节
  ],
  PROGRAM_ID
);
```

**检查 LP Mint PDA**:
```typescript
const [mintLpPDA, lpBump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("mint_lp"),
    configPDA.toBuffer(),  // ✅ 使用 Config PDA
  ],
  PROGRAM_ID
);
```

**检查 Vault（ATA）**:
```typescript
// ✅ 正确：Vault 是 Config 的 ATA
const vaultX = getAssociatedTokenAddressSync(
  mintX,       // mint
  configPDA,   // owner（Config PDA）
  true         // ✅ allowOwnerOffCurve = true
);

// ❌ 错误：使用 user 作为 owner
const vaultX = getAssociatedTokenAddressSync(
  mintX,
  user.publicKey,  // ❌ 错误！
  false
);
```

#### 3. 指令数据长度错误

**Initialize 指令**:
- 最小长度：76 字节（不含 authority）
- 完整长度：108 字节（含 authority）
- 格式：`seed(8) + fee(2) + mint_x(32) + mint_y(32) + config_bump(1) + lp_bump(1) + [authority(32)]`

**Deposit/Withdraw 指令**:
- 长度：32 字节
- 格式：`amount(8) + max_x/min_x(8) + max_y/min_y(8) + expiration(8)`

**Swap 指令**:
- 长度：25 字节
- 格式：`is_x(1) + amount(8) + min(8) + expiration(8)`

---

## 错误 2: `instruction requires an initialized account`

### 症状
- 计算单元消耗很少（< 200）
- 错误提示账户未初始化

### 可能原因

#### 1. Config 未初始化

**检查**:
```typescript
const configAccount = await connection.getAccountInfo(configPDA);
if (!configAccount) {
  console.error("Config PDA 不存在，需要先调用 Initialize");
}
if (configAccount.data.length === 0) {
  console.error("Config 账户数据为空");
}
```

**解决方案**:
1. 先调用 Initialize 指令
2. 等待交易确认
3. 再调用其他指令

#### 2. 使用了错误的 Config PDA

**验证**:
```typescript
// 重新计算 PDA
const [expectedConfig] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("config"),
    seedBuffer,
    mintX.toBuffer(),
    mintY.toBuffer(),
  ],
  PROGRAM_ID
);

if (!expectedConfig.equals(configPDA)) {
  console.error("Config PDA 不匹配！");
  console.error("期望:", expectedConfig.toBase58());
  console.error("实际:", configPDA.toBase58());
}
```

---

## 错误 3: `missing required signature`

### 症状
- 错误提示缺少签名

### 解决方案

```typescript
// ✅ 确保 user 设置为 signer
const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: true },  // ✅
  ...
];

// ✅ 在发送交易时签名
const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [user]  // ✅ 签名者数组
);
```

---

## 错误 4: 交易失败但没有详细信息

### 诊断步骤

**1. 启用详细日志**:
```typescript
const signature = await connection.sendTransaction(transaction, [user], {
  skipPreflight: false,
  preflightCommitment: 'confirmed',
});

// 获取交易详情
const tx = await connection.getTransaction(signature, {
  commitment: 'confirmed',
  maxSupportedTransactionVersion: 0,
});

console.log("程序日志:", tx?.meta?.logMessages);
```

**2. 检查账户余额**:
```typescript
// 检查租金是否足够
const balance = await connection.getBalance(payer.publicKey);
console.log("Payer 余额:", balance / LAMPORTS_PER_SOL, "SOL");

// 至少需要 0.01 SOL 用于租金
if (balance < 0.01 * LAMPORTS_PER_SOL) {
  console.error("余额不足！");
}
```

**3. 模拟交易**:
```typescript
const simulation = await connection.simulateTransaction(transaction);
console.log("模拟结果:", simulation);
if (simulation.value.err) {
  console.error("模拟失败:", simulation.value.err);
  console.error("日志:", simulation.value.logs);
}
```

---

## 调试清单

### 发送交易前检查

- [ ] **账户顺序**: 与 ACCOUNT_ORDER.md 完全一致
- [ ] **账户数量**: Initialize(5), Deposit(9), Withdraw(9), Swap(7)
- [ ] **PDA 计算**: 使用正确的 seeds
- [ ] **Vault 计算**: 使用 `getAssociatedTokenAddressSync(mint, configPDA, true)`
- [ ] **签名者**: user 设置为 isSigner: true
- [ ] **指令数据**: 长度和格式正确
- [ ] **指令鉴别器**: 0(Init), 1(Deposit), 2(Withdraw), 3(Swap)

### Initialize 指令特定检查

- [ ] Config PDA 使用 4 个 seeds: "config", seed(8字节), mint_x, mint_y
- [ ] LP Mint PDA 使用 2 个 seeds: "mint_lp", configPDA
- [ ] seed 是 u64 (8 字节)
- [ ] fee <= 10000 (100%)
- [ ] mint_x ≠ mint_y

### Deposit/Withdraw 指令特定检查

- [ ] Config 已初始化
- [ ] Vault 是 Config 的 ATA（不是 user 的 ATA！）
- [ ] amount, max_x, max_y, min_x, min_y 都 > 0
- [ ] 用户有足够的代币余额

### Swap 指令特定检查

- [ ] Config 已初始化且状态为 Initialized
- [ ] amount > 0, min > 0
- [ ] 用户有足够的输入代币

---

## 常见错误模式

### 模式 1: "所有指令都失败"

**原因**: 账户顺序错误

**解决**:
```typescript
// 打印所有账户
console.log("=== 账户列表 ===");
accounts.forEach((acc, idx) => {
  console.log(`${idx}: ${acc.pubkey.toBase58()}`);
});

// 对比 ACCOUNT_ORDER.md
```

### 模式 2: "Initialize 成功，但 Deposit 失败"

**原因**: Vault PDA 计算错误

**解决**:
```typescript
// ✅ 正确的 Vault 计算
const vaultX = getAssociatedTokenAddressSync(
  mintX,
  configPDA,  // ✅ owner 是 configPDA，不是 user!
  true        // ✅ allowOwnerOffCurve
);
```

### 模式 3: "Deposit 成功，但 Swap 失败"

**原因**: 账户数量不对

**解决**:
- Deposit 需要 9 个账户
- Swap 只需要 7 个账户（没有 mint_lp 和 user_lp_ata）

---

## 获取帮助

如果以上方法都无法解决问题：

1. **检查程序日志**:
   ```bash
   solana logs <PROGRAM_ID>
   ```

2. **查看交易详情**:
   ```typescript
   const tx = await connection.getTransaction(signature);
   console.log(tx?.meta?.logMessages);
   ```

3. **参考文档**:
   - [ACCOUNT_ORDER.md](./ACCOUNT_ORDER.md) - 账户顺序参考
   - [CLIENT_GUIDE.md](./CLIENT_GUIDE.md) - 完整示例代码
   - [FINAL_FIX_SUMMARY.md](./FINAL_FIX_SUMMARY.md) - 常见问题

4. **验证假设**:
   - 使用 `console.log` 打印所有 PDA
   - 对比期望值和实际值
   - 逐个验证账户地址

---

## 快速测试

```typescript
// 测试脚本
async function testAMM() {
  try {
    // 1. Initialize
    console.log("1. 测试 Initialize...");
    const { configPDA, mintLpPDA } = await initializeAMM(...);
    console.log("✅ Initialize 成功");
    
    // 2. 验证账户
    const configAccount = await connection.getAccountInfo(configPDA);
    if (!configAccount) throw new Error("Config 未创建");
    console.log("✅ Config 账户存在");
    
    // 3. Deposit
    console.log("2. 测试 Deposit...");
    await depositLiquidity(...);
    console.log("✅ Deposit 成功");
    
    // 4. Swap
    console.log("3. 测试 Swap...");
    await swapTokens(...);
    console.log("✅ Swap 成功");
    
    // 5. Withdraw
    console.log("4. 测试 Withdraw...");
    await withdrawLiquidity(...);
    console.log("✅ Withdraw 成功");
    
    console.log("\n🎉 所有测试通过！");
  } catch (error) {
    console.error("\n❌ 测试失败:", error);
    console.error("\n请检查:");
    console.error("1. ACCOUNT_ORDER.md - 账户顺序");
    console.error("2. PDA 计算");
    console.error("3. 指令数据格式");
  }
}
```

---

**记住**: 99% 的 `invalid account data` 错误都是账户顺序或 PDA 计算问题！
