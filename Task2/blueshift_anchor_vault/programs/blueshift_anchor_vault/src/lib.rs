/**
 * Anchor 金库程序（Vault Program）
 * 
 * 这是一个简单的 Solana 程序，允许用户：
 * 1. 将 SOL（lamports）存入个人金库
 * 2. 从个人金库中提取所有 SOL
 * 
 * 核心概念：
 * - PDA（程序派生地址）：使用用户公钥派生的确定性地址
 * - CPI（跨程序调用）：调用系统程序进行转账
 * - 租金豁免：确保账户有足够余额以免被清除
 */

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// ⚠️ 重要：此程序 ID 必须设置为指定值以通过测试
declare_id!("22222222222222222222222222222222222222222222");

/**
 * 程序模块
 * 包含两个核心指令：deposit 和 withdraw
 */
#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    /**
     * 存款指令
     * 
     * 功能：将指定数量的 lamports 从用户账户转移到其个人金库
     * 
     * 参数：
     * - ctx: 包含所有必需账户的上下文
     * - amount: 要存入的 lamports 数量
     * 
     * 返回：
     * - Result<()>: 成功返回 Ok(())，失败返回错误
     * 
     * 安全检查：
     * 1. 金库必须为空（防止重复存款）
     * 2. 存款金额必须大于免租金最低限额
     */
    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // ========================================
        // 步骤 1: 验证金库为空
        // ========================================
        // require_eq! 宏检查两个值是否相等
        // 如果金库已有 lamports，则抛出 VaultAlreadyExists 错误
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyExists
        );

        // ========================================
        // 步骤 2: 验证存款金额
        // ========================================
        // require_gt! 宏检查第一个值是否大于第二个值
        // 确保存款金额超过免租金最低限额（Rent::get()?.minimum_balance(0)）
        // 这是必要的，因为 Solana 账户需要保持一定余额才能存活
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        // ========================================
        // 步骤 3: 执行转账（CPI 调用）
        // ========================================
        // 使用跨程序调用（CPI）调用系统程序的转账指令
        // 将 lamports 从签名者账户转移到金库账户
        transfer(
            CpiContext::new(
                // 系统程序的账户信息
                ctx.accounts.system_program.to_account_info(),
                // 转账指令的参数
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),  // 转出账户（签名者）
                    to: ctx.accounts.vault.to_account_info(),     // 转入账户（金库）
                },
            ),
            amount,  // 转账金额
        )?;

        Ok(())
    }

    /**
     * 取款指令
     * 
     * 功能：将金库中的所有 lamports 转回用户账户
     * 
     * 参数：
     * - ctx: 包含所有必需账户的上下文
     * 
     * 返回：
     * - Result<()>: 成功返回 Ok(())，失败返回错误
     * 
     * 安全检查：
     * 1. 金库必须有余额（不能从空金库取款）
     * 2. 使用 PDA 签名确保只有金库所有者可以取款
     */
    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // ========================================
        // 步骤 1: 验证金库非空
        // ========================================
        // require_neq! 宏检查两个值是否不相等
        // 如果金库为空，则抛出 InvalidAmount 错误
        require_neq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::InvalidAmount
        );

        // ========================================
        // 步骤 2: 创建 PDA 签名者种子
        // ========================================
        // PDA（程序派生地址）需要特定的种子来签署交易
        // 这些种子必须与创建 PDA 时使用的种子完全匹配
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds: &[&[u8]] = &[
            b"vault",                    // 字符串种子
            signer_key.as_ref(),         // 签名者公钥作为种子
            &[ctx.bumps.vault]           // bump seed（确保地址不在曲线上）
        ];

        // ========================================
        // 步骤 3: 执行转账（带 PDA 签名的 CPI 调用）
        // ========================================
        // 使用 new_with_signer 允许 PDA 作为签名者执行转账
        // 这是关键的安全机制：只有知道正确种子的程序才能代表 PDA 签署交易
        transfer(
            CpiContext::new_with_signer(
                // 系统程序的账户信息
                ctx.accounts.system_program.to_account_info(),
                // 转账指令的参数
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),    // 转出账户（金库 PDA）
                    to: ctx.accounts.signer.to_account_info(),     // 转入账户（签名者）
                },
                // PDA 签名者种子（允许程序代表 PDA 签署）
                &[signer_seeds]
            ),
            // 转账金库中的所有 lamports
            ctx.accounts.vault.lamports()
        )?;

        Ok(())
    }
}

/**
 * VaultAction 账户结构
 * 
 * 这个结构定义了 deposit 和 withdraw 指令需要的所有账户
 * 使用相同的结构使代码更简洁、更易维护
 */
#[derive(Accounts)]
pub struct VaultAction<'info> {
    /**
     * 签名者账户
     * 
     * - Signer<'info>: 确保此账户已签署交易
     * - #[account(mut)]: 标记为可变，因为我们将修改其 lamports 余额
     * 
     * 这是金库的所有者，也是唯一可以存取金库资金的人
     */
    #[account(mut)]
    pub signer: Signer<'info>,

    /**
     * 金库账户（PDA）
     * 
     * - SystemAccount<'info>: 系统拥有的账户类型
     * - #[account(mut)]: 标记为可变，因为我们将修改其 lamports 余额
     * - seeds: 定义 PDA 的派生种子
     *   - b"vault": 字符串字面量作为固定种子
     *   - signer.key().as_ref(): 签名者的公钥作为唯一标识
     * - bump: 自动找到并验证 bump seed
     * 
     * PDA 的优势：
     * 1. 确定性地址：给定相同的种子，总是生成相同的地址
     * 2. 程序控制：只有程序可以签署代表 PDA 的交易
     * 3. 无私钥：PDA 没有对应的私钥，更安全
     */
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    /**
     * 系统程序
     * 
     * - Program<'info, System>: Solana 的系统程序类型
     * 
     * 需要包含系统程序因为我们要使用它的转账指令（CPI）
     * 系统程序是 Solana 的核心程序，负责创建账户、转账等基本操作
     */
    pub system_program: Program<'info, System>,
}

/**
 * 自定义错误枚举
 * 
 * 定义程序可能返回的错误类型
 * #[error_code] 宏会自动为每个错误分配唯一的错误码
 */
#[error_code]
pub enum VaultError {
    /**
     * 金库已存在错误
     * 
     * 当用户尝试向已有余额的金库存款时触发
     * 这防止了意外的重复存款
     */
    #[msg("金库已存在，不能重复存款")]
    VaultAlreadyExists,

    /**
     * 无效金额错误
     * 
     * 可能的情况：
     * 1. 存款金额小于或等于免租金最低限额
     * 2. 尝试从空金库取款
     */
    #[msg("无效的金额")]
    InvalidAmount,
}
