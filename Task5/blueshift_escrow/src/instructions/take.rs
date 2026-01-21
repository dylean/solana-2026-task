/**
 * take.rs - 接受托管指令
 * 
 * 功能：
 * 1. 关闭托管记录，将其租金 lamports 返还给创建者
 * 2. 将 Token A 从保管库转移到接受者，然后关闭保管库
 * 3. 将约定数量的 Token B 从接受者转移到创建者
 * 
 * Discriminator: 1
 */

use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_token::instructions::{Transfer, CloseAccount};
use crate::{ID, state::Escrow};
use super::helpers::{
    SignerAccount, MintInterface, AssociatedTokenAccount, ProgramAccount, TokenAccount,
};

/// Take 账户结构
pub struct TakeAccounts<'a> {
    pub taker: &'a AccountView,
    pub maker: &'a AccountView,
    pub escrow: &'a AccountView,
    pub mint_a: &'a AccountView,
    pub mint_b: &'a AccountView,
    pub vault: &'a AccountView,
    pub taker_ata_a: &'a AccountView,
    pub taker_ata_b: &'a AccountView,
    pub maker_ata_b: &'a AccountView,
    pub system_program: &'a AccountView,
    pub token_program: &'a AccountView,
}

impl<'a> TakeAccounts<'a> {
    /// 从账户数组解析
    pub fn try_from_accounts(accounts: &'a [AccountView]) -> Result<Self, ProgramError> {
        if accounts.len() < 11 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let taker = &accounts[0];
        let maker = &accounts[1];
        let escrow = &accounts[2];
        let mint_a = &accounts[3];
        let mint_b = &accounts[4];
        let vault = &accounts[5];
        let taker_ata_a = &accounts[6];
        let taker_ata_b = &accounts[7];
        let maker_ata_b = &accounts[8];
        let system_program = &accounts[9];
        let token_program = &accounts[10];

        // 基本账户检查
        SignerAccount::check(taker)?;
        ProgramAccount::check(escrow)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(taker_ata_b, taker, mint_b, token_program)?;
        AssociatedTokenAccount::check(vault, escrow, mint_a, token_program)?;

        Ok(Self {
            taker,
            maker,
            escrow,
            mint_a,
            mint_b,
            vault,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            system_program,
            token_program,
        })
    }
}

/// Take 指令结构
pub struct Take<'a> {
    pub accounts: TakeAccounts<'a>,
}

impl<'a> Take<'a> {
    /// 从账户创建 Take 指令
    pub fn try_from_accounts(accounts: &'a [AccountView]) -> Result<Self, ProgramError> {
        let accounts = TakeAccounts::try_from_accounts(accounts)?;

        // 初始化必要的 ATA（如果不存在）
        AssociatedTokenAccount::init_if_needed(
            accounts.taker_ata_a,
            accounts.mint_a,
            accounts.taker,
            accounts.taker,
            accounts.system_program,
            accounts.token_program,
        )?;

        AssociatedTokenAccount::init_if_needed(
            accounts.maker_ata_b,
            accounts.mint_b,
            accounts.taker,
            accounts.maker,
            accounts.system_program,
            accounts.token_program,
        )?;

        Ok(Self { accounts })
    }

    /// 处理指令
    pub fn process(&mut self) -> ProgramResult {
        // 读取 escrow 状态
        let data = self.accounts.escrow.try_borrow()?;
        let escrow = Escrow::load(&data)?;

        // 验证 escrow PDA
        let escrow_key = Address::create_program_address(
            &[
                b"escrow",
                self.accounts.maker.address().as_ref(),
                &escrow.seed.to_le_bytes(),
                &escrow.bump,
            ],
            &ID,
        )?;

        if &escrow_key != self.accounts.escrow.address() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // 创建 PDA 签名种子
        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = escrow.bump;
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(self.accounts.maker.address().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        let signer = Signer::from(&escrow_seeds);

        // 获取 vault 余额
        let amount = TokenAccount::from_account_info(self.accounts.vault)?.amount()?;

        // 保存 receive 值，然后释放 data
        let receive_amount = escrow.receive;
        drop(data);

        // 1. 从 Vault 转移到 Taker (使用 PDA 签名)
        Transfer {
            from: self.accounts.vault,
            to: self.accounts.taker_ata_a,
            authority: self.accounts.escrow,
            amount,
        }.invoke_signed(&[signer.clone()])?;

        // 2. 关闭 Vault (使用 PDA 签名)
        CloseAccount {
            account: self.accounts.vault,
            destination: self.accounts.maker,
            authority: self.accounts.escrow,
        }.invoke_signed(&[signer.clone()])?;

        // 3. 从 Taker 转移到 Maker
        Transfer {
            from: self.accounts.taker_ata_b,
            to: self.accounts.maker_ata_b,
            authority: self.accounts.taker,
            amount: receive_amount,
        }.invoke()?;

        // 4. 关闭 Escrow (将租金返还给 maker，不是 taker！)
        ProgramAccount::close(self.accounts.escrow, self.accounts.maker)?;

        Ok(())
    }
}

/// take 指令入口点
pub fn take(accounts: &[AccountView]) -> ProgramResult {
    let mut take_ix = Take::try_from_accounts(accounts)?;
    take_ix.process()
}
