/**
 * helpers.rs - Pinocchio 辅助函数
 * 
 * 提供账户验证和操作的辅助功能
 */

use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    AccountView,
    ProgramResult,
    Address,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_associated_token_account::instructions::Create;

/// 签名者账户辅助
pub struct SignerAccount;

impl SignerAccount {
    /// 验证账户是签名者
    pub fn check(account: &AccountView) -> ProgramResult {
        if !account.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

/// Mint 接口辅助
pub struct MintInterface;

impl MintInterface {
    /// 验证 Mint 账户（基本检查）
    pub fn check(account: &AccountView) -> ProgramResult {
        // 基本检查：账户数据长度应该是 82 字节（Mint 账户）
        if account.data_len() < 82 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

/// 关联代币账户辅助
pub struct AssociatedTokenAccount;

impl AssociatedTokenAccount {
    /// 验证 ATA
    pub fn check(
        ata: &AccountView,
        owner: &AccountView,
        mint: &AccountView,
        _token_program: &AccountView,
    ) -> ProgramResult {
        // 基本检查：账户数据长度应该是 165 字节（Token 账户）
        if ata.data_len() < 165 {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // 验证 owner (offset 32)
        let ata_data = unsafe { ata.borrow_unchecked() };
        let ata_owner = Address::new_from_array(
            ata_data[32..64].try_into().map_err(|_| ProgramError::InvalidAccountData)?
        );
        if &ata_owner != owner.address() {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // 验证 mint (offset 0)
        let ata_mint = Address::new_from_array(
            ata_data[0..32].try_into().map_err(|_| ProgramError::InvalidAccountData)?
        );
        if &ata_mint != mint.address() {
            return Err(ProgramError::InvalidAccountData);
        }
        
        Ok(())
    }

    /// 初始化 ATA
    pub fn init(
        ata: &AccountView,
        mint: &AccountView,
        payer: &AccountView,
        authority: &AccountView,
        _system_program: &AccountView,
        _token_program: &AccountView,
    ) -> ProgramResult {
        // 如果已经初始化，跳过
        if ata.data_len() > 0 {
            return Ok(());
        }

        // 使用 pinocchio-associated-token-account 创建 ATA
        Create {
            funding_account: payer,
            account: ata,
            wallet: authority,
            mint,
            system_program: _system_program,
            token_program: _token_program,
        }.invoke()?;

        Ok(())
    }

    /// 如果需要则初始化 ATA
    pub fn init_if_needed(
        ata: &AccountView,
        mint: &AccountView,
        payer: &AccountView,
        authority: &AccountView,
        system_program: &AccountView,
        token_program: &AccountView,
    ) -> ProgramResult {
        if ata.data_len() == 0 {
            Self::init(ata, mint, payer, authority, system_program, token_program)?;
        }
        Ok(())
    }
}

/// 程序账户辅助
pub struct ProgramAccount;

impl ProgramAccount {
    /// 验证程序账户
    pub fn check(account: &AccountView) -> ProgramResult {
        if account.data_len() == 0 {
            return Err(ProgramError::UninitializedAccount);
        }
        Ok(())
    }

    /// 初始化 PDA
    pub fn init<T>(
        payer: &AccountView,
        account: &AccountView,
        seeds: &[Seed],
        space: usize,
    ) -> ProgramResult {
        // 如果已经初始化，跳过
        if account.data_len() >= space {
            return Ok(());
        }

        // 创建签名者
        let signers = [Signer::from(seeds)];

        // 创建账户
        CreateAccount {
            from: payer,
            to: account,
            lamports: 10_000_000, // 租金豁免
            space: space as u64,
            owner: &crate::ID,
        }.invoke_signed(&signers)?;

        Ok(())
    }

    /// 关闭 PDA
    pub fn close(account: &AccountView, destination: &AccountView) -> ProgramResult {
        // 将账户的所有 lamports 转移到 destination
        unsafe {
            let account_lamports = account.lamports();
            let dest_lamports_ptr = destination.lamports() as *const u64 as *mut u64;
            let acc_lamports_ptr = account.lamports() as *const u64 as *mut u64;
            
            *dest_lamports_ptr += account_lamports;
            *acc_lamports_ptr = 0;
        }

        // 清零数据
        let mut data = account.try_borrow_mut()?;
        data.fill(0);

        Ok(())
    }
}

/// Token 账户辅助
pub struct TokenAccount<'a> {
    account: &'a AccountView,
}

impl<'a> TokenAccount<'a> {
    /// 从 AccountView 创建 TokenAccount
    pub fn from_account_info(account: &'a AccountView) -> Result<Self, ProgramError> {
        if account.data_len() < 165 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self { account })
    }

    /// 获取 Token 账户余额
    pub fn amount(&self) -> Result<u64, ProgramError> {
        let data = unsafe { self.account.borrow_unchecked() };
        if data.len() < 72 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(u64::from_le_bytes(
            data[64..72].try_into().map_err(|_| ProgramError::InvalidAccountData)?
        ))
    }
}
