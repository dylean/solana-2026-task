use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    AccountView,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

/// Withdraw 指令处理函数
pub fn withdraw(accounts: &[AccountView]) -> ProgramResult {
    // 解析账户
    let [owner, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    // 账户验证
    if !owner.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    if !vault.owned_by(&pinocchio_system::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    
    if vault.lamports() == 0 {
        return Err(ProgramError::InsufficientFunds);
    }
    
    let (vault_key, bump) = pinocchio::Address::find_program_address(&[b"vault", owner.address().as_ref()], &crate::ID);
    if vault.address() != &vault_key {
        return Err(ProgramError::InvalidSeeds);
    }
    
    // 创建 PDA 签名种子
    let bump_binding = [bump];
    let seeds = [
        Seed::from(b"vault"),
        Seed::from(owner.address().as_ref()),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&seeds)];
    
    // 执行转账（使用 PDA 签名）
    Transfer {
        from: vault,
        to: owner,
        lamports: vault.lamports(),
    }
    .invoke_signed(&signers)?;
    
    Ok(())
}
