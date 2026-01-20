use pinocchio::{
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
    cpi::{Seed, Signer},
};

/// Initialize 指令数据
pub struct InitializeInstructionData {
    pub seed: u64,
    pub fee: u16,
    pub mint_x: Address,
    pub mint_y: Address,
    pub config_bump: u8,
    pub lp_bump: u8,
    pub authority: Address,
}

impl InitializeInstructionData {
    /// 从字节数组解析指令数据
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        // 最小长度：8 + 2 + 32 + 32 + 1 + 1 = 76
        // 带 authority：76 + 32 = 108
        if data.len() < 76 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let fee = u16::from_le_bytes(data[8..10].try_into().unwrap());
        
        let mut mint_x = [0u8; 32];
        mint_x.copy_from_slice(&data[10..42]);
        let mint_x = Address::new_from_array(mint_x);
        
        let mut mint_y = [0u8; 32];
        mint_y.copy_from_slice(&data[42..74]);
        let mint_y = Address::new_from_array(mint_y);
        
        let config_bump = data[74];
        let lp_bump = data[75];
        
        let authority = if data.len() >= 108 {
            let mut auth = [0u8; 32];
            auth.copy_from_slice(&data[76..108]);
            Address::new_from_array(auth)
        } else {
            // 默认权限为零地址（不可变）
            Address::new_from_array([0u8; 32])
        };

        // 验证费用不超过 100% (10000 基点)
        if fee > 10_000 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            fee,
            mint_x,
            mint_y,
            config_bump,
            lp_bump,
            authority,
        })
    }
}

/// Initialize 指令 - 初始化 AMM
/// 
/// 账户顺序：
/// 0. initializer (signer, writable) - 初始化者
/// 1. config (writable) - Config 账户
/// 2. mint_lp (writable) - LP Token Mint
/// 3. system_program - 系统程序
/// 4. token_program - Token 程序
pub fn initialize(program_id: &Address, data: &[u8], accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let initializer = &accounts[0];
    let config = &accounts[1];
    let mint_lp = &accounts[2];
    let _system_program = &accounts[3];
    let _token_program = &accounts[4];

    // 验证 initializer 是签名者
    if !initializer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 解析指令数据
    let instruction_data = InitializeInstructionData::try_from_bytes(data)?;

    // 验证 mint 不同
    if instruction_data.mint_x == instruction_data.mint_y {
        return Err(ProgramError::InvalidInstructionData);
    }

    // 1. 创建 Config 账户（使用 PDA）
    let seed_bytes = instruction_data.seed.to_le_bytes();
    let config_bump_binding = [instruction_data.config_bump];
    let config_seeds = [
        Seed::from(b"config"),
        Seed::from(&seed_bytes),
        Seed::from(instruction_data.mint_x.as_ref()),
        Seed::from(instruction_data.mint_y.as_ref()),
        Seed::from(&config_bump_binding),
    ];
    let config_signers = [Signer::from(&config_seeds)];

    // 创建 config 账户
    pinocchio_system::instructions::CreateAccount {
        from: initializer,
        to: config,
        lamports: 10_000_000, // 足够的租金豁免
        space: 108, // Config::LEN
        owner: program_id,
    }.invoke_signed(&config_signers)?;
    
    // 2. 填充 Config 数据
    let mut config_data = config.try_borrow_mut()?;
    let mut offset = 0;
    
    // state (1 byte) - Initialized = 1
    config_data[offset] = 1;
    offset += 1;
    
    // seed (8 bytes)
    config_data[offset..offset+8].copy_from_slice(&instruction_data.seed.to_le_bytes());
    offset += 8;
    
    // authority (32 bytes)
    config_data[offset..offset+32].copy_from_slice(instruction_data.authority.as_ref());
    offset += 32;
    
    // mint_x (32 bytes)
    config_data[offset..offset+32].copy_from_slice(instruction_data.mint_x.as_ref());
    offset += 32;
    
    // mint_y (32 bytes)
    config_data[offset..offset+32].copy_from_slice(instruction_data.mint_y.as_ref());
    offset += 32;
    
    // fee (2 bytes)
    config_data[offset..offset+2].copy_from_slice(&instruction_data.fee.to_le_bytes());
    offset += 2;
    
    // config_bump (1 byte)
    config_data[offset] = instruction_data.config_bump;
    
    drop(config_data);

    // 3. 创建 LP Mint 账户（使用 PDA）
    let lp_bump_binding = [instruction_data.lp_bump];
    let lp_seeds = [
        Seed::from(b"mint_lp"),
        Seed::from(config.address().as_ref()),
        Seed::from(&lp_bump_binding),
    ];
    let lp_signers = [Signer::from(&lp_seeds)];

    pinocchio_system::instructions::CreateAccount {
        from: initializer,
        to: mint_lp,
        lamports: 2_000_000,
        space: 82, // Token Mint 标准大小
        owner: &pinocchio_token::ID,
    }.invoke_signed(&lp_signers)?;

    // 手动初始化 LP Mint，直接写入数据，避免 CPI 权限问题
    // Mint 账户布局：82 字节
    let mut mint_data = mint_lp.try_borrow_mut()?;
    
    // COption::Some(mint_authority) - 1 byte (1) + 32 bytes (address)
    mint_data[0] = 1; // COption::Some
    mint_data[1..33].copy_from_slice(config.address().as_ref());
    
    // supply - 8 bytes (u64) = 0
    mint_data[33..41].copy_from_slice(&0u64.to_le_bytes());
    
    // decimals - 1 byte = 6
    mint_data[41] = 6;
    
    // is_initialized - 1 byte = 1
    mint_data[42] = 1;
    
    // COption::None(freeze_authority) - 1 byte (0)
    mint_data[43] = 0;

    Ok(())
}
