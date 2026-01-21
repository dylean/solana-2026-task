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
    /// 
    /// 正确地转移 lamports 并清零数据
    pub fn close(account: &AccountView, destination: &AccountView) -> ProgramResult {
        // 获取账户的 lamports
        let account_lamports = account.lamports();
        
        // 转移 lamports
        // 在 Solana 中，AccountInfo 的内部结构包含一个指向 lamports 的指针
        // 我们需要使用 unsafe 代码来访问和修改这些值
        unsafe {
            // 获取账户结构的内部指针
            // AccountView 内部应该有一个指向账户数据的指针
            // lamports 字段位于账户数据的开头（offset 0）
            
            // 将 AccountView 转换为原始指针，然后访问 lamports 字段
            // 注意：这依赖于 Solana 账户的内存布局
            let account_ptr = account as *const AccountView as *const u8 as *mut u8;
            let dest_ptr = destination as *const AccountView as *const u8 as *mut u8;
            
            // Solana 账户的内存布局：
            // - duplicate_info: u8 (1 byte)
            // - is_signer: u8 (1 byte) 
            // - is_writable: u8 (1 byte)
            // - executable: u8 (1 byte)
            // - padding: u32 (4 bytes)
            // - key: Pubkey (32 bytes)
            // - owner: Pubkey (32 bytes)
            // - lamports: *mut u64 (8 bytes pointer)
            // - data_len: u64 (8 bytes)
            // - data: *mut u8 (8 bytes pointer)
            
            // lamports 指针位于偏移 72 的位置
            let account_lamports_ptr_addr = account_ptr.add(72) as *mut *mut u64;
            let dest_lamports_ptr_addr = dest_ptr.add(72) as *mut *mut u64;
            
            let account_lamports_ptr = *account_lamports_ptr_addr;
            let dest_lamports_ptr = *dest_lamports_ptr_addr;
            
            // 现在我们有了实际的 lamports 值的指针，可以修改它们
            *dest_lamports_ptr = (*dest_lamports_ptr).checked_add(account_lamports)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            *account_lamports_ptr = 0;
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
