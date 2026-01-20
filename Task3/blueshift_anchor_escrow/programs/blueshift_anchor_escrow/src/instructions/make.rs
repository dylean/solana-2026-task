/**
 * make.rs - 创建托管指令
 * 
 * 功能：
 * 1. 初始化托管记录并存储所有条款
 * 2. 创建金库（escrow 拥有的 mint_a 的 ATA）
 * 3. 使用 CPI 调用 SPL-Token 程序，将创建者的代币 A 转移到金库
 * 
 * Discriminator: 0
 */

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer as SystemTransfer};

use crate::errors::EscrowError;
use crate::state::Escrow;

// SPL Token 程序 ID
declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/**
 * Make 账户上下文
 * 
 * 定义创建托管所需的所有账户
 */
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    /**
     * maker - 创建者账户
     * 
     * 约束：
     * - mut：需要支付租金和代币转账
     * - Signer：必须签署交易
     */
    #[account(mut)]
    pub maker: Signer<'info>,

    /**
     * escrow - 托管账户（PDA）
     * 
     * 约束：
     * - init：初始化新账户
     * - payer = maker：由创建者支付租金
     * - space：账户所需空间（数据 + discriminator）
     * - seeds：PDA 派生种子
     * - bump：自动计算 bump seed
     */
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    /**
     * mint_a - 代币 A 的铸币账户（只需验证，不需要修改）
     */
    /// CHECK: 代币 mint 账户，通过 CPI 验证
    pub mint_a: AccountInfo<'info>,

    /**
     * mint_b - 代币 B 的铸币账户（只需验证，不需要修改）
     */
    /// CHECK: 代币 mint 账户，通过 CPI 验证
    pub mint_b: AccountInfo<'info>,

    /**
     * maker_ata_a - 创建者的代币 A 账户
     * 
     * 约束：
     * - mut：需要转出代币
     */
    #[account(mut)]
    /// CHECK: 代币账户，通过 CPI 验证
    pub maker_ata_a: AccountInfo<'info>,

    /**
     * vault - 金库代币账户（托管程序控制）
     * 
     * 约束：
     * - init：初始化金库
     * - payer = maker：由创建者支付
     * - seeds：PDA 派生
     */
    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        space = 165, // Token Account 固定大小
    )]
    /// CHECK: 金库账户，由程序控制
    pub vault: AccountInfo<'info>,

    /**
     * token_program - SPL Token 程序
     */
    /// CHECK: SPL Token 程序
    pub token_program: AccountInfo<'info>,

    /**
     * system_program - 系统程序
     */
    pub system_program: Program<'info, System>,
}

/**
 * make 指令的处理函数
 * 
 * 步骤：
 * 1. 填充 escrow 状态
 * 2. 初始化 vault 为 Token Account
 * 3. 从 maker_ata_a 转移代币到 vault
 */
pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    // 验证金额有效
    require!(amount > 0, EscrowError::InvalidAmount);
    require!(receive > 0, EscrowError::InvalidAmount);

    // 填充 escrow 状态
    let escrow = &mut ctx.accounts.escrow;
    escrow.seed = seed;
    escrow.maker = ctx.accounts.maker.key();
    escrow.mint_a = ctx.accounts.mint_a.key();
    escrow.mint_b = ctx.accounts.mint_b.key();
    escrow.receive = receive;
    escrow.bump = ctx.bumps.escrow;

    // 初始化 vault 为 Token Account
    // 使用 CPI 调用 spl_token::initialize_account
    let cpi_accounts = spl_token::instruction::initialize_account(
        ctx.accounts.token_program.key,
        ctx.accounts.vault.key,
        ctx.accounts.mint_a.key,
        &ctx.accounts.escrow.key(),
    )?;
    
    anchor_lang::solana_program::program::invoke(
        &cpi_accounts,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.mint_a.to_account_info(),
            ctx.accounts.escrow.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    // 转移代币从 maker 到 vault
    let transfer_ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        ctx.accounts.maker_ata_a.key,
        ctx.accounts.vault.key,
        ctx.accounts.maker.key,
        &[],
        amount,
    )?;

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.maker_ata_a.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.maker.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    msg!("托管创建成功！Seed: {}, Amount: {}, Receive: {}", seed, amount, receive);
    
    Ok(())
}
