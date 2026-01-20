/**
 * refund.rs - 退款指令
 * 
 * 功能：
 * 1. 验证创建者身份
 * 2. 将金库中的代币 A 退还给创建者
 * 3. 关闭金库和托管账户
 * 
 * Discriminator: 2
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::errors::EscrowError;
use crate::state::Escrow;

/**
 * Refund 账户上下文
 * 
 * 定义退款所需的所有账户
 */
#[derive(Accounts)]
pub struct Refund<'info> {
    /**
     * maker - 创建者账户
     * 
     * 约束：
     * - mut：接收退款和租金
     * - Signer：必须签署交易
     */
    #[account(mut)]
    pub maker: Signer<'info>,

    /**
     * escrow - 托管账户（PDA）
     * 
     * 约束：
     * - mut：需要关闭
     * - close = maker：关闭后租金返还给创建者
     * - seeds：验证 PDA
     * - has_one：验证创建者
     */
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
    )]
    pub escrow: Account<'info, Escrow>,

    /**
     * maker_ata_a - 创建者的代币 A 账户
     * 
     * 约束：
     * - mut：接收退款
     * - constraint：验证 mint 和 owner
     */
    #[account(
        mut,
        constraint = maker_ata_a.mint == escrow.mint_a @ EscrowError::InvalidMint,
        constraint = maker_ata_a.owner == maker.key() @ EscrowError::InvalidOwner,
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    /**
     * vault - 金库代币账户
     * 
     * 约束：
     * - mut：转出代币并关闭
     * - seeds：验证 PDA
     */
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    /**
     * token_program - SPL Token 程序
     */
    pub token_program: Program<'info, Token>,
}

/**
 * refund 指令的处理函数
 * 
 * 步骤：
 * 1. 转移代币 A：vault -> maker（使用 PDA 签名）
 * 2. 关闭 vault（使用 PDA 签名）
 * 3. 关闭 escrow（通过约束自动处理）
 */
pub fn handler(ctx: Context<Refund>) -> Result<()> {
    // 准备 escrow PDA 签名种子
    let seed_bytes = ctx.accounts.escrow.seed.to_le_bytes();
    let escrow_seeds = &[
        b"escrow",
        ctx.accounts.maker.key.as_ref(),
        seed_bytes.as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    let signer_seeds = &[&escrow_seeds[..]];

    // 1. 转移所有代币 A 从 vault 到 maker
    let vault_amount = ctx.accounts.vault.amount;
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.maker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, vault_amount)?;

    // 2. 关闭 vault
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::close_account(cpi_ctx)?;

    msg!("托管已退款！Maker 收回代币 A");

    Ok(())
}
