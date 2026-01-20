use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_token::instructions::{Transfer, MintTo};
use core::mem::size_of;
use crate::state::Config;

/// Deposit 指令数据
pub struct DepositInstructionData {
    pub amount: u64,     // LP 数量
    pub max_x: u64,      // 最大 X 数量
    pub max_y: u64,      // 最大 Y 数量
    pub expiration: i64, // 过期时间
}

impl DepositInstructionData {
    /// 从字节数组解析指令数据
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() != size_of::<u64>() * 3 + size_of::<i64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let max_x = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let max_y = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let expiration = i64::from_le_bytes(data[24..32].try_into().unwrap());

        // 验证数据
        if amount == 0 || max_x == 0 || max_y == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            amount,
            max_x,
            max_y,
            expiration,
        })
    }
}

/// Deposit 指令 - 存入流动性
/// 
/// 账户顺序：
/// 0. user (signer, writable) - 用户
/// 1. config - Config 账户
/// 2. mint_lp (writable) - LP Token Mint
/// 3. vault_x (writable) - X 代币金库
/// 4. vault_y (writable) - Y 代币金库
/// 5. user_x_ata (writable) - 用户的 X 代币账户
/// 6. user_y_ata (writable) - 用户的 Y 代币账户
/// 7. user_lp_ata (writable) - 用户的 LP 代币账户
/// 8. token_program - Token 程序
pub fn deposit(_program_id: &Address, data: &[u8], accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 9 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let user = &accounts[0];
    let config = &accounts[1];
    let mint_lp = &accounts[2];
    let vault_x = &accounts[3];
    let vault_y = &accounts[4];
    let user_x_ata = &accounts[5];
    let user_y_ata = &accounts[6];
    let user_lp_ata = &accounts[7];
    let _token_program = &accounts[8];

    // 验证 user 是签名者
    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 解析指令数据
    let instruction_data = DepositInstructionData::try_from_bytes(data)?;

    // 读取 config 状态
    let config_data = config.try_borrow()?;
    let config_state = Config::load(&config_data)?;

    // 验证 AMM 状态
    if !config_state.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // 简化版本：假设比例正确，直接转移代币
    // 实际实现需要使用 constant-product-curve 计算精确金额

    // 转移 X 代币到金库
    Transfer {
        from: user_x_ata,
        to: vault_x,
        authority: user,
        amount: instruction_data.max_x,
    }.invoke()?;

    // 转移 Y 代币到金库
    Transfer {
        from: user_y_ata,
        to: vault_y,
        authority: user,
        amount: instruction_data.max_y,
    }.invoke()?;

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

    // 铸造 LP 代币给用户（使用 config PDA 签名）
    MintTo {
        mint: mint_lp,
        account: user_lp_ata,
        mint_authority: config,
        amount: instruction_data.amount,
    }.invoke_signed(&config_signers)?;

    Ok(())
}
