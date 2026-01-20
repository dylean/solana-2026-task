use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_token::instructions::Transfer;
use core::mem::size_of;
use crate::state::Config;

/// Swap 指令数据
pub struct SwapInstructionData {
    pub is_x: bool,      // 是否从 X 交换到 Y
    pub amount: u64,     // 输入数量
    pub min: u64,        // 最小输出数量
    pub expiration: i64, // 过期时间
}

impl SwapInstructionData {
    /// 从字节数组解析指令数据
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() != size_of::<u8>() + size_of::<u64>() * 2 + size_of::<i64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let is_x = data[0] != 0;
        let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
        let min = u64::from_le_bytes(data[9..17].try_into().unwrap());
        let expiration = i64::from_le_bytes(data[17..25].try_into().unwrap());

        // 验证数据
        if amount == 0 || min == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            is_x,
            amount,
            min,
            expiration,
        })
    }
}

/// Swap 指令 - 代币交换
/// 
/// 账户顺序：
/// 0. user (signer) - 用户
/// 1. config - Config 账户
/// 2. vault_x (writable) - X 代币金库
/// 3. vault_y (writable) - Y 代币金库
/// 4. user_x_ata (writable) - 用户的 X 代币账户
/// 5. user_y_ata (writable) - 用户的 Y 代币账户
/// 6. token_program - Token 程序
pub fn swap(_program_id: &Address, data: &[u8], accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 7 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let user = &accounts[0];
    let config = &accounts[1];
    let vault_x = &accounts[2];
    let vault_y = &accounts[3];
    let user_x_ata = &accounts[4];
    let user_y_ata = &accounts[5];
    let _token_program = &accounts[6];

    // 验证 user 是签名者
    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 解析指令数据
    let instruction_data = SwapInstructionData::try_from_bytes(data)?;

    // 读取 config 状态
    let config_data = config.try_borrow()?;
    let config_state = Config::load(&config_data)?;

    // 验证 AMM 状态
    if !config_state.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 简化版本：直接按固定比例交换
    // 实际实现需要使用 constant-product-curve 计算精确金额和费用
    
    // 创建 PDA 签名种子
    let seed_bytes = config_state.seed.to_le_bytes();
    let config_bump_binding = [config_state.config_bump];
    let mint_x_address = config_state.mint_x_address();
    let mint_y_address = config_state.mint_y_address();
    
    let config_seeds = [
        Seed::from(b"config"),
        Seed::from(&seed_bytes),
        Seed::from(mint_x_address.as_ref()),
        Seed::from(mint_y_address.as_ref()),
        Seed::from(&config_bump_binding),
    ];
    let config_signers = [Signer::from(&config_seeds)];
    
    if instruction_data.is_x {
        // X -> Y: 用户转入 X，接收 Y
        Transfer {
            from: user_x_ata,
            to: vault_x,
            authority: user,
            amount: instruction_data.amount,
        }.invoke()?;

        // 从金库转出 Y（使用 PDA 签名）
        Transfer {
            from: vault_y,
            to: user_y_ata,
            authority: config,
            amount: instruction_data.min,
        }.invoke_signed(&config_signers)?;
    } else {
        // Y -> X: 用户转入 Y，接收 X
        Transfer {
            from: user_y_ata,
            to: vault_y,
            authority: user,
            amount: instruction_data.amount,
        }.invoke()?;

        // 从金库转出 X（使用 PDA 签名）
        Transfer {
            from: vault_x,
            to: user_x_ata,
            authority: config,
            amount: instruction_data.min,
        }.invoke_signed(&config_signers)?;
    }

    Ok(())
}
