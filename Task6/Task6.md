# Pinocchio AMM（自动化做市商）

## 项目简介

### 什么是 AMM？

自动化做市商（AMM）是去中心化金融的基础构件之一，它使用户能够直接通过智能合约交换代币，而无需依赖传统的订单簿或中心化交易所。

可以将 AMM 想象成一个自运行的流动性池：用户存入代币对，AMM 使用数学公式来确定价格并促进代币之间的交换。这使得任何人都可以随时即时交易代币，而无需交易对手方。

> **提示**：如果仔细观察，你会发现 AMM 其实就是一个带有额外步骤、计算和逻辑的托管服务。因此，如果你错过了，请先完成 **Pinocchio 托管挑战**，然后再学习本课程。

### 核心功能

在本次挑战中，你将实现一个简单的 AMM，包含以下四个核心指令：

1. **初始化（Initialize）**：通过创建配置账户并铸造代表池中份额的 LP（流动性提供者）代币来设置 AMM。

2. **存入（Deposit）**：允许用户向池中提供 `token_x` 和 `token_y`。作为回报，他们将收到与其流动性份额成比例的 LP 代币。

3. **提取（Withdraw）**：允许用户赎回其 LP 代币，以提取其在池中的 `token_x` 和 `token_y` 份额，从而移除流动性。

4. **交换（Swap）**：允许任何人使用池来交易 `token_x` 和 `token_y`（或反之），并向流动性提供者支付少量费用。

> **注意**：如果你不熟悉 Pinocchio，建议先阅读 Pinocchio 简介，以熟悉我们将在本程序中使用的核心概念。

---

## AMM 核心概念

### 恒定乘积曲线

大多数 AMM 的核心是一个简单但强大的公式，称为**恒定乘积曲线**。该公式确保池中两种代币储备的乘积始终保持不变，即使用户进行交易或提供流动性。

#### 公式

最常见的 AMM 公式是：

```
x * y = k
```

其中：
- **x** = 池中代币 X 的数量
- **y** = 池中代币 Y 的数量
- **k** = 一个常数（永不改变）

每当有人将一种代币兑换为另一种代币时，池会调整储备，以确保乘积 `k` 保持不变。这会根据供需自动调整价格曲线。

#### 示例

假设池开始时有 **100 个代币 X** 和 **100 个代币 Y**：

```
100 * 100 = 10,000
```

如果用户想用 **10 个代币 X** 兑换代币 Y，池必须保持 `k = 10,000`。因此：

1. 存入后：`x_new = 110`
2. 求解 `y_new`：`110 * y_new = 10,000`
3. 因此：`y_new = 10,000 / 110 ≈ 90.91`

用户将收到：`100 - 90.91 = 9.09` 个代币 Y（扣除任何费用）。

### 流动性提供

当用户将两种代币存入池中时，他们成为**流动性提供者（LP）**。作为回报，他们会收到代表其池中份额的 **LP 代币**。

#### 工作原理

1. **LP 代币铸造**：LP 代币的铸造比例与您添加的流动性成正比。

2. **提取流动性**：当您提取时，您会销毁您的 LP 代币以取回您在两种代币中的份额（加上从兑换中收取的费用份额）。

3. **初始比例**：第一个流动性提供者设置初始比例。
   - 例如，如果您存入 **100 X** 和 **100 Y**，您可能会收到 **100 个 LP 代币**。

4. **后续存入**：之后，如果池中已经有 100 X 和 100 Y，而您再添加 10 X 和 10 Y：
   ```
   share = deposit_x / total_x = 10 / 100 = 10%
   ```
   因此 AMM 会向用户钱包铸造总 LP 供应量的 10%。

### 费用机制

每次交换通常会收取一小笔费用（例如 **0.3%**），该费用会添加到池中。

**好处**：
- LP 可以分享交易费用
- 随着时间的推移增加其 LP 代币的价值
- 激励人们提供流动性

---

## 项目设置

### 安装

让我们从创建一个全新的 Rust 环境开始：

```bash
# 创建工作空间
cargo new blueshift_native_amm --lib --edition 2021
cd blueshift_native_amm
```

### 添加依赖

添加 Pinocchio 相关的依赖包和恒定乘积曲线库：

```bash
# 添加核心依赖
cargo add pinocchio pinocchio-system pinocchio-token pinocchio-associated-token-account

# 添加 constant-product-curve（处理 AMM 计算）
cargo add --git="https://github.com/deanmlittle/constant-product-curve" constant-product-curve
```

### 配置 Cargo.toml

在 `Cargo.toml` 中声明 crate 类型，以便在 `target/deploy` 中生成部署工件：

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

现在您可以开始编写您的 AMM 程序了。

---

## 项目结构

这次我们将把程序拆分为小而集中的模块，而不是将所有内容塞入 `lib.rs` 中。文件夹结构大致如下：

```
src
├── instructions
│   ├── deposit.rs
│   ├── initialize.rs
│   ├── mod.rs
│   ├── swap.rs
│   └── withdraw.rs
├── lib.rs
└── state.rs
```

---

## 程序入口点

入口点位于 `lib.rs` 中，看起来总是一样的：

```rust
use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    program_error::ProgramError, 
    pubkey::Pubkey,
    ProgramResult,
};

entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

// 程序 ID: 22222222222222222222222222222222222222222222
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07, 
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07, 
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Initialize::DISCRIMINATOR, data)) => {
            Initialize::try_from((data, accounts))?.process()
        }
        Some((Deposit::DISCRIMINATOR, data)) => {
            Deposit::try_from((data, accounts))?.process()
        }
        Some((Withdraw::DISCRIMINATOR, data)) => {
            Withdraw::try_from((data, accounts))?.process()
        }
        Some((Swap::DISCRIMINATOR, data)) => {
            Swap::try_from((data, accounts))?.process()
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
```

---

## 状态管理（State）

我们将在 `state.rs` 中存储 AMM 的所有数据。

我们将其分为三个部分：结构定义、读取辅助函数和写入辅助函数。

### 结构定义

```rust
use core::mem::size_of;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
pub struct Config {
    state: u8,              // AMM 状态
    seed: [u8; 8],          // PDA 派生种子（u64）
    authority: Pubkey,      // 管理权限
    mint_x: Pubkey,         // 代币 X 的 Mint
    mint_y: Pubkey,         // 代币 Y 的 Mint
    fee: [u8; 2],           // 交换费用（u16，基点）
    config_bump: [u8; 1],   // PDA bump seed
}

#[repr(u8)]
pub enum AmmState {
    Uninitialized = 0u8,    // 未初始化
    Initialized = 1u8,      // 已初始化
    Disabled = 2u8,         // 已禁用
    WithdrawOnly = 3u8,     // 仅限提取
}

impl Config {
    pub const LEN: usize = size_of::<Config>();
    //...
}
```

#### 内存布局说明

`#[repr(C)]` 属性确保我们的结构具有可预测的、与 C 兼容的内存布局，在不同平台和 Rust 编译器版本之间保持一致。这对于链上程序至关重要，因为数据必须可靠地序列化和反序列化。

**为什么使用字节数组？**

我们将 `seed`（u64）和 `fee`（u16）存储为字节数组，而不是它们的原生类型，以确保安全的反序列化。当从账户存储中读取数据时，内存对齐没有保证，从未对齐的内存地址读取 u64 是未定义行为。通过使用字节数组并通过 `from_le_bytes()` 进行转换，我们确保数据可以安全读取，无论对齐情况如何，同时还保证在所有平台上始终使用一致的小端字节顺序。

#### 字段说明

- **state**：跟踪 AMM 的当前状态（例如，未初始化、已初始化、已禁用或仅限提取）
- **seed**：用于程序派生地址（PDA）生成的唯一值，允许多个 AMM 以不同配置共存
- **authority**：对 AMM 拥有管理控制权的公钥（例如，用于暂停或升级池）。可以通过传递 `[0u8; 32]` 将其设置为不可变
- **mint_x**：池中代币 X 的 SPL 代币铸造地址
- **mint_y**：池中代币 Y 的 SPL 代币铸造地址
- **fee**：以基点（1 基点 = 0.01%）表示的交换费用，在每次交易中收取并分配给流动性提供者
- **config_bump**：用于 PDA 派生的 bump 种子，确保配置账户地址有效且唯一。保存此值以提高 PDA 派生效率

`AmmState` 枚举定义了 AMM 的可能状态，使得管理池的生命周期并根据其状态限制某些操作变得更加容易。

### 读取辅助函数

读取辅助函数提供了对 `Config` 数据的安全、高效访问，并进行适当的验证和借用：

```rust
impl Config {
    //...
    
    /// 安全加载 Config（带借用检查）
    #[inline(always)]
    pub fn load(account_info: &AccountInfo) -> Result<Ref<Self>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner().ne(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes_unchecked(data)
        }))
    }
    
    /// 不安全加载 Config（无借用检查，性能更高）
    #[inline(always)]
    pub unsafe fn load_unchecked(account_info: &AccountInfo) -> Result<&Self, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self::from_bytes_unchecked(
            account_info.borrow_data_unchecked(),
        ))
    }
    
    /// 从字节数组返回 Config（不可变）
    ///
    /// # Safety
    ///
    /// 调用者必须确保 `bytes` 包含 `Config` 的有效表示，
    /// 并且它正确对齐以被解释为 `Config` 的实例。
    /// 目前 `Config` 的对齐为 1 字节。
    /// 此方法不执行长度验证。
    #[inline(always)]
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Config)
    }
    
    /// 从字节数组返回可变 Config 引用
    ///
    /// # Safety
    ///
    /// 调用者必须确保 `bytes` 包含 `Config` 的有效表示。
    #[inline(always)]
    pub unsafe fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut Self {
        &mut *(bytes.as_mut_ptr() as *mut Config)
    }
    
    // Getter 方法，用于安全字段访问
    #[inline(always)]
    pub fn state(&self) -> u8 { 
        self.state 
    }
    
    #[inline(always)]
    pub fn seed(&self) -> u64 { 
        u64::from_le_bytes(self.seed) 
    }
    
    #[inline(always)]
    pub fn authority(&self) -> &Pubkey { 
        &self.authority 
    }
    
    #[inline(always)]
    pub fn mint_x(&self) -> &Pubkey { 
        &self.mint_x 
    }
    
    #[inline(always)]
    pub fn mint_y(&self) -> &Pubkey { 
        &self.mint_y 
    }
    
    #[inline(always)]
    pub fn fee(&self) -> u16 { 
        u16::from_le_bytes(self.fee) 
    }
    
    #[inline(always)]
    pub fn config_bump(&self) -> [u8; 1] { 
        self.config_bump 
    }
}
```

#### 读取辅助函数的关键特性

- **安全借用**：`load` 方法返回一个 `Ref<Self>`，安全地管理从账户数据的借用，防止数据竞争并确保内存安全
- **验证**：`load` 和 `load_unchecked` 都会在允许访问结构之前验证账户数据的长度和所有者
- **获取方法**：所有字段都通过获取方法访问，这些方法处理从字节数组到其正确类型的转换（例如，`u64::from_le_bytes` 用于 `seed`）
- **性能**：`#[inline(always)]` 属性确保这些频繁调用的方法被内联以实现最佳性能

### 写入辅助函数

写入辅助函数提供了安全且经过验证的方法，用于修改 `Config` 数据：

```rust
impl Config {
    //...
    
    /// 加载可变 Config
    #[inline(always)]
    pub fn load_mut(account_info: &AccountInfo) -> Result<RefMut<Self>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner().ne(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(RefMut::map(account_info.try_borrow_mut_data()?, |data| unsafe {
            Self::from_bytes_unchecked_mut(data)
        }))
    }
    
    /// 设置 AMM 状态
    #[inline(always)]
    pub fn set_state(&mut self, state: u8) -> Result<(), ProgramError> {
        if state.ge(&(AmmState::WithdrawOnly as u8)) {
            return Err(ProgramError::InvalidAccountData);
        }
        self.state = state as u8;
        Ok(())
    }
    
    /// 设置费用
    #[inline(always)]
    pub fn set_fee(&mut self, fee: u16) -> Result<(), ProgramError> {
        if fee.ge(&10_000) {
            return Err(ProgramError::InvalidAccountData);
        }
        self.fee = fee.to_le_bytes();
        Ok(())
    }
    
    /// 批量设置所有字段
    #[inline(always)]
    pub fn set_inner(
        &mut self,
        seed: u64,
        authority: Pubkey,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee: u16,
        config_bump: [u8; 1],
    ) -> Result<(), ProgramError> {
        self.set_state(AmmState::Initialized as u8)?;
        self.set_seed(seed);
        self.set_authority(authority);
        self.set_mint_x(mint_x);
        self.set_mint_y(mint_y);
        self.set_fee(fee)?;
        self.set_config_bump(config_bump);
        Ok(())
    }
    
    /// 检查是否设置了权限
    #[inline(always)]
    pub fn has_authority(&self) -> Option<Pubkey> {
        let bytes = self.authority();
        let chunks: &[u64; 4] = unsafe { &*(bytes.as_ptr() as *const [u64; 4]) };
        if chunks.iter().any(|&x| x != 0) {
            Some(self.authority)
        } else {
            None
        }
    }
}
```

#### 写入辅助函数的主要功能

- **可变借用**：`load_mut` 方法返回一个 `RefMut<Self>`，安全地管理账户数据的可变借用
- **输入验证**：像 `set_state` 和 `set_fee` 这样的方法包含验证，以确保只存储有效值（例如，费用不能超过 10,000 个基点）
- **原子更新**：`set_inner` 方法允许高效地一次性原子更新所有结构字段，最大限度地减少状态不一致的风险
- **权限检查**：`has_authority` 方法提供了一种高效的方式来检查权限是否已设置（非零）或 AMM 是否不可变（全为零）
- **字节转换**：多字节值通过像 `to_le_bytes()` 这样的方法正确地转换为小端字节数组，以确保跨平台行为的一致性

---

## Initialize 指令（初始化）

`initialize` 指令执行两个主要任务：

1. 初始化 Config 账户，并存储 AMM 正常运行所需的所有信息
2. 创建 `mint_lp` 铸币账户，并将 `mint_authority` 分配给 `config` 账户

> **注意**：我们不会在这里初始化任何关联代币账户（ATAs），因为这通常是没有必要的，并且可能会浪费资源。在后续的 `deposit`、`withdraw` 和 `swap` 指令中，我们会检查代币是否存入了正确的 ATAs。然而，您应该在前端创建一个 "initializeAccount" 辅助工具，以按需生成这些账户。

### 所需账户

以下是此上下文所需的账户：

- **initializer**：config 账户的创建者。这不一定也必须是其权限持有者。必须是 signer 和 mutable，因为此账户将支付 config 和 mint_lp 的初始化费用
- **mint_lp**：代表池流动性的铸币账户。mint_authority 应设置为 config 账户。必须作为 mutable 传递
- **config**：正在初始化的配置账户。必须是 mutable
- **system 和 token 程序**：初始化上述账户所需的程序账户。必须是 executable

> **提示**：随着经验的积累，您会注意到许多这些检查可以省略，而依赖于 CPI 本身强制执行的约束。例如，对于此账户结构，不需要任何显式检查；如果不满足约束，程序将默认失败。

### 账户结构

由于与我们通常创建的结构体相比没有太多变化，我将把实现部分留给你：

```rust
pub struct InitializeAccounts<'a> {
    pub initializer: &'a AccountInfo,
    pub mint_lp: &'a AccountInfo,
    pub config: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        //..
    }
}
```

> **注意**：你需要传入上述讨论的所有账户，但并不是所有账户都需要包含在 `InitializeAccounts` 结构体中，因为在实现中你可能并不需要直接引用每个账户。

### 指令数据

以下是我们需要传入的指令数据：

- **seed**：用于 PDA（程序派生地址）种子推导的随机数。这允许创建唯一的池实例。必须是 `u64`
- **fee**：以基点表示的交换费（1 基点 = 0.01%）。此费用在每次交易中收取，并分配给流动性提供者。必须是 `u16`
- **mint_x**：池中代币 X 的 SPL 代币铸造地址。必须是 `[u8; 32]`
- **mint_y**：池中代币 Y 的 SPL 代币铸造地址。必须是 `[u8; 32]`
- **config_bump**：用于推导 config 账户 PDA 的 bump 种子。必须是 `u8`
- **lp_bump**：用于推导 lp_mint 账户 PDA 的 bump 种子。必须是 `u8`
- **authority**：将拥有 AMM 管理权限的公钥。如果未提供，池可以设置为不可变。必须是 `[u8; 32]`

> **说明**：如你所见，这些字段中的一些可以通过不同方式推导。例如，我们可以通过传入 Mint 账户并直接从中读取来获取 `mint_x`，或者在程序中直接生成 bump 值。然而，通过显式传递这些字段，我们的目标是创建一个最优化和高效的程序。

#### 指令数据结构

在此实现中，我们以比通常更灵活和底层的方式处理指令数据解析：

```rust
#[repr(C, packed)]
pub struct InitializeInstructionData {
    pub seed: u64,
    pub fee: u16,
    pub mint_x: [u8; 32],
    pub mint_y: [u8; 32],
    pub config_bump: [u8; 1],
    pub lp_bump: [u8; 1],
    pub authority: [u8; 32],
}

impl TryFrom<&[u8]> for InitializeInstructionData {
    type Error = ProgramError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        const INITIALIZE_DATA_LEN_WITH_AUTHORITY: usize = size_of::<InitializeInstructionData>();
        const INITIALIZE_DATA_LEN: usize =
            INITIALIZE_DATA_LEN_WITH_AUTHORITY - size_of::<[u8; 32]>();
        
        match data.len() {
            INITIALIZE_DATA_LEN_WITH_AUTHORITY => {
                Ok(unsafe { (data.as_ptr() as *const Self).read_unaligned() })
            }
            INITIALIZE_DATA_LEN => {
                // 如果未提供 authority，我们需要构建缓冲区并在末尾添加它
                let mut raw: MaybeUninit<[u8; INITIALIZE_DATA_LEN_WITH_AUTHORITY]> = 
                    MaybeUninit::uninit();
                let raw_ptr = raw.as_mut_ptr() as *mut u8;
                
                unsafe {
                    // 复制提供的数据
                    core::ptr::copy_nonoverlapping(
                        data.as_ptr(), 
                        raw_ptr, 
                        INITIALIZE_DATA_LEN
                    );
                    // 在缓冲区末尾添加 authority
                    core::ptr::write_bytes(raw_ptr.add(INITIALIZE_DATA_LEN), 0, 32);
                    // 现在转换为结构体
                    Ok((raw.as_ptr() as *const Self).read_unaligned())
                }
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
```

`InitializeInstructionData` 中的 `authority` 字段是可选的，可以省略以创建不可变池。

为了实现这一点并在创建不可变池时节省 32 字节的交易数据，我们会检查指令数据的长度并相应地解析数据：
- 如果数据较短，我们将 `authority` 字段设置为 `None`，通过向缓冲区末尾写入 32 个零字节
- 如果包含完整的 `authority` 字段，我们会将字节切片直接转换为结构体

### 指令逻辑

我们首先反序列化 `instruction_data` 和 `accounts`。

然后我们需要：

#### 1. 创建 Config 账户

使用系统程序中的 `CreateAccount` 指令和以下种子创建 Config 账户：

```rust
let seed_binding = self.instruction_data.seed.to_le_bytes();
let config_seeds = [
    Seed::from(b"config"),
    Seed::from(&seed_binding),
    Seed::from(&self.instruction_data.mint_x),
    Seed::from(&self.instruction_data.mint_y),
    Seed::from(&self.instruction_data.config_bump),
];
```

#### 2. 填充 Config 数据

使用 `Config::load_mut_unchecked()` 辅助工具加载 Config 账户并填充所需的所有数据，然后使用 `config.set_inner()` 辅助工具填充。

#### 3. 创建 LP Mint 账户

使用 `CreateAccount` 和 `InitializeMint2` 指令以及以下种子为 lp 创建 Mint 账户：

```rust
let mint_lp_seeds = [
    Seed::from(b"mint_lp"),
    Seed::from(self.accounts.config.key()),
    Seed::from(&self.instruction_data.lp_bump),
];
```

`mint_lp` 的 `mint_authority` 是 `config` 账户。

### 实现框架

你应该足够熟练，可以自行完成这一部分，因此我将实现留给你：

```rust
pub struct Initialize<'a> {
    pub accounts: InitializeAccounts<'a>,
    pub instruction_data: InitializeInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Initialize<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = InitializeAccounts::try_from(accounts)?;
        let instruction_data: InitializeInstructionData = InitializeInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Initialize<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
    
    pub fn process(&mut self) -> ProgramResult {
        //..
        Ok(())
    }
}
```

### 安全性说明

如前所述，这可能看起来不寻常，但我们不需要对传入的账户进行显式检查。

这是因为在实际操作中，如果有问题，指令会失败；要么在 CPI（跨程序调用）期间，要么通过我们在程序中设置的早期检查失败。

**例如**：
- 考虑 `initializer` 账户。我们期望它既是 signer 又是 mutable，但如果不是，`CreateAccount` 指令将会自动失败，因为它需要这些属性来满足 payer 的要求
- 同样地，如果传递的 `config` 账户具有无效的 `mint_x` 或 `mint_y`，任何尝试向协议中存入资金的操作都会在代币转移期间失败

随着经验的积累，您会发现可以省略许多检查，以保持指令的轻量化和优化，依赖系统和下游指令来强制执行约束。

---

## Deposit 指令（存款）

`deposit` 指令执行以下三个主要任务：

1. 根据用户希望 mint 的 LP 数量，存入 `mint_x` 和 `mint_y` 代币
2. 计算存款金额，并检查金额是否超过用户指定的 `max_x` 和 `max_y`
3. 在用户的 ATA 中铸造正确数量的 `mint_lp`

> **注意**：如 `initialize` 指令部分所述；为了优化，我们将所有 Associated Token Accounts 初始化在指令之外。

### 所需账户

以下是此上下文所需的账户：

- **user**：将代币存入 AMM 流动性的用户。必须是 signer
- **mint_lp**：代表池流动性的铸币账户。必须作为 mutable 传递
- **vault_x**：存储所有存入池中的 X 代币的代币账户。必须作为 mutable 传递
- **vault_y**：存储所有存入池中的 Y 代币的代币账户。必须作为 mutable 传递
- **user_x_ata**：用户的 X 代币关联账户。这是用户的 X 代币将从中转移到池中的源账户。必须作为 mutable 传递
- **user_y_ata**：用户的 Y 代币关联账户。这是用户的 Y 代币将从中转移到池中的源账户。必须作为 mutable 传递
- **user_lp_ata**：用户的 LP 代币关联账户。这是铸造 LP 代币的目标账户。必须作为 mutable 传递
- **config**：AMM 池的配置账户。存储所有相关的池参数和状态
- **token program**：SPL 代币程序账户。执行代币操作（如转账和铸造）所需。必须是 executable

### 账户结构

这里，我将再次把实现留给你：

```rust
pub struct DepositAccounts<'a> {
    pub user: &'a AccountInfo,
    pub mint_lp: &'a AccountInfo,
    pub vault_x: &'a AccountInfo,
    pub vault_y: &'a AccountInfo,
    pub user_x_ata: &'a AccountInfo,
    pub user_y_ata: &'a AccountInfo,
    pub user_lp_ata: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        //..
    }
}
```

### 指令数据

以下是我们需要传入的指令数据：

- **amount**：用户希望接收的 LP 代币数量。必须是 `u64`
- **max_x**：用户愿意存入的最大 Token X 数量。必须是 `u64`
- **max_y**：用户愿意存入的最大 Token Y 数量。必须是 `u64`
- **expiration**：此订单的过期时间。确保交易必须在一定时间内完成非常重要。必须是 `i64`

我们将以与初始化相同的方式处理 `DepositInstructionData` 的实现：

```rust
pub struct DepositInstructionData {
    pub amount: u64,
    pub max_x: u64,
    pub max_y: u64,
    pub expiration: i64,
}

impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        //..
    }
}
```

> **注意**：确保任何数量，例如 `amount`、`max_y` 和 `max_x` 都大于零，并且订单尚未过期，可以使用 Clock sysvar 进行检查。

### 指令逻辑

我们首先反序列化 `instruction_data` 和 `accounts`。

然后我们需要：

#### 1. 加载 Config 账户

加载 Config 账户以获取其中的所有数据。我们可以使用 `Config::load()` 辅助工具来完成。

#### 2. 验证 AMM 状态

验证 `AmmState` 是否有效（例如它是否等于 `AmmState::Initialized`）。

#### 3. 检查 Vault 派生

检查 `vault_x` 和 `vault_y` 的派生是否为关联代币账户（Associated Token Accounts），如下所示：

```rust
// 检查 vault_x 是否有效
let (vault_x, _) = find_program_address(
    &[
        self.accounts.config.key(),
        self.accounts.token_program.key(),
        config.mint_x(),
    ],
    &pinocchio_associated_token_account::ID,
);

if vault_x.ne(self.accounts.vault_x.key()) {
    return Err(ProgramError::InvalidAccountData);
}
```

#### 4. 计算存款金额

反序列化所有涉及的代币账户，并使用其中的数据通过 `constant-product-curve` crate 计算存款金额，并检查滑点：

```rust
// 反序列化代币账户
let mint_lp = unsafe { Mint::from_account_info_unchecked(self.accounts.mint_lp)? };
let vault_x = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_x)? };
let vault_y = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_y)? };

// 获取要存入的金额
let (x, y) = match mint_lp.supply() == 0 && vault_x.amount() == 0 && vault_y.amount() == 0 {
    true => (self.instruction_data.max_x, self.instruction_data.max_y),
    false => {
        let amounts = ConstantProduct::xy_deposit_amounts_from_l(
            vault_x.amount(),
            vault_y.amount(),
            mint_lp.supply(),
            self.instruction_data.amount,
            6,
        )
        .map_err(|_| ProgramError::InvalidArgument)?;
        (amounts.x, amounts.y)
    }
};

// 检查滑点
if !(x <= self.instruction_data.max_x && y <= self.instruction_data.max_y) {
    return Err(ProgramError::InvalidArgument);
}
```

> **说明**：如果是首次存款，我们可以跳过 LP 代币和存款的计算，直接采用用户建议的数值。

#### 5. 执行转账和铸造

将用户的代币账户中的金额转移到金库，并向用户的代币账户铸造相应数量的 LP 代币。

### 实现框架

你应该已经足够熟练可以独立完成这部分内容：

```rust
pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;
    
    pub fn process(&mut self) -> ProgramResult {
        //..
        Ok(())
    }
}
```

---

## Withdraw 指令（提现）

`withdraw` 指令主要执行以下三项任务：

1. 根据用户希望 burn 的 LP 数量，提取 `mint_x` 和 `mint_y` 代币
2. 计算提取金额，并检查金额是否不低于用户指定的 `mint_x` 和 `mint_y`
3. 从用户的 ATA 中销毁相应数量的 `mint_lp`

> **注意**：如 `initialize` 指令部分所述；为了优化目的，我们将所有 Associated Token Accounts 初始化在指令之外。

### 所需账户

以下是此上下文所需的账户：

- **user**：将代币提取到 AMM 流动性中的用户。必须是 signer
- **mint_lp**：表示池流动性的 Mint 账户。必须作为 mutable 传递
- **vault_x**：存储所有存入池中的 X 代币的代币账户。必须作为 mutable 传递
- **vault_y**：存储所有存入池中的 Y 代币的代币账户。必须作为 mutable 传递
- **user_x_ata**：用户的 X 代币关联账户。这是用户的 X 代币将从池中转移到的目标账户。必须作为 mutable 传递
- **user_y_ata**：用户的 Y 代币关联账户。这是用户的 Y 代币将从池中转移到的目标账户。必须作为 mutable 传递
- **user_lp_ata**：用户的 LP 代币关联账户。这是 LP 代币将被销毁的来源账户。必须作为 mutable 传递
- **config**：AMM 池的配置账户。存储所有相关的池参数和状态
- **token program**：SPL 代币程序账户。这是执行代币操作（如转账和铸造）所需的。必须是 executable

### 账户结构

```rust
pub struct WithdrawAccounts<'a> {
    pub user: &'a AccountInfo,
    pub mint_lp: &'a AccountInfo,
    pub vault_x: &'a AccountInfo,
    pub vault_y: &'a AccountInfo,
    pub user_x_ata: &'a AccountInfo,
    pub user_y_ata: &'a AccountInfo,
    pub user_lp_ata: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        //..
    }
}
```

### 指令数据

以下是我们需要传入的指令数据：

- **amount**：用户希望销毁的 LP 代币数量。必须是 `u64`
- **min_x**：用户愿意提取的最小 Token X 数量。必须是 `u64`
- **min_y**：用户愿意提取的最小 Token Y 数量。必须是 `u64`
- **expiration**：此订单的过期时间。确保交易必须在一定时间内完成非常重要。必须是 `i64`

```rust
pub struct WithdrawInstructionData {
    pub amount: u64,
    pub min_x: u64,
    pub min_y: u64,
    pub expiration: i64,
}

impl<'a> TryFrom<&'a [u8]> for WithdrawInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        //..
    }
}
```

> **注意**：确保任何数量，例如 `amount`、`min_y` 和 `min_x` 都大于零，并且订单尚未使用 Clock sysvar 过期。

### 指令逻辑

我们首先反序列化 `instruction_data` 和 `accounts`。

然后我们需要：

#### 1. 加载 Config 账户

使用 `Config::load()` 辅助工具加载配置。

#### 2. 验证 AMM 状态

验证 `AmmState` 是否有效（即它不等于 `AmmState::Disabled`）。

#### 3. 检查 Vault 派生

检查 `vault_x` 和 `vault_y` 的派生是否为关联代币账户。

#### 4. 计算提取金额

反序列化所有涉及的代币账户，并使用其中的数据通过 `constant-product-curve` crate 计算提取的数量，并检查滑点：

```rust
let mint_lp = unsafe { Mint::from_account_info_unchecked(self.accounts.mint_lp)? };
let vault_x = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_x)? };
let vault_y = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_y)? };

let (x, y) = match mint_lp.supply() == self.instruction_data.amount {
    true => (vault_x.amount(), vault_y.amount()),
    false => {
        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            vault_x.amount(),
            vault_y.amount(),
            mint_lp.supply(),
            self.instruction_data.amount,
            6,
        )
        .map_err(|_| ProgramError::InvalidArgument)?;
        (amounts.x, amounts.y)
    }
};

// 检查滑点
if !(x >= self.instruction_data.min_x && y >= self.instruction_data.min_y) {
    return Err(ProgramError::InvalidArgument);
}
```

#### 5. 执行转账和销毁

将金额从金库转移到用户的代币账户，并从用户的代币账户中销毁相应数量的 LP 代币。

> **注意**：`vault_x` 和 `vault_y` 的 authority 是 `config` 账户。

### 实现框架

```rust
pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
    pub instruction_data: WithdrawInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Withdraw<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = WithdrawAccounts::try_from(accounts)?;
        let instruction_data = WithdrawInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;
    
    pub fn process(&mut self) -> ProgramResult {
        //..
        Ok(())
    }
}
```

---

## Swap 指令（交换）

`swap` 指令主要执行两个任务：

1. 计算通过将一定数量的 `mint_y` 发送到 AMM（或反之）后，能够接收到的 `mint_x` 的数量，包括手续费
2. 将 `from` 代币转移到金库，并将 `to` 代币转移到用户的代币账户

> **注意**：如 `initialize` 指令部分所述；为了优化，我们将在指令外部初始化所有 Associated Token Accounts。

### 所需账户

以下是此上下文所需的账户：

- **user**：将代币交换到 AMM 流动性中的用户。必须是 signer
- **user_x_ata**：用户的代币 X 关联账户。此账户将接收或发送代币 X 到池中。必须作为 mutable 传递
- **user_y_ata**：用户的代币 Y 关联账户。此账户将接收或发送代币 Y 到池中。必须作为 mutable 传递
- **vault_x**：持有所有存入池中的代币 X 的代币账户。必须作为 mutable 传递
- **vault_y**：持有所有存入池中的代币 Y 的代币账户。必须作为 mutable 传递
- **config**：AMM 池的配置账户。存储所有相关的池参数和状态
- **token program**：SPL 代币程序账户。执行代币操作（如转账和铸造）所需。必须是 executable

### 账户结构

```rust
pub struct SwapAccounts<'a> {
    pub user: &'a AccountInfo,
    pub user_x_ata: &'a AccountInfo,
    pub user_y_ata: &'a AccountInfo,
    pub vault_x: &'a AccountInfo,
    pub vault_y: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for SwapAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        //..
    }
}
```

### 指令数据

以下是我们需要传递的指令数据：

- **is_x**：此交换是从代币 X 到代币 Y 或反之进行的；需要正确对齐账户。必须是 `bool` (u8)
- **amount**：用户愿意用来交换另一种代币的代币数量。必须是 `u64`
- **min**：用户愿意在交换 `amount` 时接收的最小代币数量。必须是 `u64`
- **expiration**：此订单的过期时间。确保交易必须在一定时间内完成非常重要。必须是 `i64`

```rust
pub struct SwapInstructionData {
    pub is_x: bool,
    pub amount: u64,
    pub min: u64,
    pub expiration: i64,
}

impl<'a> TryFrom<&'a [u8]> for SwapInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        //..
    }
}
```

> **注意**：确保任何数量，例如 `amount` 和 `min` 都大于零，并且使用 Clock 系统变量检查订单尚未过期。

### 指令逻辑

我们首先反序列化 `instruction_data` 和 `accounts`。

然后我们需要：

#### 1. 加载 Config 账户

使用 `Config::load()` 辅助工具加载配置。

#### 2. 验证 AMM 状态

验证 `AmmState` 是否有效（例如它是否等于 `AmmState::Initialized`）。

#### 3. 检查 Vault 派生

检查 `vault_x` 和 `vault_y` 的派生是否为关联代币账户。

#### 4. 计算交换金额

反序列化所有涉及的代币账户，并使用其中的数据通过 `constant-product-curve` crate 计算交换数量，并检查滑点：

```rust
// 反序列化代币账户
let vault_x = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_x)? };
let vault_y = unsafe { TokenAccount::from_account_info_unchecked(self.accounts.vault_y)? };

// 交换计算
let mut curve = ConstantProduct::init(
    vault_x.amount(),
    vault_y.amount(),
    vault_x.amount(),
    config.fee(),
    None,
)
.map_err(|_| ProgramError::Custom(1))?;

let p = match self.instruction_data.is_x {
    true => LiquidityPair::X,
    false => LiquidityPair::Y,
};

let swap_result = curve
    .swap(p, self.instruction_data.amount, self.instruction_data.min)
    .map_err(|_| ProgramError::Custom(1))?;

// 检查正确的值
if swap_result.deposit == 0 || swap_result.withdraw == 0 {
    return Err(ProgramError::InvalidArgument);
}
```

#### 5. 执行转账

创建转账逻辑，检查 `is_x` 值，并将 `from` 金额转入金库，将 `to` 金额转入用户的代币账户：

```rust
if self.instruction_data.is_x {
    Transfer {
        //...
    }
    .invoke()?;
    
    Transfer {
        //...
    }
    .invoke_signed(&signer_seeds)?;
} else {
    Transfer {
        //...
    }
    .invoke()?;
    
    Transfer {
        //...
    }
    .invoke_signed(&signer_seeds)?;
}
```

### 实现框架

```rust
pub struct Swap<'a> {
    pub accounts: SwapAccounts<'a>,
    pub instruction_data: SwapInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Swap<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = SwapAccounts::try_from(accounts)?;
        let instruction_data = SwapInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Swap<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3;
    
    pub fn process(&mut self) -> ProgramResult {
        //..
        Ok(())
    }
}
```

---

## 测试和部署

### 构建程序

现在，您可以通过我们的单元测试来测试您的程序并领取您的 NFT！

首先，在终端中使用以下命令构建您的程序：

```bash
cargo build-sbf
```

这将在您的 `target/deploy` 文件夹中直接生成一个 `.so` 文件。

### 提交挑战

现在点击 **Take Challenge** 按钮并将文件拖放到那里！

---

## 总结

通过本教程，您已经学会了：

- ✅ 理解 AMM 的工作原理和恒定乘积曲线
- ✅ 使用 Pinocchio 框架构建高性能 Solana 程序
- ✅ 实现完整的 AMM 功能（初始化、存款、提取、交换）
- ✅ 处理流动性提供和 LP 代币管理
- ✅ 使用 `constant-product-curve` 库进行复杂计算
- ✅ 优化账户验证和 CPI 调用
- ✅ 实现滑点保护和过期检查

**准备接受挑战了吗？** 🚀

---

## 参考资料

- [Pinocchio 文档](https://docs.rs/pinocchio/)
- [Constant Product Curve](https://github.com/deanmlittle/constant-product-curve)
- [Solana 程序库](https://github.com/solana-labs/solana-program-library)
- [AMM 原理](https://academy.binance.com/en/articles/what-is-an-automated-market-maker-amm)
