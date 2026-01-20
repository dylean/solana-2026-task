use core::mem::size_of;
use pinocchio::address::Address;

/// Escrow 状态结构
/// 
/// 存储托管交易的所有条款和状态
#[repr(C)]
pub struct Escrow {
    pub seed: u64,        // 用于 PDA 派生的随机数
    pub maker: Address,   // 创建者公钥
    pub mint_a: Address,  // 代币 A 的 Mint 地址
    pub mint_b: Address,  // 代币 B 的 Mint 地址
    pub receive: u64,     // 期望接收的代币 B 数量
    pub bump: [u8; 1],    // PDA bump seed
}

impl Escrow {
    /// Escrow 结构的大小（字节）
    pub const LEN: usize = size_of::<u64>()      // seed
        + size_of::<Address>()                    // maker (32 bytes)
        + size_of::<Address>()                    // mint_a (32 bytes)
        + size_of::<Address>()                    // mint_b (32 bytes)
        + size_of::<u64>()                        // receive
        + size_of::<[u8; 1]>();                   // bump

    /// 从字节数组加载 Escrow（不可变）
    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, pinocchio::error::ProgramError> {
        if bytes.len() < Self::LEN {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
        
        Ok(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
    
    /// 从字节数组加载 Escrow（可变）
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, pinocchio::error::ProgramError> {
        if bytes.len() < Self::LEN {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
        
        Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut Self) })
    }

    /// 设置所有字段
    #[inline(always)]
    pub fn set_inner(
        &mut self,
        seed: u64,
        maker: Address,
        mint_a: Address,
        mint_b: Address,
        receive: u64,
        bump: [u8; 1],
    ) {
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.bump = bump;
    }

    /// 设置 seed
    #[inline(always)]
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    /// 设置 maker
    #[inline(always)]
    pub fn set_maker(&mut self, maker: Address) {
        self.maker = maker;
    }

    /// 设置 mint_a
    #[inline(always)]
    pub fn set_mint_a(&mut self, mint_a: Address) {
        self.mint_a = mint_a;
    }

    /// 设置 mint_b
    #[inline(always)]
    pub fn set_mint_b(&mut self, mint_b: Address) {
        self.mint_b = mint_b;
    }

    /// 设置 receive
    #[inline(always)]
    pub fn set_receive(&mut self, receive: u64) {
        self.receive = receive;
    }

    /// 设置 bump
    #[inline(always)]
    pub fn set_bump(&mut self, bump: [u8; 1]) {
        self.bump = bump;
    }
}
