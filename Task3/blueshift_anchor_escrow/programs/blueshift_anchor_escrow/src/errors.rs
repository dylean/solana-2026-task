/**
 * errors.rs - 自定义错误定义
 * 
 * 定义了程序可能返回的所有错误类型
 */

use anchor_lang::prelude::*;

/**
 * EscrowError - 托管程序错误枚举
 * 
 * 每个错误都映射到一个清晰、易于理解的消息
 * 当约束或 require!() 失败时，Anchor 会显示这些消息
 */
#[error_code]
pub enum EscrowError {
    /**
     * InvalidAmount - 无效的金额
     * 
     * 触发场景：
     * - 尝试存入或接收 0 数量的代币
     * - 金额超出合理范围
     * 
     * 使用位置：
     * - make 指令：验证 receive 和 amount 参数
     */
    #[msg("无效的金额")]
    InvalidAmount,

    /**
     * InvalidMaker - 无效的创建者
     * 
     * 触发场景：
     * - Escrow 账户中存储的 maker 与传入的 maker 不匹配
     * - 防止未授权的用户执行操作
     * 
     * 使用位置：
     * - take 指令：验证 maker 地址
     * - refund 指令：确保只有创建者可以退款
     */
    #[msg("无效的创建者")]
    InvalidMaker,

    /**
     * InvalidMintA - 无效的代币 A 铸币地址
     * 
     * 触发场景：
     * - Escrow 账户中存储的 mint_a 与传入的 mint_a 不匹配
     * - 防止使用错误的代币类型
     * 
     * 使用位置：
     * - take 指令：验证代币 A 的铸币地址
     * - refund 指令：验证要退还的代币类型
     */
    #[msg("无效的代币 A 铸币地址")]
    InvalidMintA,

    /**
     * InvalidMintB - 无效的代币 B 铸币地址
     * 
     * 触发场景：
     * - Escrow 账户中存储的 mint_b 与传入的 mint_b 不匹配
     * - 防止接受者使用错误的代币类型
     * 
     * 使用位置：
     * - take 指令：验证代币 B 的铸币地址
     */
    #[msg("无效的代币 B 铸币地址")]
    InvalidMintB,
}

/**
 * 错误处理最佳实践：
 * 
 * 1. 清晰的错误消息
 *    - 使用中文描述，便于理解
 *    - 明确指出问题所在
 * 
 * 2. 细粒度的错误类型
 *    - 为不同的验证失败提供不同的错误
 *    - 便于调试和问题定位
 * 
 * 3. 安全性考虑
 *    - 通过验证防止未授权操作
 *    - 确保数据一致性
 * 
 * 使用示例：
 * ```rust
 * require_gt!(amount, 0, EscrowError::InvalidAmount);
 * require!(
 *     escrow.maker == maker.key(),
 *     EscrowError::InvalidMaker
 * );
 * ```
 */
