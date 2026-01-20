/**
 * take.rs - 接受托管指令（简化版本）
 */

use anchor_lang::prelude::*;
use crate::errors::EscrowError;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    /// CHECK: Maker 账户
    pub maker: AccountInfo<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: Mint A
    pub mint_a: AccountInfo<'info>,

    /// CHECK: Mint B  
    pub mint_b: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Maker ATA for B
    pub maker_ata_b: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Taker ATA for A
    pub taker_ata_a: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Taker ATA for B
    pub taker_ata_b: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault
    pub vault: AccountInfo<'info>,

    /// CHECK: Token program
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    let escrow_key = ctx.accounts.escrow.key();
    let escrow_seeds = &[
        b"escrow",
        ctx.accounts.maker.key.as_ref(),
        ctx.accounts.escrow.seed.to_le_bytes().as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    let signer_seeds = &[&escrow_seeds[..]];

    // 1. Transfer B from taker to maker
    let transfer_b_ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        ctx.accounts.taker_ata_b.key,
        ctx.accounts.maker_ata_b.key,
        ctx.accounts.taker.key,
        &[],
        ctx.accounts.escrow.receive,
    )?;

    anchor_lang::solana_program::program::invoke(
        &transfer_b_ix,
        &[
            ctx.accounts.taker_ata_b.to_account_info(),
            ctx.accounts.maker_ata_b.to_account_info(),
            ctx.accounts.taker.to_account_info(),
        ],
    )?;

    // 2. Transfer A from vault to taker (with PDA signature)
    let vault_balance = ctx.accounts.vault.lamports();
    let transfer_a_ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        ctx.accounts.vault.key,
        ctx.accounts.taker_ata_a.key,
        &escrow_key,
        &[],
        vault_balance,
    )?;

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_a_ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.taker_ata_a.to_account_info(),
            ctx.accounts.escrow.to_account_info(),
        ],
        signer_seeds,
    )?;

    // 3. Close vault
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

    msg!("托管交换完成！");
    Ok(())
}
