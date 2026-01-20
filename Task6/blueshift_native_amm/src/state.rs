use core::mem::size_of;
use pinocchio::{error::ProgramError, Address};

/// AMM 配置状态
/// 使用字节数组确保内存对齐
#[repr(C)]
pub struct Config {
    pub state: u8,              // AMM 状态
    pub seed: u64,              // PDA 派生种子
    pub authority: [u8; 32],    // 管理权限
    pub mint_x: [u8; 32],       // 代币 X 的 Mint
    pub mint_y: [u8; 32],       // 代币 Y 的 Mint
    pub fee: u16,               // 交换费用（基点）
    pub config_bump: u8,        // PDA bump seed
}

/// AMM 状态枚举
#[repr(u8)]
pub enum AmmState {
    Uninitialized = 0u8,    // 未初始化
    Initialized = 1u8,      // 已初始化
    Disabled = 2u8,         // 已禁用
    WithdrawOnly = 3u8,     // 仅限提取
}

impl Config {
    /// Config 结构的大小（字节）
    pub const LEN: usize = size_of::<u8>()       // state
        + size_of::<u64>()                        // seed
        + 32                                      // authority
        + 32                                      // mint_x
        + 32                                      // mint_y
        + size_of::<u16>()                        // fee
        + size_of::<u8>();                        // config_bump

    /// 从字节数组加载 Config（不可变）
    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        Ok(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
    
    /// 从字节数组加载 Config（可变）
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut Self) })
    }

    /// 设置所有字段
    #[inline(always)]
    pub fn set_inner(
        &mut self,
        seed: u64,
        authority: &Address,
        mint_x: &Address,
        mint_y: &Address,
        fee: u16,
        config_bump: u8,
    ) {
        self.state = AmmState::Initialized as u8;
        self.seed = seed;
        self.authority.copy_from_slice(authority.as_ref());
        self.mint_x.copy_from_slice(mint_x.as_ref());
        self.mint_y.copy_from_slice(mint_y.as_ref());
        self.fee = fee;
        self.config_bump = config_bump;
    }

    /// 检查 AMM 状态
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.state == AmmState::Initialized as u8
    }

    /// 检查是否可以提取
    #[inline(always)]
    pub fn can_withdraw(&self) -> bool {
        self.state == AmmState::Initialized as u8 || self.state == AmmState::WithdrawOnly as u8
    }
    
    /// 获取 mint_x 作为 Address
    #[inline(always)]
    pub fn mint_x_address(&self) -> Address {
        Address::new_from_array(self.mint_x)
    }
    
    /// 获取 mint_y 作为 Address
    #[inline(always)]
    pub fn mint_y_address(&self) -> Address {
        Address::new_from_array(self.mint_y)
    }
}
