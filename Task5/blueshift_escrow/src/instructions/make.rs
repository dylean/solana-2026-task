use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::Transfer;
use core::mem::size_of;
use crate::{ID, state::Escrow};

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
        if amount == 0 || receive == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}

/// Make 指令 - 创建托管
/// 
/// 账户顺序：
/// 0. maker (signer, writable) - 创建者
/// 1. escrow (writable) - Escrow 账户
/// 2. mint_a - 代币 A 的 Mint
/// 3. mint_b - 代币 B 的 Mint
/// 4. maker_ata_a (writable) - 创建者的代币 A 账户
/// 5. vault (writable) - 金库代币账户
/// 6. system_program - 系统程序
/// 7. token_program - Token 程序
/// 8. associated_token_program - ATA 程序
pub fn make(data: &[u8], accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 9 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let maker = &accounts[0];
    let escrow = &accounts[1];
    let mint_a = &accounts[2];
    let mint_b = &accounts[3];
    let maker_ata_a = &accounts[4];
    let vault = &accounts[5];
    let _system_program = &accounts[6];
    let _token_program = &accounts[7];
    let _ata_program = &accounts[8];

    // 验证 maker 是签名者
    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 解析指令数据
    let instruction_data = MakeInstructionData::try_from_bytes(data)?;

    // 验证 mint 不同
    if mint_a.address() == mint_b.address() {
        return Err(ProgramError::InvalidAccountData);
    }

    // 验证 escrow PDA
    let seed_bytes = instruction_data.seed.to_le_bytes();
    let (expected_escrow, bump) = Address::find_program_address(
        &[b"escrow", maker.address().as_ref(), &seed_bytes],
        &ID,
    );

    if escrow.address() != &expected_escrow {
        return Err(ProgramError::InvalidAccountData);
    }

    // 创建 PDA 签名种子
    let bump_binding = [bump];
    let seeds = [
        Seed::from(b"escrow"),
        Seed::from(maker.address().as_ref()),
        Seed::from(&seed_bytes),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&seeds)];

    // 创建 escrow 账户（如果还未初始化）
    if escrow.data_len() == 0 {
        CreateAccount {
            from: maker,
            to: escrow,
            lamports: 10_000_000, // 租金豁免金额
            space: Escrow::LEN as u64,
            owner: &ID,
        }.invoke_signed(&signers)?;

        // 填充 escrow 数据
        unsafe {
            let escrow_data = escrow.borrow_unchecked();
            if escrow_data.len() >= Escrow::LEN {
                // 需要可变借用来修改数据
                let escrow_ptr = escrow_data.as_ptr() as *mut u8;
                let escrow_slice = core::slice::from_raw_parts_mut(escrow_ptr, Escrow::LEN);
                let escrow_account = Escrow::load_mut(escrow_slice)?;
                escrow_account.set_inner(
                    instruction_data.seed,
                    maker.address().clone(),
                    mint_a.address().clone(),
                    mint_b.address().clone(),
                    instruction_data.receive,
                    [bump],
                );
            }
        }
    }

    // 转移代币到 vault
    // 注意：假设 vault ATA 已经在客户端创建
    if vault.data_len() > 0 {
        Transfer {
            from: maker_ata_a,
            to: vault,
            authority: maker,
            amount: instruction_data.amount,
        }.invoke()?;
    }

    Ok(())
}
