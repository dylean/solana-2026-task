/**
 * make.rs - 创建托管指令
 * 
 * 功能：
 * 1. 初始化托管记录并存储所有交易条款
 * 2. 创建金库（escrow 拥有的 mint_a 的 ATA）
 * 3. 使用 CPI 调用 SPL-Token 程序，将创建者的 Token A 转移到金库
 * 
 * Discriminator: 0
 */

use pinocchio::{
    cpi::Seed,
    error::ProgramError,
    AccountView,
    ProgramResult,
    Address,
};
use pinocchio_token::instructions::Transfer;
use core::mem::size_of;
use crate::{ID, state::Escrow};
use super::helpers::{SignerAccount, MintInterface, AssociatedTokenAccount, ProgramAccount};

/// Make 指令数据
pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

impl MakeInstructionData {
    /// 从字节数组解析指令数据
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

        // 验证数据
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}

/// Make 账户结构
pub struct MakeAccounts<'a> {
    pub maker: &'a AccountView,
    pub escrow: &'a AccountView,
    pub mint_a: &'a AccountView,
    pub mint_b: &'a AccountView,
    pub maker_ata_a: &'a AccountView,
    pub vault: &'a AccountView,
    pub system_program: &'a AccountView,
    pub token_program: &'a AccountView,
}

impl<'a> MakeAccounts<'a> {
    /// 从账户数组解析
    pub fn try_from_accounts(accounts: &'a [AccountView]) -> Result<Self, ProgramError> {
        if accounts.len() < 8 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let maker = &accounts[0];
        let escrow = &accounts[1];
        let mint_a = &accounts[2];
        let mint_b = &accounts[3];
        let maker_ata_a = &accounts[4];
        let vault = &accounts[5];
        let system_program = &accounts[6];
        let token_program = &accounts[7];

        // 基本账户检查
        SignerAccount::check(maker)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(maker_ata_a, maker, mint_a, token_program)?;

        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}

/// Make 指令结构
pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

impl<'a> Make<'a> {
    /// 从数据和账户创建 Make 指令
    pub fn try_from_parts(
        data: &[u8],
        accounts: &'a [AccountView],
    ) -> Result<Self, ProgramError> {
        let accounts = MakeAccounts::try_from_accounts(accounts)?;
        let instruction_data = MakeInstructionData::try_from_bytes(data)?;

        // 计算 PDA bump
        let seed_bytes = instruction_data.seed.to_le_bytes();
        let (_, bump) = Address::find_program_address(
            &[b"escrow", accounts.maker.address().as_ref(), &seed_bytes],
            &ID,
        );

        // 初始化 escrow 账户
        let bump_binding = [bump];
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(accounts.maker.address().as_ref()),
            Seed::from(&seed_bytes),
            Seed::from(&bump_binding),
        ];

        ProgramAccount::init::<Escrow>(
            accounts.maker,
            accounts.escrow,
            &escrow_seeds,
            Escrow::LEN,
        )?;

        // 初始化 vault (ATA)
        AssociatedTokenAccount::init(
            accounts.vault,
            accounts.mint_a,
            accounts.maker,
            accounts.escrow,
            accounts.system_program,
            accounts.token_program,
        )?;

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }

    /// 处理指令
    pub fn process(&mut self) -> ProgramResult {
        // 填充 escrow 账户
        let mut data = self.accounts.escrow.try_borrow_mut()?;
        let escrow = Escrow::load_mut(&mut data)?;

        escrow.set_inner(
            self.instruction_data.seed,
            self.accounts.maker.address().clone(),
            self.accounts.mint_a.address().clone(),
            self.accounts.mint_b.address().clone(),
            self.instruction_data.receive,
            [self.bump],
        );

        drop(data);

        // 转移代币到 vault
        Transfer {
            from: self.accounts.maker_ata_a,
            to: self.accounts.vault,
            authority: self.accounts.maker,
            amount: self.instruction_data.amount,
        }.invoke()?;

        Ok(())
    }
}

/// make 指令入口点
pub fn make(data: &[u8], accounts: &[AccountView]) -> ProgramResult {
    let mut make_ix = Make::try_from_parts(data, accounts)?;
    make_ix.process()
}
