/**
 * refund.rs - 退款指令（简化版本）
 */

use anchor_lang::prelude::*;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    /// CHECK: Maker ATA for A
    pub maker_ata_a: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault
    pub vault: AccountInfo<'info>,

    /// CHECK: Token program
    pub token_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let escrow_key = ctx.accounts.escrow.key();
    let escrow_seeds = &[
        b"escrow",
        ctx.accounts.maker.key.as_ref(),
        ctx.accounts.escrow.seed.to_le_bytes().as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    let signer_seeds = &[&escrow_seeds[..]];

    // 1. Transfer all tokens from vault to maker
    let vault_balance = ctx.accounts.vault.lamports();
    let transfer_ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        ctx.accounts.vault.key,
        ctx.accounts.maker_ata_a.key,
        &escrow_key,
        &[],
        vault_balance,
    )?;

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.maker_ata_a.to_account_info(),
            ctx.accounts.escrow.to_account_info(),
        ],
        signer_seeds,
    )?;

    // 2. Close vault
    let close_ix = spl_token::instruction::close_account(
        ctx.accounts.token_program.key,
        ctx.accounts.vault.key,
        ctx.accounts.maker.key,
        &escrow_key,
        &[],
    )?;

    anchor_lang::solana_program::program::invoke_signed(
        &close_ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.maker.to_account_info(),
            ctx.accounts.escrow.to_account_info(),
        ],
        signer_seeds,
    )?;

    msg!("托管已退款！");
    Ok(())
}
