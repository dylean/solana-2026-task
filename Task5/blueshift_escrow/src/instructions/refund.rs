use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_token::instructions::{Transfer, CloseAccount};
use crate::{ID, state::Escrow};

/// Refund 指令 - 退款
/// 
/// 账户顺序：
/// 0. maker (signer, writable) - 创建者
/// 1. escrow (writable) - Escrow 账户
/// 2. mint_a - 代币 A 的 Mint
/// 3. vault (writable) - 金库代币账户
/// 4. maker_ata_a (writable) - 创建者的代币 A 账户
/// 5. system_program - 系统程序
/// 6. token_program - Token 程序
pub fn refund(accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 7 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let maker = &accounts[0];
    let escrow = &accounts[1];
    let _mint_a = &accounts[2];
    let vault = &accounts[3];
    let maker_ata_a = &accounts[4];
    let _system_program = &accounts[5];
    let _token_program = &accounts[6];

    // 验证 maker 是签名者
    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 读取 escrow 状态
    let escrow_data = unsafe { escrow.borrow_unchecked() };
    let escrow_state = Escrow::load(&escrow_data)?;

    // 验证 maker 匹配
    if maker.address() != &escrow_state.maker {
        return Err(ProgramError::InvalidAccountData);
    }

    // 验证 escrow PDA
    let seed_bytes = escrow_state.seed.to_le_bytes();
    let (expected_escrow, _bump) = Address::find_program_address(
        &[b"escrow", maker.address().as_ref(), &seed_bytes],
        &ID,
    );

    if escrow.address() != &expected_escrow {
        return Err(ProgramError::InvalidAccountData);
    }

    let bump = escrow_state.bump[0];

    // 创建 PDA 签名种子
    let bump_binding = [bump];
    let seeds = [
        Seed::from(b"escrow"),
        Seed::from(maker.address().as_ref()),
        Seed::from(&seed_bytes),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&seeds)];

    // 1. 使用 PDA 签名转移代币 A 从 vault 回到 maker
    // 获取 vault 中的全部余额
    let vault_data = unsafe { vault.borrow_unchecked() };
    let vault_amount = if vault_data.len() >= 72 {
        // SPL Token 账户中 amount 字段在偏移 64 处
        u64::from_le_bytes(vault_data[64..72].try_into().unwrap())
    } else {
        0
    };
    drop(vault_data);

    if vault_amount > 0 {
        Transfer {
            from: vault,
            to: maker_ata_a,
            authority: escrow,
            amount: vault_amount,
        }.invoke_signed(&signers)?;
    }

    // 2. 关闭 vault 账户
    CloseAccount {
        account: vault,
        destination: maker,
        authority: escrow,
    }.invoke_signed(&signers)?;

    // 注意：escrow 账户关闭需要在客户端处理或使用其他机制
    // 这里我们简化处理，只关闭 token 账户

    Ok(())
}
