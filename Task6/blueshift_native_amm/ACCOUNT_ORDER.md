# Task6 AMM 账户顺序参考

## ⚠️ 重要提示

**账户顺序必须完全匹配程序期望的顺序，否则会导致 `invalid account data` 错误！**

---

## 1. Initialize 指令

**指令鉴别器**: `0`

**账户顺序**（5个账户）:
```
0. initializer (signer, writable) - 支付租金的账户
1. config (writable) - Config PDA
2. mint_lp (writable) - LP Token Mint PDA
3. system_program - SystemProgram.programId
4. token_program - TOKEN_PROGRAM_ID
```

**TypeScript 示例**:
```typescript
const accounts = [
  { pubkey: payer.publicKey, isSigner: true, isWritable: true },      // 0
  { pubkey: configPDA, isSigner: false, isWritable: true },           // 1
  { pubkey: mintLpPDA, isSigner: false, isWritable: true },           // 2
  { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // 3
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },  // 4
];
```

---

## 2. Deposit 指令

**指令鉴别器**: `1`

**账户顺序**（9个账户）:
```
0. user (signer, writable) - 用户钱包
1. config - Config PDA
2. mint_lp (writable) - LP Token Mint PDA
3. vault_x (writable) - X 代币金库（Config 的 ATA for mint_x）
4. vault_y (writable) - Y 代币金库（Config 的 ATA for mint_y）
5. user_x_ata (writable) - 用户的 X 代币账户
6. user_y_ata (writable) - 用户的 Y 代币账户
7. user_lp_ata (writable) - 用户的 LP 代币账户
8. token_program - TOKEN_PROGRAM_ID
```

**TypeScript 示例**:
```typescript
// 计算 Vault（Config 的 ATA）
const vaultX = getAssociatedTokenAddressSync(mintX, configPDA, true);
const vaultY = getAssociatedTokenAddressSync(mintY, configPDA, true);

// 计算用户 ATA
const userXAta = getAssociatedTokenAddressSync(mintX, user.publicKey);
const userYAta = getAssociatedTokenAddressSync(mintY, user.publicKey);
const userLpAta = getAssociatedTokenAddressSync(mintLp, user.publicKey);

const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: true },       // 0
  { pubkey: configPDA, isSigner: false, isWritable: false },          // 1
  { pubkey: mintLpPDA, isSigner: false, isWritable: true },           // 2
  { pubkey: vaultX, isSigner: false, isWritable: true },              // 3
  { pubkey: vaultY, isSigner: false, isWritable: true },              // 4
  { pubkey: userXAta, isSigner: false, isWritable: true },            // 5
  { pubkey: userYAta, isSigner: false, isWritable: true },            // 6
  { pubkey: userLpAta, isSigner: false, isWritable: true },           // 7
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },  // 8
];
```

---

## 3. Withdraw 指令

**指令鉴别器**: `2`

**账户顺序**（9个账户）:
```
0. user (signer, writable) - 用户钱包
1. config - Config PDA
2. mint_lp (writable) - LP Token Mint PDA
3. vault_x (writable) - X 代币金库
4. vault_y (writable) - Y 代币金库
5. user_x_ata (writable) - 用户的 X 代币账户
6. user_y_ata (writable) - 用户的 Y 代币账户
7. user_lp_ata (writable) - 用户的 LP 代币账户
8. token_program - TOKEN_PROGRAM_ID
```

**TypeScript 示例**:
```typescript
// 与 Deposit 相同的账户顺序
const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: true },       // 0
  { pubkey: configPDA, isSigner: false, isWritable: false },          // 1
  { pubkey: mintLpPDA, isSigner: false, isWritable: true },           // 2
  { pubkey: vaultX, isSigner: false, isWritable: true },              // 3
  { pubkey: vaultY, isSigner: false, isWritable: true },              // 4
  { pubkey: userXAta, isSigner: false, isWritable: true },            // 5
  { pubkey: userYAta, isSigner: false, isWritable: true },            // 6
  { pubkey: userLpAta, isSigner: false, isWritable: true },           // 7
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },  // 8
];
```

---

## 4. Swap 指令

**指令鉴别器**: `3`

**账户顺序**（7个账户）:
```
0. user (signer) - 用户钱包
1. config - Config PDA
2. vault_x (writable) - X 代币金库
3. vault_y (writable) - Y 代币金库
4. user_x_ata (writable) - 用户的 X 代币账户
5. user_y_ata (writable) - 用户的 Y 代币账户
6. token_program - TOKEN_PROGRAM_ID
```

**TypeScript 示例**:
```typescript
const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: false },      // 0
  { pubkey: configPDA, isSigner: false, isWritable: false },          // 1
  { pubkey: vaultX, isSigner: false, isWritable: true },              // 2
  { pubkey: vaultY, isSigner: false, isWritable: true },              // 3
  { pubkey: userXAta, isSigner: false, isWritable: true },            // 4
  { pubkey: userYAta, isSigner: false, isWritable: true },            // 5
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },  // 6
];
```

---

## 重要提示

### 1. PDA 计算

**Config PDA**:
```typescript
const [configPDA, configBump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("config"),
    seedBuffer,                // u64 (8 bytes)
    mintX.toBuffer(),
    mintY.toBuffer(),
  ],
  PROGRAM_ID
);
```

**LP Mint PDA**:
```typescript
const [mintLpPDA, lpBump] = PublicKey.findProgramAddressSync(
  [Buffer.from("mint_lp"), configPDA.toBuffer()],
  PROGRAM_ID
);
```

**Vault（ATA）**:
```typescript
import { getAssociatedTokenAddressSync } from '@solana/spl-token';

// Vault 是 Config 拥有的 Associated Token Account
const vaultX = getAssociatedTokenAddressSync(
  mintX,          // mint
  configPDA,      // owner
  true            // allowOwnerOffCurve = true（因为 owner 是 PDA）
);
```

### 2. 常见错误

| 错误信息 | 可能原因 |
|---------|---------|
| `invalid account data` | 账户顺序错误 |
| `invalid account data` | PDA 计算错误 |
| `uninitialized account` | Config 未初始化，先调用 Initialize |
| `missing required signature` | user 账户未签名 |

### 3. 调试技巧

**验证 PDA**:
```typescript
// 确保客户端计算的 PDA 与程序计算的一致
const [expectedConfig] = PublicKey.findProgramAddressSync(...);
console.log("Expected Config PDA:", expectedConfig.toBase58());
console.log("Actual Config PDA:", configPDA.toBase58());
```

**验证账户顺序**:
```typescript
accounts.forEach((acc, idx) => {
  console.log(`账户 ${idx}:`, acc.pubkey.toBase58());
});
```

---

## 完整测试流程

```typescript
// 1. Initialize
const { configPDA, mintLpPDA } = await initializeAMM(
  connection,
  payer,
  mintX,
  mintY,
  30, // 0.3% fee
);

// 2. Deposit
await depositLiquidity(
  connection,
  user,
  configPDA,
  mintX,
  mintY,
  mintLpPDA,
  1000000n, // 1 X token
  1000000n, // 1 Y token
);

// 3. Swap
await swapTokens(
  connection,
  user,
  configPDA,
  mintX,
  mintY,
  true, // X -> Y
  100000n, // 0.1 X token
);

// 4. Withdraw
await withdrawLiquidity(
  connection,
  user,
  configPDA,
  mintX,
  mintY,
  mintLpPDA,
  500000n, // 0.5 LP token
);
```

---

## 检查清单

在发送交易前，请确保：

- [ ] 指令鉴别器正确（0, 1, 2, 或 3）
- [ ] 账户数量正确（5, 9, 9, 或 7）
- [ ] 账户顺序与文档完全一致
- [ ] PDA 计算正确（Config 和 LP Mint）
- [ ] Vault 使用 `getAssociatedTokenAddressSync` 并设置 `allowOwnerOffCurve = true`
- [ ] 用户账户设置为 signer
- [ ] 指令数据格式正确（小端序）

---

**遇到问题？**

1. 检查账户顺序是否与本文档完全匹配
2. 验证 PDA 计算
3. 确认所有账户都已创建和初始化
4. 查看 FINAL_FIX_SUMMARY.md 了解常见问题
