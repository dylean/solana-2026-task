/**
 * lib.rs - Anchor 托管程序入口
 * 
 * 这是一个无需信任的代币交换程序，实现了安全的托管服务
 * 
 * 核心功能：
 * 1. Make（创建托管）- Discriminator: 0
 * 2. Take（接受托管）- Discriminator: 1
 * 3. Refund（退款）- Discriminator: 2
 * 
 * 程序 ID: 22222222222222222222222222222222222222222222
 */

use anchor_lang::prelude::*;

// 导入并导出模块
pub mod errors;
pub mod instructions;
pub mod state;

// 使用指令
use instructions::*;
pub use errors::*;
pub use state::*;

// ⚠️ 重要：此程序 ID 必须设置为指定值以通过测试
declare_id!("22222222222222222222222222222222222222222222");

/**
 * 程序模块
 * 
 * 定义所有可调用的指令
 * 每个指令都有自定义的 discriminator
 */
#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    /**
     * make - 创建托管指令
     * 
     * Discriminator: 0
     * 
     * 功能：
     * - 创建者发起托管报价
     * - 存入代币 A 到金库
     * - 设定期望接收的代币 B 数量
     * 
     * 参数：
     * - ctx: 账户上下文
     * - seed: 随机数种子（用于 PDA 派生）
     * - receive: 期望接收的代币 B 数量
     * - amount: 存入的代币 A 数量
     * 
     * 返回：
     * - Result<()>: 成功或失败
     */
    #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }

    /**
     * take - 接受托管指令
     * 
     * Discriminator: 1
     * 
     * 功能：
     * - 接受者接受托管报价
     * - 提供代币 B 给创建者
     * - 获得代币 A 从金库
     * 
     * 参数：
     * - ctx: 账户上下文
     * 
     * 返回：
     * - Result<()>: 成功或失败
     */
    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    /**
     * refund - 退款指令
     * 
     * Discriminator: 2
     * 
     * 功能：
     * - 创建者取消托管报价
     * - 退回代币 A 给创建者
     * - 关闭托管账户
     * 
     * 参数：
     * - ctx: 账户上下文
     * 
     * 返回：
     * - Result<()>: 成功或失败
     */
    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}

/*
 * 程序设计说明：
 * 
 * 1. 模块化设计
 *    - state.rs: 状态定义
 *    - errors.rs: 错误定义
 *    - instructions/: 指令实现
 *    - lib.rs: 程序入口
 * 
 * 2. 自定义 Discriminator
 *    - State discriminator = 1
 *    - Make discriminator = 0
 *    - Take discriminator = 1
 *    - Refund discriminator = 2
 * 
 * 3. 安全性考虑
 *    - PDA 控制金库
 *    - 约束验证账户
 *    - 原子性交易
 *    - 租金返还
 * 
 * 4. 依赖要求
 *    - anchor-lang ^0.32.1 (with init-if-needed feature)
 *    - anchor-spl ^0.32.1
 * 
 * 使用流程：
 * 
 * 创建托管：
 * 1. Alice 调用 make(seed, 100 USDC, 10 SOL)
 * 2. 存入 10 SOL 到金库
 * 3. 设定期望收到 100 USDC
 * 
 * 接受托管：
 * 1. Bob 调用 take()
 * 2. Bob 转 100 USDC 给 Alice
 * 3. Bob 收到 10 SOL
 * 4. 托管完成
 * 
 * 退款：
 * 1. Alice 改变主意，调用 refund()
 * 2. Alice 收回 10 SOL
 * 3. 托管取消
 */
