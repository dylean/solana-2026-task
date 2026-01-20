/**
 * take.rs - 接受托管指令
 * 
 * 功能：
 * 1. 验证接受者提供正确的代币 B
 * 2. 将接受者的代币 B 转给创建者
 * 3. 将金库中的代币 A 转给接受者
 * 4. 关闭金库和托管账户
 * 
 * Discriminator: 1
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::errors::EscrowError;
use crate::state::Escrow;

/**
 * Take 账户上下文
 * 
 * 定义接受托管所需的所有账户
 */
#[derive(Accounts)]
pub struct Take<'info> {
    /**
     * taker - 接受者账户
     * 
     * 约束：
     * - mut：需要签署并支付费用
     * - Signer：必须签署交易
     */
    #[account(mut)]
    pub taker: Signer<'info>,

    /**
     * maker - 创建者账户
     * 
     * 约束：
     * - mut：接收代币 B
     */
    #[account(mut)]
    pub maker: SystemAccount<'info>,

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
     * taker_ata_a - 接受者的代币 A 账户
     * 
     * 约束：
     * - mut：接收代币 A
     * - constraint：验证 mint 和 owner
     */
    #[account(
        mut,
        constraint = taker_ata_a.mint == escrow.mint_a @ EscrowError::InvalidMint,
        constraint = taker_ata_a.owner == taker.key() @ EscrowError::InvalidOwner,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,

    /**
     * taker_ata_b - 接受者的代币 B 账户
     * 
     * 约束：
     * - mut：转出代币 B
     * - constraint：验证 mint 和 owner
     */
    #[account(
        mut,
        constraint = taker_ata_b.mint == escrow.mint_b @ EscrowError::InvalidMint,
        constraint = taker_ata_b.owner == taker.key() @ EscrowError::InvalidOwner,
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    /**
     * maker_ata_b - 创建者的代币 B 账户
     * 
     * 约束：
     * - mut：接收代币 B
     * - constraint：验证 mint 和 owner
     */
    #[account(
        mut,
        constraint = maker_ata_b.mint == escrow.mint_b @ EscrowError::InvalidMint,
        constraint = maker_ata_b.owner == maker.key() @ EscrowError::InvalidOwner,
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,

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
    /// CHECK: Token program account
    pub token_program: AccountInfo<'info>,
}

/**
 * take 指令的处理函数
 * 
 * 步骤：
 * 1. 转移代币 B：taker -> maker
 * 2. 转移代币 A：vault -> taker（使用 PDA 签名）
 * 3. 关闭 vault（使用 PDA 签名）
 * 4. 关闭 escrow（通过约束自动处理）
 */
pub fn handler(ctx: Context<Take>) -> Result<()> {
    // 准备 escrow PDA 签名种子
    let seed_bytes = ctx.accounts.escrow.seed.to_le_bytes();
    let escrow_seeds = &[
        b"escrow",
        ctx.accounts.maker.key.as_ref(),
        seed_bytes.as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    let signer_seeds = &[&escrow_seeds[..]];

    // 1. 转移代币 B 从 taker 到 maker
    let cpi_accounts = Transfer {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, ctx.accounts.escrow.receive)?;

    // 2. 转移代币 A 从 vault 到 taker（使用 PDA 签名）
    let vault_amount = ctx.accounts.vault.amount;
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, vault_amount)?;

    // 3. 关闭 vault（使用 PDA 签名）
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

    msg!("托管交易完成！Taker 收到代币 A，Maker 收到代币 B");

    Ok(())
}
