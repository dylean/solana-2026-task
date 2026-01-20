use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
    AccountView,
    ProgramResult,
};
use pinocchio_token::instructions::{Transfer, CloseAccount};
use crate::{ID, state::Escrow};

/// Take 指令 - 接受托管
/// 
/// 账户顺序：
/// 0. taker (signer, writable) - 接受者
/// 1. maker (writable) - 创建者
/// 2. escrow (writable) - Escrow 账户
/// 3. mint_a - 代币 A 的 Mint
/// 4. mint_b - 代币 B 的 Mint
/// 5. vault (writable) - 金库代币账户
/// 6. taker_ata_a (writable) - 接受者的代币 A 账户
/// 7. taker_ata_b (writable) - 接受者的代币 B 账户
/// 8. maker_ata_b (writable) - 创建者的代币 B 账户
/// 9. system_program - 系统程序
/// 10. token_program - Token 程序
pub fn take(accounts: &[AccountView]) -> ProgramResult {
    // 验证账户数量
    if accounts.len() < 11 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // 解析账户
    let taker = &accounts[0];
    let maker = &accounts[1];
    let escrow = &accounts[2];
    let _mint_a = &accounts[3];
    let _mint_b = &accounts[4];
    let vault = &accounts[5];
    let taker_ata_a = &accounts[6];
    let taker_ata_b = &accounts[7];
    let maker_ata_b = &accounts[8];
    let _system_program = &accounts[9];
    let _token_program = &accounts[10];

    // 验证 taker 是签名者
    if !taker.is_signer() {
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

    // 1. 转移代币 B 从 taker 到 maker
    Transfer {
        from: taker_ata_b,
        to: maker_ata_b,
        authority: taker,
        amount: escrow_state.receive,
    }.invoke()?;

    // 2. 使用 PDA 签名转移代币 A 从 vault 到 taker
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
            to: taker_ata_a,
            authority: escrow,
            amount: vault_amount,
        }.invoke_signed(&signers)?;
    }

    // 3. 关闭 vault 账户
    CloseAccount {
        account: vault,
        destination: maker,
        authority: escrow,
    }.invoke_signed(&signers)?;

    // 注意：escrow 账户关闭需要在客户端处理或使用其他机制
    // 这里我们简化处理，只关闭 token 账户

    Ok(())
}
