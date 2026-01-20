/**
 * make.rs - 创建托管指令
 * 
 * 功能：
 * 1. 初始化托管记录并存储所有条款
 * 2. 创建金库（escrow 拥有的 mint_a 的 Token Account）
 * 3. 使用 CPI 调用 SPL-Token 程序，将创建者的代币 A 转移到金库
 * 
 * Discriminator: 0
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

use crate::errors::EscrowError;
use crate::state::Escrow;

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
     * mint_a - 代币 A 的铸币账户
     */
    pub mint_a: Account<'info, Mint>,

    /**
     * mint_b - 代币 B 的铸币账户
     */
    pub mint_b: Account<'info, Mint>,

    /**
     * maker_ata_a - 创建者的代币 A 账户
     * 
     * 约束：
     * - mut：需要转出代币
     * - constraint: 验证是正确的 mint
     */
    #[account(
        mut,
        constraint = maker_ata_a.mint == mint_a.key() @ EscrowError::InvalidMint,
        constraint = maker_ata_a.owner == maker.key() @ EscrowError::InvalidOwner,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    /**
     * vault - 金库代币账户（托管程序控制）
     * 
     * 约束：
     * - init：初始化金库
     * - payer = maker：由创建者支付
     * - seeds：PDA 派生
     * - mint：指定代币类型
     * - authority：由 escrow PDA 控制
     */
    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = mint_a,
        token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    /**
     * token_program - SPL Token 程序
     */
    /// CHECK: Token program account
    pub token_program: AccountInfo<'info>,

    /**
     * system_program - 系统程序
     */
    /// CHECK: System program account
    pub system_program: AccountInfo<'info>,
}

/**
 * make 指令的处理函数
 * 
 * 步骤：
 * 1. 填充 escrow 状态
 * 2. vault 已通过 init 约束自动初始化
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

    // 转移代币从 maker 到 vault
    // 使用 Anchor CPI 帮助函数
    let cpi_accounts = Transfer {
        from: ctx.accounts.maker_ata_a.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, amount)?;

    msg!("托管创建成功！Seed: {}, Amount: {}, Receive: {}", seed, amount, receive);
    
    Ok(())
}
