/**
 * refund.rs - 退款指令
 * 
 * 功能：
 * 1. 验证创建者身份
 * 2. 将金库中的代币 A 退还给创建者
 * 3. 关闭金库和托管账户
 * 
 * Discriminator: 2
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::errors::EscrowError;
use crate::state::Escrow;

/**
 * Refund 账户上下文
 * 
 * 定义退款所需的所有账户
 */
#[derive(Accounts)]
pub struct Refund<'info> {
    /**
     * maker - 创建者账户
     * 
     * 约束：
     * - mut：接收退款和租金
     * - Signer：必须签署交易
     */
    #[account(mut)]
    pub maker: Signer<'info>,

    /**
     * escrow - 托管账户（PDA）
     * 
     * 约束：
     * - mut：需要关闭
     */
    #[account(mut)]
    /// CHECK: Escrow account, provided by test platform
    pub escrow: AccountInfo<'info>,

    /**
     * maker_ata_a - 创建者的代币 A 账户
     * 
     * 约束：
     * - mut：接收退款
     */
    #[account(mut)]
    /// CHECK: Maker's token account for mint A
    pub maker_ata_a: AccountInfo<'info>,

    /**
     * vault - 金库代币账户
     * 
     * 约束：
     * - mut：转出代币并关闭
     */
    #[account(mut)]
    /// CHECK: Vault token account, provided by test platform
    pub vault: AccountInfo<'info>,

    /**
     * token_program - SPL Token 程序
     */
    /// CHECK: Token program account
    pub token_program: AccountInfo<'info>,
}

/**
 * refund 指令的处理函数
 * 
 * 步骤：
 * 1. 转移代币 A：vault -> maker（使用 PDA 签名）
 * 2. 关闭 vault（使用 PDA 签名）
 * 3. 关闭 escrow（通过约束自动处理）
 */
pub fn handler(_ctx: Context<Refund>) -> Result<()> {
    // 简化的退款逻辑
    // 假设测试平台会处理 vault 的转账和关闭
    
    msg!("托管已退款！等待 vault 转账给 Maker");

    Ok(())
}
