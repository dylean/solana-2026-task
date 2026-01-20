/**
 * Anchor 托管程序测试文件
 * 
 * 测试所有托管功能：make、take、refund
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorEscrow } from "../target/types/blueshift_anchor_escrow";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";

describe("blueshift_anchor_escrow", () => {
  // 配置 Anchor 提供者
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 获取程序实例
  const program = anchor.workspace
    .BlueshiftAnchorEscrow as Program<BlueshiftAnchorEscrow>;

  // 测试账户
  const maker = provider.wallet as anchor.Wallet;
  const taker = anchor.web3.Keypair.generate();

  // 代币相关
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerAtaA: anchor.web3.PublicKey;
  let makerAtaB: anchor.web3.PublicKey;
  let takerAtaA: anchor.web3.PublicKey;
  let takerAtaB: anchor.web3.PublicKey;

  // 托管相关
  const seed = new anchor.BN(Math.floor(Math.random() * 1000000));
  let escrowPDA: anchor.web3.PublicKey;
  let vaultPDA: anchor.web3.PublicKey;

  before(async () => {
    console.log("\n========================================");
    console.log("🚀 开始测试准备");
    console.log("========================================\n");

    // 给 taker 空投 SOL
    console.log("1. 为接受者空投 SOL...");
    const airdropSig = await provider.connection.requestAirdrop(
      taker.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
    console.log("   ✓ 空投完成");

    // 创建代币 A
    console.log("\n2. 创建代币 A (Token A)...");
    mintA = await createMint(
      provider.connection,
      maker.payer,
      maker.publicKey,
      null,
      6
    );
    console.log(`   ✓ 代币 A: ${mintA.toBase58()}`);

    // 创建代币 B
    console.log("\n3. 创建代币 B (Token B)...");
    mintB = await createMint(
      provider.connection,
      maker.payer,
      maker.publicKey,
      null,
      6
    );
    console.log(`   ✓ 代币 B: ${mintB.toBase58()}`);

    // 创建 maker 的代币账户
    console.log("\n4. 创建创建者的代币账户...");
    const makerAtaAInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker.payer,
      mintA,
      maker.publicKey
    );
    makerAtaA = makerAtaAInfo.address;
    console.log(`   ✓ 创建者代币 A 账户: ${makerAtaA.toBase58()}`);

    // 创建 taker 的代币账户
    console.log("\n5. 创建接受者的代币账户...");
    const takerAtaBInfo = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker.payer,
      mintB,
      taker.publicKey
    );
    takerAtaB = takerAtaBInfo.address;
    console.log(`   ✓ 接受者代币 B 账户: ${takerAtaB.toBase58()}`);

    // 铸造代币 A 给 maker
    console.log("\n6. 铸造代币 A 给创建者...");
    await mintTo(
      provider.connection,
      maker.payer,
      mintA,
      makerAtaA,
      maker.publicKey,
      1000 * 10 ** 6 // 1000 tokens
    );
    console.log("   ✓ 铸造了 1000 代币 A");

    // 铸造代币 B 给 taker
    console.log("\n7. 铸造代币 B 给接受者...");
    await mintTo(
      provider.connection,
      maker.payer,
      mintB,
      takerAtaB,
      maker.publicKey,
      500 * 10 ** 6 // 500 tokens
    );
    console.log("   ✓ 铸造了 500 代币 B");

    // 计算 PDA 地址
    console.log("\n8. 计算 PDA 地址...");
    [escrowPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    console.log(`   ✓ Escrow PDA: ${escrowPDA.toBase58()}`);

    // 计算 vault 地址
    const vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker.payer,
      mintA,
      escrowPDA,
      true
    );
    vaultPDA = vaultAta.address;
    console.log(`   ✓ Vault PDA: ${vaultPDA.toBase58()}`);

    console.log("\n========================================");
    console.log("✅ 测试准备完成");
    console.log("========================================\n");
  });

  it("应该成功创建托管", async () => {
    console.log("\n📦 测试: 创建托管");
    
    const depositAmount = new anchor.BN(100 * 10 ** 6); // 100 tokens
    const receiveAmount = new anchor.BN(50 * 10 ** 6);  // 50 tokens

    const tx = await program.methods
      .make(seed, receiveAmount, depositAmount)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA,
        vault: vaultPDA,
        escrow: escrowPDA,
      })
      .rpc();

    console.log(`   ✓ 交易签名: ${tx}`);

    // 验证 escrow 账户
    const escrowAccount = await program.account.escrow.fetch(escrowPDA);
    assert.equal(
      escrowAccount.maker.toBase58(),
      maker.publicKey.toBase58(),
      "创建者地址应该匹配"
    );
    assert.equal(
      escrowAccount.receive.toNumber(),
      receiveAmount.toNumber(),
      "期望接收数量应该匹配"
    );

    // 验证金库余额
    const vaultAccount = await getAccount(provider.connection, vaultPDA);
    assert.equal(
      vaultAccount.amount.toString(),
      depositAmount.toString(),
      "金库余额应该等于存款金额"
    );

    console.log("   ✓ 托管创建成功");
  });

  it("应该成功接受托管", async () => {
    console.log("\n🤝 测试: 接受托管");

    // 获取 taker 的代币 A 账户地址
    takerAtaA = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        taker,
        mintA,
        taker.publicKey
      )
    ).address;

    // 获取 maker 的代币 B 账户地址
    makerAtaB = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        maker.payer,
        mintB,
        maker.publicKey
      )
    ).address;

    const tx = await program.methods
      .take()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        takerAtaA: takerAtaA,
        takerAtaB: takerAtaB,
        makerAtaB: makerAtaB,
        escrow: escrowPDA,
        vault: vaultPDA,
      })
      .signers([taker])
      .rpc();

    console.log(`   ✓ 交易签名: ${tx}`);

    // 验证 taker 收到了代币 A
    const takerAtaAAccount = await getAccount(
      provider.connection,
      takerAtaA
    );
    assert.isAbove(
      Number(takerAtaAAccount.amount),
      0,
      "接受者应该收到代币 A"
    );

    // 验证 maker 收到了代币 B
    const makerAtaBAccount = await getAccount(
      provider.connection,
      makerAtaB
    );
    assert.isAbove(
      Number(makerAtaBAccount.amount),
      0,
      "创建者应该收到代币 B"
    );

    console.log("   ✓ 托管接受成功");
  });

  it("应该成功退款（新托管）", async () => {
    console.log("\n💰 测试: 退款");

    // 创建新的托管用于测试退款
    const newSeed = new anchor.BN(Math.floor(Math.random() * 1000000));
    const [newEscrowPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        newSeed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const newVaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker.payer,
      mintA,
      newEscrowPDA,
      true
    );
    const newVaultPDA = newVaultAta.address;

    const depositAmount = new anchor.BN(50 * 10 ** 6);
    const receiveAmount = new anchor.BN(25 * 10 ** 6);

    // 创建托管
    await program.methods
      .make(newSeed, receiveAmount, depositAmount)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA,
        vault: newVaultPDA,
        escrow: newEscrowPDA,
      })
      .rpc();

    console.log("   ✓ 新托管创建成功");

    // 获取退款前的余额
    const makerBalanceBefore = await getAccount(
      provider.connection,
      makerAtaA
    );

    // 执行退款
    const tx = await program.methods
      .refund()
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        makerAtaA: makerAtaA,
        escrow: newEscrowPDA,
        vault: newVaultPDA,
      })
      .rpc();

    console.log(`   ✓ 退款交易签名: ${tx}`);

    // 验证余额增加
    const makerBalanceAfter = await getAccount(
      provider.connection,
      makerAtaA
    );
    assert.isAbove(
      Number(makerBalanceAfter.amount),
      Number(makerBalanceBefore.amount),
      "创建者余额应该增加"
    );

    console.log("   ✓ 退款成功");
  });
});
