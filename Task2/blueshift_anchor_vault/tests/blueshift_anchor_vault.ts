/**
 * Anchor 金库程序测试文件
 * 
 * 这个文件包含了金库程序的完整测试用例
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorVault } from "../target/types/blueshift_anchor_vault";
import { assert } from "chai";

describe("blueshift_anchor_vault", () => {
  // 配置 Anchor 提供者（连接到本地测试网络）
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 获取程序实例
  const program = anchor.workspace.BlueshiftAnchorVault as Program<BlueshiftAnchorVault>;
  
  // 获取钱包（测试用户）
  const wallet = provider.wallet as anchor.Wallet;

  // 派生金库 PDA 地址
  // 这个地址是确定性的，由程序 ID 和种子派生而来
  const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), wallet.publicKey.toBuffer()],
    program.programId
  );

  // 测试前的清理工作（如果金库有余额则先取出）
  before(async () => {
    try {
      const vaultAccount = await provider.connection.getAccountInfo(vaultPDA);
      if (vaultAccount && vaultAccount.lamports > 0) {
        await program.methods
          .withdraw()
          .accounts({
            signer: wallet.publicKey,
            vault: vaultPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        
        console.log("已清理之前的金库余额");
      }
    } catch (error) {
      // 金库可能不存在，忽略错误
      console.log("无需清理金库");
    }
  });

  /**
   * 测试 1: 存款功能
   * 
   * 验证用户可以成功将 SOL 存入金库
   */
  it("应该成功存款到金库", async () => {
    // 存款金额：1 SOL = 1,000,000,000 lamports
    const depositAmount = new anchor.BN(1_000_000_000);

    // 记录存款前的余额
    const signerBalanceBefore = await provider.connection.getBalance(wallet.publicKey);
    console.log("存款前签名者余额:", signerBalanceBefore / anchor.web3.LAMPORTS_PER_SOL, "SOL");

    // 执行存款交易
    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("存款交易签名:", tx);

    // 获取金库余额
    const vaultBalance = await provider.connection.getBalance(vaultPDA);
    console.log("存款后金库余额:", vaultBalance / anchor.web3.LAMPORTS_PER_SOL, "SOL");

    // 断言：金库余额应该等于存款金额
    assert.equal(
      vaultBalance,
      depositAmount.toNumber(),
      "金库余额应该等于存款金额"
    );

    // 验证签名者余额减少（存款金额 + 交易费用）
    const signerBalanceAfter = await provider.connection.getBalance(wallet.publicKey);
    console.log("存款后签名者余额:", signerBalanceAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    
    assert.isBelow(
      signerBalanceAfter,
      signerBalanceBefore - depositAmount.toNumber(),
      "签名者余额应该减少"
    );
  });

  /**
   * 测试 2: 重复存款应该失败
   * 
   * 验证不能向已有余额的金库再次存款
   */
  it("重复存款应该失败", async () => {
    const depositAmount = new anchor.BN(500_000_000); // 0.5 SOL

    try {
      await program.methods
        .deposit(depositAmount)
        .accounts({
          signer: wallet.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      // 如果没有抛出错误，测试失败
      assert.fail("重复存款应该抛出错误");
    } catch (error) {
      // 验证错误类型
      assert.include(
        error.toString(),
        "VaultAlreadyExists",
        "应该抛出 VaultAlreadyExists 错误"
      );
      console.log("✓ 重复存款被正确拒绝");
    }
  });

  /**
   * 测试 3: 存款金额过小应该失败
   * 
   * 验证存款金额必须大于免租金最低限额
   */
  it("存款金额过小应该失败", async () => {
    // 首先清空金库以便重新测试存款
    await program.methods
      .withdraw()
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 尝试存入一个很小的金额（小于免租金最低限额）
    const tinyAmount = new anchor.BN(100); // 100 lamports（远低于免租金限额）

    try {
      await program.methods
        .deposit(tinyAmount)
        .accounts({
          signer: wallet.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("小额存款应该抛出错误");
    } catch (error) {
      assert.include(
        error.toString(),
        "InvalidAmount",
        "应该抛出 InvalidAmount 错误"
      );
      console.log("✓ 小额存款被正确拒绝");
    }
  });

  /**
   * 测试 4: 取款功能
   * 
   * 验证用户可以成功从金库中取出所有 SOL
   */
  it("应该成功从金库取款", async () => {
    // 首先存入一些 SOL
    const depositAmount = new anchor.BN(2_000_000_000); // 2 SOL
    await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 记录取款前的余额
    const signerBalanceBefore = await provider.connection.getBalance(wallet.publicKey);
    const vaultBalanceBefore = await provider.connection.getBalance(vaultPDA);
    console.log("取款前签名者余额:", signerBalanceBefore / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("取款前金库余额:", vaultBalanceBefore / anchor.web3.LAMPORTS_PER_SOL, "SOL");

    // 执行取款交易
    const tx = await program.methods
      .withdraw()
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("取款交易签名:", tx);

    // 验证金库余额为 0
    const vaultBalanceAfter = await provider.connection.getBalance(vaultPDA);
    console.log("取款后金库余额:", vaultBalanceAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    
    assert.equal(
      vaultBalanceAfter,
      0,
      "金库余额应该为 0"
    );

    // 验证签名者余额增加（约等于之前的金库余额，减去交易费用）
    const signerBalanceAfter = await provider.connection.getBalance(wallet.publicKey);
    console.log("取款后签名者余额:", signerBalanceAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    
    assert.isAbove(
      signerBalanceAfter,
      signerBalanceBefore,
      "签名者余额应该增加"
    );
  });

  /**
   * 测试 5: 从空金库取款应该失败
   * 
   * 验证不能从空金库取款
   */
  it("从空金库取款应该失败", async () => {
    try {
      await program.methods
        .withdraw()
        .accounts({
          signer: wallet.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("从空金库取款应该抛出错误");
    } catch (error) {
      assert.include(
        error.toString(),
        "InvalidAmount",
        "应该抛出 InvalidAmount 错误"
      );
      console.log("✓ 空金库取款被正确拒绝");
    }
  });

  /**
   * 测试 6: 完整的存取循环
   * 
   * 验证可以多次进行存款-取款循环
   */
  it("应该支持多次存取循环", async () => {
    for (let i = 0; i < 3; i++) {
      console.log(`\n=== 第 ${i + 1} 轮存取 ===`);
      
      // 存款
      const depositAmount = new anchor.BN((i + 1) * 1_000_000_000); // 递增金额
      await program.methods
        .deposit(depositAmount)
        .accounts({
          signer: wallet.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      const vaultBalance = await provider.connection.getBalance(vaultPDA);
      console.log(`存款 ${(i + 1)} SOL，金库余额:`, vaultBalance / anchor.web3.LAMPORTS_PER_SOL, "SOL");
      
      // 取款
      await program.methods
        .withdraw()
        .accounts({
          signer: wallet.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      const vaultBalanceAfter = await provider.connection.getBalance(vaultPDA);
      console.log(`取款后金库余额:`, vaultBalanceAfter / anchor.web3.LAMPORTS_PER_SOL, "SOL");
      
      assert.equal(vaultBalanceAfter, 0, "每轮取款后金库应为空");
    }
    
    console.log("\n✓ 多次存取循环测试通过");
  });
});
