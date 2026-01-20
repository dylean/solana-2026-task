/**
 * state.rs - 托管状态定义
 * 
 * 定义了 Escrow 账户的数据结构，存储托管交换的所有条款
 */

use anchor_lang::prelude::*;

/**
 * Escrow 账户结构
 * 
 * 存储托管交换的所有必要信息
 * 使用自定义 discriminator = 1（根据要求）
 */
#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    /**
     * seed - 随机数种子
     * 
     * 用途：
     * - 在 PDA 派生过程中使用
     * - 允许同一个创建者使用相同的代币对创建多个托管
     * - 存储在链上，以便始终可以重新派生 PDA
     */
    pub seed: u64,

    /**
     * maker - 创建者地址
     * 
     * 用途：
     * - 创建托管账户的钱包地址
     * - 用于退款操作
     * - 用于接收代币 B 的付款
     */
    pub maker: Pubkey,

    /**
     * mint_a - 代币 A 的铸币地址
     * 
     * 用途：
     * - 创建者"给出"的代币类型
     * - 存入金库的代币
     */
    pub mint_a: Pubkey,

    /**
     * mint_b - 代币 B 的铸币地址
     * 
     * 用途：
     * - 创建者"想要"的代币类型
     * - 接受者需要提供的代币
     */
    pub mint_b: Pubkey,

    /**
     * receive - 期望接收的代币 B 数量
     * 
     * 用途：
     * - 创建者希望获得的代币 B 的数量
     * - 接受者必须转移的金额
     * 
     * 注意：
     * - 金库的余额本身显示了存入的代币 A 的数量
     * - 因此我们不需要单独存储代币 A 的数量
     */
    pub receive: u64,

    /**
     * bump - PDA bump seed
     * 
     * 用途：
     * - 缓存的 bump 字节
     * - 动态派生它会消耗计算资源
     * - 保存一次以提高效率
     */
    pub bump: u8,
}

/*
 * 为什么只存储这些字段？
 * 
 * 设计原则：
 * - 额外的字节意味着额外的租金成本
 * - 仅存储必要内容可以保持存款成本低
 * - 同时仍然让程序执行所需的每一条规则
 * 
 * 可以推导的信息（无需存储）：
 * - 代币 A 的存入数量：从 vault.amount 获取
 * - Escrow PDA 地址：可以从 seed 和 maker 重新派生
 * - 创建时间：可以从区块链历史查询（如果需要）
 */
