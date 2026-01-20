铸造一个 SPL 代币
SPL 代币程序，特别是使用 TypeScript 铸造一些代币，应该是您作为 Solana 开发者旅程的起点。

如果您是 SPL 代币程序的新手，请先按照本课程学习！

## 任务要求

在本次挑战中，我们将实现四个简单的指令：

1. **创建一个铸币账户**：使用原始指令或 SDK 提供的抽象指令创建一个 Mint Account。

2. **初始化铸币**：使用原始指令或 SDK 提供的抽象指令初始化创建的 Mint Account。铸币应具有：
   - 6 位小数
   - 铸币权限（mint authority）设置为运行代码的钱包（feePayer）
   - 冻结权限（freeze authority）设置为运行代码的钱包（feePayer）

3. **创建一个关联代币账户**：使用原始指令或 SDK 提供的抽象指令创建并初始化一个 Associated Token Account。

4. **铸造 2100 万代币**：将 2100 万（21,000,000）新创建的代币铸造到新创建的 Associated Token Account 中。

注意：如果您不熟悉如何使用 TypeScript 操作 SPL-Token 程序，建议先阅读 SPL Token with Web3.js，以熟悉我们在本次挑战中需要的 SDK。

Blueshift 沙盒环境
本次挑战设计为在 Blueshift 沙盒环境 中完成。

这个零配置沙盒提供了完成挑战所需的一切。您可以访问内置编辑器，支持类型检查，预打包的库，一个钱包，以及一个隔离环境中的 RPC 节点。

准备开始编码了吗？点击下方按钮打开沙箱，开始你的挑战吧。

```js
/** 挑战：铸造一个 SPL 代币
 *
 * 在本挑战中，您将创建一个 SPL 代币！
 *
 * 目标：
 *   使用 Web3.js 和 SPL Token 库在单个交易中铸造一个 SPL 代币。
 *
 * 任务目标：
 *   1. 创建一个 SPL mint 账户。
 *   2. 使用 6 位小数初始化 mint，并将您的公钥（feePayer）设置为 mint 权限和冻结权限。
 *   3. 为您的公钥（feePayer）创建一个关联代币账户来持有铸造的代币。
 *   4. 铸造 21,000,000 个代币到您的关联代币账户。
 *   5. 签名并发送交易。
 */

// 导入 Solana Web3.js 核心库
import {
  Keypair,                      // 密钥对工具类，用于生成和管理公私钥对
  Connection,                   // 用于连接 Solana RPC 节点
  sendAndConfirmTransaction,    // 发送交易并等待确认
  SystemProgram,                // Solana 系统程序，用于创建账户等基础操作
  Transaction,                  // 交易对象，用于构建和组合多个指令
} from "@solana/web3.js";

// 导入 SPL Token 相关的工具和常量
import {
  createAssociatedTokenAccountInstruction,  // 创建关联代币账户的指令
  createInitializeMint2Instruction,         // 初始化 mint 账户的指令（第二版，更简洁）
  createMintToInstruction,                   // 铸造代币的指令
  createMintToCheckedInstruction,            // 带验证的铸造代币指令（推荐使用）
  MINT_SIZE,                                 // mint 账户所需的空间大小（82 字节）
  getMinimumBalanceForRentExemptMint,       // 获取 mint 账户免租金所需的最小余额
  TOKEN_PROGRAM_ID,                          // SPL Token 程序的 ID
  getAssociatedTokenAddressSync,             // 同步获取关联代币账户地址
  ASSOCIATED_TOKEN_PROGRAM_ID                // 关联代币账户程序的 ID
} from "@solana/spl-token";

// 导入 base58 编码工具（用于解码密钥）
import bs58 from "bs58";

// 从环境变量中导入支付者（payer）的密钥对
// feePayer 是支付交易费用和作为 mint authority 的账户
const feePayer = Keypair.fromSecretKey(
  // ⚠️ 不安全的密钥，仅用于此挑战，请勿在实际生产环境中使用
  bs58.decode(process.env.SECRET)
);

// 创建与 Solana RPC 节点的连接
// "confirmed" 表示等待交易被确认（而非最终确定）
const connection = new Connection(
  process.env.RPC_ENDPOINT,
  "confirmed"
);

// TypeScript 代码入口点（主函数）
async function main() {
  try {

    // ========================================
    // 步骤 1: 生成 mint 账户的密钥对
    // ========================================
    // mint 账户是代币的"铸造厂"，控制代币的供应
    const mint = Keypair.generate();

    // 获取 mint 账户需要的最小租金（Solana 的账户需要支付租金或达到免租金余额）
    const mintRent = await getMinimumBalanceForRentExemptMint(connection);

    // ========================================
    // 步骤 2: 创建 mint 账户
    // ========================================
    // 使用 SystemProgram 创建一个新账户，分配给 TOKEN_PROGRAM_ID 程序
    const createAccountIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,       // 支付账户创建费用的账户
      newAccountPubkey: mint.publicKey,     // 新创建的 mint 账户的公钥
      space: MINT_SIZE,                     // mint 账户需要的存储空间（82 字节）
      lamports: mintRent,                   // 支付的租金（lamports 是 SOL 的最小单位）
      programId: TOKEN_PROGRAM_ID,          // 将此账户分配给 SPL Token 程序
    });

    // ========================================
    // 步骤 3: 初始化 mint 账户
    // ========================================
    // 设置 mint 的配置：小数位数、权限等
    const initializeMintIx = createInitializeMint2Instruction(
      mint.publicKey,        // mint 账户地址
      6,                     // 小数位数（6 位意味着 1 个代币 = 1,000,000 个最小单位）
      feePayer.publicKey,    // mint authority（铸币权限）：有权铸造新代币的账户
      feePayer.publicKey     // freeze authority（冻结权限）：有权冻结代币账户的账户
    );

    // ========================================
    // 步骤 4: 创建关联代币账户（Associated Token Account, ATA）
    // ========================================
    // ATA 是用于持有特定 mint 代币的账户，地址由 owner 和 mint 确定性派生
    
    // 计算关联代币账户的地址（这是一个确定性的 PDA 地址）
    const associatedTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,        // 代币的 mint 地址
      feePayer.publicKey     // 代币账户的所有者
    );
    
    // 创建关联代币账户的指令
    const createAssociatedTokenAccountIx = createAssociatedTokenAccountInstruction(
      feePayer.publicKey,              // payer：支付账户创建费用的账户
      associatedTokenAccount,          // associatedToken：要创建的关联代币账户地址
      feePayer.publicKey,              // owner：代币账户的所有者
      mint.publicKey                   // mint：代币的 mint 地址
    );

    // ========================================
    // 步骤 5: 铸造代币
    // ========================================
    // 计算要铸造的代币数量（考虑小数位数）
    // 21,000,000 个代币 × 10^6（因为有 6 位小数）= 21,000,000,000,000 个最小单位
    const mintAmount = 21_000_000 * 10 ** 6;
    
    // 创建铸造代币的指令（使用 Checked 版本会验证小数位数，更安全）
    const mintToCheckedIx = createMintToCheckedInstruction(
      mint.publicKey,                  // mint：代币的 mint 账户
      associatedTokenAccount,          // destination：接收代币的目标账户
      feePayer.publicKey,              // authority：mint authority（必须是有铸币权限的账户）
      mintAmount,                      // amount：要铸造的代币数量（最小单位）
      6                                // decimals：小数位数（必须与 mint 的小数位数匹配）
    );

    // ========================================
    // 步骤 6: 构建并发送交易
    // ========================================
    
    // 获取最新的区块哈希（用于交易的有效性验证）
    const recentBlockhash = await connection.getLatestBlockhash();

    // 创建交易对象，并添加所有指令
    const transaction = new Transaction({
      feePayer: feePayer.publicKey,                      // 指定支付交易费用的账户
      blockhash: recentBlockhash.blockhash,              // 最新区块哈希
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight  // 交易有效的最后区块高度
    }).add(
        createAccountIx,                    // 指令 1: 创建 mint 账户
        initializeMintIx,                   // 指令 2: 初始化 mint
        createAssociatedTokenAccountIx,     // 指令 3: 创建关联代币账户
        mintToCheckedIx                     // 指令 4: 铸造代币
    );

    // 发送交易并等待确认
    // signers 数组包含所有需要签名的密钥对：
    // - feePayer：支付费用并作为 mint authority
    // - mint：新创建的 mint 账户需要用其私钥签名
    const transactionSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [feePayer, mint]
    );

    // 输出结果
    console.log("Mint Address:", mint.publicKey.toBase58());
    console.log("Transaction Signature:", transactionSignature);
  } catch (error) {
    console.error(`糟糕，出错了: ${error}`);
  }
}
```