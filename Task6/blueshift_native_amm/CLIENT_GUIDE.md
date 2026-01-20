# Task6 AMM 客户端调用指南

## 概述

本文档说明如何从 TypeScript/JavaScript 客户端正确调用 Task6 AMM 程序的各个指令。

## 程序 ID

```typescript
const PROGRAM_ID = new PublicKey("22222222222222222222222222222222222222222222");
```

## 指令鉴别器

| 指令 | 鉴别器 | 说明 |
|------|--------|------|
| Initialize | 0 | 初始化 AMM 池 |
| Deposit | 1 | 存入流动性 |
| Withdraw | 2 | 提取流动性 |
| Swap | 3 | 交换代币 |

## 1. Initialize 指令

### 功能
创建 Config 账户和 LP Token Mint 账户。

### 指令数据结构

```typescript
interface InitializeInstructionData {
  seed: bigint;              // 8 字节 - PDA 种子
  fee: number;               // 2 字节 - 费用（基点，0-10000）
  mint_x: PublicKey;         // 32 字节 - 代币 X 的 Mint
  mint_y: PublicKey;         // 32 字节 - 代币 Y 的 Mint
  config_bump: number;       // 1 字节 - Config PDA bump（程序会重新计算）
  lp_bump: number;           // 1 字节 - LP Mint PDA bump（程序会重新计算）
  authority?: PublicKey;     // 32 字节（可选）- 管理权限
}
```

### 账户列表

```typescript
const accounts = [
  { pubkey: initializer.publicKey, isSigner: true, isWritable: true },
  { pubkey: configPDA, isSigner: false, isWritable: true },
  { pubkey: mintLpPDA, isSigner: false, isWritable: true },
  { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
];
```

### 示例代码

```typescript
import { PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

async function initializeAMM(
  connection: Connection,
  payer: Keypair,
  mintX: PublicKey,
  mintY: PublicKey,
  fee: number = 30, // 0.3%
  authority?: PublicKey
) {
  // 1. 生成随机种子
  const seed = BigInt(Math.floor(Math.random() * 1000000));
  
  // 2. 计算 Config PDA
  const seedBuffer = Buffer.alloc(8);
  seedBuffer.writeBigUInt64LE(seed);
  
  const [configPDA, configBump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("config"),
      seedBuffer,
      mintX.toBuffer(),
      mintY.toBuffer(),
    ],
    PROGRAM_ID
  );
  
  // 3. 计算 LP Mint PDA
  const [mintLpPDA, lpBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint_lp"), configPDA.toBuffer()],
    PROGRAM_ID
  );
  
  // 4. 构造指令数据
  const instructionData = Buffer.alloc(
    1 +  // 鉴别器
    8 +  // seed
    2 +  // fee
    32 + // mint_x
    32 + // mint_y
    1 +  // config_bump
    1 +  // lp_bump
    (authority ? 32 : 0) // authority（可选）
  );
  
  let offset = 0;
  
  // 鉴别器
  instructionData.writeUInt8(0, offset);
  offset += 1;
  
  // seed
  instructionData.writeBigUInt64LE(seed, offset);
  offset += 8;
  
  // fee
  instructionData.writeUInt16LE(fee, offset);
  offset += 2;
  
  // mint_x
  mintX.toBuffer().copy(instructionData, offset);
  offset += 32;
  
  // mint_y
  mintY.toBuffer().copy(instructionData, offset);
  offset += 32;
  
  // config_bump（程序会重新计算，传入计算的值或 0）
  instructionData.writeUInt8(configBump, offset);
  offset += 1;
  
  // lp_bump（程序会重新计算，传入计算的值或 0）
  instructionData.writeUInt8(lpBump, offset);
  offset += 1;
  
  // authority（可选）
  if (authority) {
    authority.toBuffer().copy(instructionData, offset);
  }
  
  // 5. 构造交易
  const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: configPDA, isSigner: false, isWritable: true },
      { pubkey: mintLpPDA, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: instructionData,
  });
  
  const transaction = new Transaction().add(instruction);
  
  // 6. 发送交易
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payer]
  );
  
  console.log("Initialize AMM 成功！");
  console.log("签名:", signature);
  console.log("Config PDA:", configPDA.toBase58());
  console.log("LP Mint PDA:", mintLpPDA.toBase58());
  
  return { configPDA, mintLpPDA, signature };
}
```

## 2. Deposit 指令

### 功能
存入流动性，铸造 LP 代币。

### 指令数据结构

```typescript
interface DepositInstructionData {
  amount: bigint;      // 8 字节 - 期望的 LP 代币数量
  max_x: bigint;       // 8 字节 - 最大存入的 X 代币数量
  max_y: bigint;       // 8 字节 - 最大存入的 Y 代币数量
  expiration: bigint;  // 8 字节 - 过期时间戳
}
```

### 账户列表

```typescript
const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: true },
  { pubkey: mintLpPDA, isSigner: false, isWritable: true },
  { pubkey: vaultX, isSigner: false, isWritable: true },
  { pubkey: vaultY, isSigner: false, isWritable: true },
  { pubkey: userXAta, isSigner: false, isWritable: true },
  { pubkey: userYAta, isSigner: false, isWritable: true },
  { pubkey: userLpAta, isSigner: false, isWritable: true },
  { pubkey: configPDA, isSigner: false, isWritable: false },
  { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
];
```

### 示例代码

```typescript
async function depositLiquidity(
  connection: Connection,
  user: Keypair,
  configPDA: PublicKey,
  mintX: PublicKey,
  mintY: PublicKey,
  mintLp: PublicKey,
  amountX: bigint,
  amountY: bigint,
  minLpTokens: bigint
) {
  // 1. 计算 Vault PDAs
  const [vaultX] = PublicKey.findProgramAddressSync(
    [
      configPDA.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mintX.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
  
  const [vaultY] = PublicKey.findProgramAddressSync(
    [
      configPDA.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mintY.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
  
  // 2. 计算用户 ATAs
  const userXAta = getAssociatedTokenAddressSync(mintX, user.publicKey);
  const userYAta = getAssociatedTokenAddressSync(mintY, user.publicKey);
  const userLpAta = getAssociatedTokenAddressSync(mintLp, user.publicKey);
  
  // 3. 构造指令数据
  const instructionData = Buffer.alloc(1 + 8 + 8 + 8 + 8);
  let offset = 0;
  
  // 鉴别器
  instructionData.writeUInt8(1, offset);
  offset += 1;
  
  // amount (LP tokens)
  instructionData.writeBigUInt64LE(minLpTokens, offset);
  offset += 8;
  
  // max_x
  instructionData.writeBigUInt64LE(amountX, offset);
  offset += 8;
  
  // max_y
  instructionData.writeBigUInt64LE(amountY, offset);
  offset += 8;
  
  // expiration
  const expiration = BigInt(Math.floor(Date.now() / 1000) + 60); // 60秒后过期
  instructionData.writeBigUInt64LE(expiration, offset);
  
  // 4. 构造交易
  const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: user.publicKey, isSigner: true, isWritable: true },
      { pubkey: mintLp, isSigner: false, isWritable: true },
      { pubkey: vaultX, isSigner: false, isWritable: true },
      { pubkey: vaultY, isSigner: false, isWritable: true },
      { pubkey: userXAta, isSigner: false, isWritable: true },
      { pubkey: userYAta, isSigner: false, isWritable: true },
      { pubkey: userLpAta, isSigner: false, isWritable: true },
      { pubkey: configPDA, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: instructionData,
  });
  
  const transaction = new Transaction().add(instruction);
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [user]
  );
  
  console.log("Deposit 成功！签名:", signature);
  return signature;
}
```

## 3. Withdraw 指令

### 功能
销毁 LP 代币，提取流动性。

### 指令数据结构

```typescript
interface WithdrawInstructionData {
  amount: bigint;      // 8 字节 - 销毁的 LP 代币数量
  min_x: bigint;       // 8 字节 - 最小提取的 X 代币数量
  min_y: bigint;       // 8 字节 - 最小提取的 Y 代币数量
  expiration: bigint;  // 8 字节 - 过期时间戳
}
```

### 账户列表（与 Deposit 相同）

### 示例代码

```typescript
async function withdrawLiquidity(
  connection: Connection,
  user: Keypair,
  configPDA: PublicKey,
  mintX: PublicKey,
  mintY: PublicKey,
  mintLp: PublicKey,
  lpTokenAmount: bigint,
  minX: bigint,
  minY: bigint
) {
  // 账户计算与 Deposit 相同
  // ...
  
  // 构造指令数据
  const instructionData = Buffer.alloc(1 + 8 + 8 + 8 + 8);
  let offset = 0;
  
  // 鉴别器
  instructionData.writeUInt8(2, offset);
  offset += 1;
  
  // amount (LP tokens)
  instructionData.writeBigUInt64LE(lpTokenAmount, offset);
  offset += 8;
  
  // min_x
  instructionData.writeBigUInt64LE(minX, offset);
  offset += 8;
  
  // min_y
  instructionData.writeBigUInt64LE(minY, offset);
  offset += 8;
  
  // expiration
  const expiration = BigInt(Math.floor(Date.now() / 1000) + 60);
  instructionData.writeBigUInt64LE(expiration, offset);
  
  // 构造并发送交易
  // ...
}
```

## 4. Swap 指令

### 功能
交换 X ↔ Y 代币。

### 指令数据结构

```typescript
interface SwapInstructionData {
  is_x: boolean;       // 1 字节 - true: X→Y, false: Y→X
  amount: bigint;      // 8 字节 - 输入数量
  min: bigint;         // 8 字节 - 最小输出数量
  expiration: bigint;  // 8 字节 - 过期时间戳
}
```

### 账户列表

```typescript
const accounts = [
  { pubkey: user.publicKey, isSigner: true, isWritable: false },
  { pubkey: userXAta, isSigner: false, isWritable: true },
  { pubkey: userYAta, isSigner: false, isWritable: true },
  { pubkey: vaultX, isSigner: false, isWritable: true },
  { pubkey: vaultY, isSigner: false, isWritable: true },
  { pubkey: configPDA, isSigner: false, isWritable: false },
  { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
];
```

### 示例代码

```typescript
async function swapTokens(
  connection: Connection,
  user: Keypair,
  configPDA: PublicKey,
  mintX: PublicKey,
  mintY: PublicKey,
  isXToY: boolean,
  inputAmount: bigint,
  minOutputAmount: bigint
) {
  // 账户计算
  // ...
  
  // 构造指令数据
  const instructionData = Buffer.alloc(1 + 1 + 8 + 8 + 8);
  let offset = 0;
  
  // 鉴别器
  instructionData.writeUInt8(3, offset);
  offset += 1;
  
  // is_x
  instructionData.writeUInt8(isXToY ? 1 : 0, offset);
  offset += 1;
  
  // amount
  instructionData.writeBigUInt64LE(inputAmount, offset);
  offset += 8;
  
  // min
  instructionData.writeBigUInt64LE(minOutputAmount, offset);
  offset += 8;
  
  // expiration
  const expiration = BigInt(Math.floor(Date.now() / 1000) + 60);
  instructionData.writeBigUInt64LE(expiration, offset);
  
  // 构造并发送交易
  // ...
}
```

## 重要提示

### 1. PDA Bump
- **不要依赖客户端传入的 bump**
- 程序会重新计算正确的 bump
- 客户端可以传入任意值（推荐传入正确计算的值）

### 2. 账户顺序
- 账户顺序必须与程序期望的完全一致
- 错误的顺序会导致 `invalid account data` 错误

### 3. 指令数据格式
- 所有整数使用小端序（Little Endian）
- PublicKey 是 32 字节
- 布尔值用 0 或 1 表示

### 4. 错误处理
```typescript
try {
  const signature = await initializeAMM(...);
  console.log("成功！", signature);
} catch (error) {
  if (error.message.includes("invalid account data")) {
    console.error("账户数据无效，检查 PDA 计算或账户顺序");
  } else if (error.message.includes("requires an initialized account")) {
    console.error("账户未初始化，先调用 Initialize 指令");
  } else {
    console.error("未知错误:", error);
  }
}
```

## 测试建议

1. **先测试 Initialize**
   ```typescript
   const { configPDA, mintLpPDA } = await initializeAMM(...);
   ```

2. **验证账户创建**
   ```typescript
   const configAccount = await connection.getAccountInfo(configPDA);
   console.log("Config 账户大小:", configAccount.data.length);
   ```

3. **测试 Deposit**
   ```typescript
   await depositLiquidity(...);
   ```

4. **验证 LP 代币铸造**
   ```typescript
   const lpBalance = await connection.getTokenAccountBalance(userLpAta);
   console.log("LP 代币余额:", lpBalance.value.uiAmount);
   ```

## 常见问题

### Q1: 如何计算 Vault PDA？
A: Vault 是 Associated Token Account (ATA)，使用 `getAssociatedTokenAddressSync` 计算，owner 是 configPDA。

### Q2: 为什么需要传入 bump？
A: 虽然程序会重新计算，但保留这个字段是为了向后兼容。建议传入正确计算的值。

### Q3: 如何获取 Config 中的数据？
A: 读取 Config 账户数据并按照 `state.rs` 中的结构解析：
```typescript
const configData = await connection.getAccountInfo(configPDA);
// 解析字节数据
```

### Q4: 程序支持哪些代币？
A: 支持任何 SPL Token，只需在 Initialize 时传入对应的 Mint 地址。

## 完整示例

查看 `tests/` 目录中的完整测试示例，了解如何使用所有指令。
