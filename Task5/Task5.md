# Pinocchio 托管（Escrow）

## 项目简介

托管是一种强大的金融工具，可以在两方之间实现安全的代币交换。

可以将其视为一个数字保险箱，其中一位用户可以锁定代币 A，等待另一位用户存入代币 B，然后完成交换。

这创造了一个无需信任的环境，双方都不需要担心对方会退出交易。

### 核心功能

在本次挑战中，我们将通过三个简单但强大的指令来实现这一概念：

1. **创建（Make）**：创建者（第一位用户）定义交易条款，并将约定数量的代币 A 存入一个安全的保险库。这就像将您的物品放入保险箱并设置交换条款。

2. **接受（Take）**：接受者（第二位用户）通过将承诺的代币 B 转移给创建者来接受报价，并作为回报，获得锁定的代币 A。这是双方完成各自交易的一刻。

3. **退款（Refund）**：如果创建者改变主意或未找到合适的接受者，他们可以取消报价并取回代币 A。这就像在交易失败时从保险箱中取回您的物品。

> **注意**：如果您不熟悉 Pinocchio，建议先阅读 Pinocchio 简介，以熟悉我们将在本程序中使用的核心概念。

---

## 项目设置

### 安装

让我们从创建一个全新的 Rust 环境开始：

```bash
# 创建工作空间
cargo new blueshift_escrow --lib --edition 2021
cd blueshift_escrow
```

### 添加依赖

添加 Pinocchio 相关的依赖包：

```bash
cargo add pinocchio pinocchio-system pinocchio-token pinocchio-associated-token-account
```

### 配置 Cargo.toml

在 `Cargo.toml` 中声明 crate 类型，以便在 `target/deploy` 中生成部署工件：

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

现在您可以开始编写您的托管程序了。

---

## 项目结构

这次我们将把程序分成小而集中的模块，而不是将所有内容都塞进 `lib.rs` 中。文件夹结构大致如下：

```
src
├── instructions
│   ├── make.rs
│   ├── helpers.rs
│   ├── mod.rs
│   ├── refund.rs
│   └── take.rs
├── errors.rs
├── lib.rs
└── state.rs
```

---

## 程序入口点

入口点位于 `lib.rs` 中，与我们在上一课中所做的非常相似：

```rust
use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    program_error::ProgramError, 
    pubkey::Pubkey, 
    ProgramResult
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
        Some((Make::DISCRIMINATOR, data)) => Make::try_from((data, accounts))?.process(),
        Some((Take::DISCRIMINATOR, _)) => Take::try_from(accounts)?.process(),
        Some((Refund::DISCRIMINATOR, _)) => Refund::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}
```

---

## 状态管理（State）

我们将在 `state.rs` 中存储 Escrow 的所有数据。我们将其分为两部分：结构体定义及其实现。

### 结构体定义

```rust
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use core::mem::size_of;

#[repr(C)]
pub struct Escrow {
    pub seed: u64,        // 用于 PDA 派生的随机数
    pub maker: Pubkey,    // 创建者地址
    pub mint_a: Pubkey,   // 存入的代币
    pub mint_b: Pubkey,   // 请求的代币
    pub receive: u64,     // 希望接收的代币 B 数量
    pub bump: [u8; 1]     // PDA bump seed
}
```

#### 字段说明

`#[repr(C)]` 属性确保我们的结构体具有可预测的内存布局，这对于链上数据至关重要。每个字段都有特定的用途：

- **seed**：一个随机数，允许一个创建者使用相同的代币对创建多个托管
- **maker**：创建托管并将接收代币的钱包地址
- **mint_a**：存入代币的 SPL 代币铸造地址
- **mint_b**：请求代币的 SPL 代币铸造地址
- **receive**：创建者希望接收的代币 B 的确切数量
- **bump**：在 PDA 推导中使用的单字节，用于确保地址不在 Ed25519 曲线上

### 实现方法

```rust
impl Escrow {
    /// 精确计算账户大小
    pub const LEN: usize = size_of::<u64>() 
        + size_of::<Pubkey>() 
        + size_of::<Pubkey>() 
        + size_of::<Pubkey>() 
        + size_of::<u64>()
        + size_of::<[u8; 1]>();
    
    /// 从字节数组加载 Escrow（可变）
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { 
            &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) 
        })
    }
    
    /// 从字节数组加载 Escrow（不可变）
    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { 
            &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) 
        })
    }
    
    /// 设置单个字段的方法
    #[inline(always)]
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
    
    #[inline(always)]
    pub fn set_maker(&mut self, maker: Pubkey) {
        self.maker = maker;
    }
    
    #[inline(always)]
    pub fn set_mint_a(&mut self, mint_a: Pubkey) {
        self.mint_a = mint_a;
    }
    
    #[inline(always)]
    pub fn set_mint_b(&mut self, mint_b: Pubkey) {
        self.mint_b = mint_b;
    }
    
    #[inline(always)]
    pub fn set_receive(&mut self, receive: u64) {
        self.receive = receive;
    }
    
    #[inline(always)]
    pub fn set_bump(&mut self, bump: [u8; 1]) {
        self.bump = bump;
    }
    
    /// 批量设置所有字段
    #[inline(always)]
    pub fn set_inner(
        &mut self, 
        seed: u64, 
        maker: Pubkey, 
        mint_a: Pubkey, 
        mint_b: Pubkey, 
        receive: u64, 
        bump: [u8; 1]
    ) {
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.bump = bump;
    }
}
```

#### 实现特点

该实现提供了几个关键功能：

- **精确的大小计算**：`LEN` 通过汇总每个字段的大小，精确计算账户大小
- **安全加载**：`load` 提供了一种安全的方式来加载和验证托管数据
- **性能优化**：
  - 在 getter 上使用 `#[inline(always)]` 以实现最大性能
  - 当我们确定借用是安全时，使用 unsafe 方法
  - 使用 `set_inner` 高效地设置字段
- **内存安全**：对账户数据长度和所有权进行适当验证

此实现确保我们的托管状态既安全又高效，在适当的地方进行了适当的验证和性能优化。

---

## Make 指令（创建托管）

`make` 指令完成以下三项工作：

1. 初始化托管记录并存储所有交易条款
2. 创建金库（一个由 `mint_a` 拥有的 `escrow` 的关联代币账户 ATA）
3. 使用 CPI 调用 SPL-Token 程序，将创建者的 Token A 转移到该金库中

### 所需账户

以下是上下文所需的账户：

- **maker**: 托管的创建者。必须是签名者且可变
- **escrow**: 我们正在初始化的托管账户。必须是可变的
- **mint_a**: 我们存入托管的代币
- **mint_b**: 我们希望接收的代币
- **maker_ata_a**: 由创建者拥有的关联代币账户。必须是可变的
- **vault**: 由托管拥有的关联代币账户。必须是可变的
- **system_program**: 系统程序。必须是可执行的
- **token_program**: 代币程序。必须是可执行的

> **注意**：我们将使用 Pinocchio 简介中介绍的类型。

### 账户结构

在代码中，这看起来像这样：

```rust
pub struct MakeAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for MakeAccounts<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, escrow, mint_a, mint_b, maker_ata_a, vault, 
             system_program, token_program, _] = accounts 
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        
        // 基本账户检查
        SignerAccount::check(maker)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(maker_ata_a, maker, mint_a, token_program)?;
        
        // 返回账户
        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}
```

### 指令数据

以下是我们需要传入的指令数据：

- **seed**: 在种子派生过程中使用的随机数。必须是 `u64`
- **receive**: 创建者希望接收的金额。必须是 `u64`
- **amount**: 创建者希望存入的金额。必须是 `u64`

我们将检查 `amount` 是否为零，因为这对于托管来说没有意义。

```rust
pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for MakeInstructionData {
    type Error = ProgramError;
    
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }
        
        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());
        
        // 指令检查
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }
        
        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}
```

### 指令逻辑

我们首先在 `TryFrom` 实现中初始化所需的账户，在此之前我们已经反序列化了 `instruction_data` 和 `accounts`。

在此步骤中，我们使用从 Pinocchio 简介中介绍的辅助函数的 `ProgramAccount::init::<Escrow>` 特性创建 Escrow 账户。同样，我们初始化 Vault 账户，因为它需要重新创建：

```rust
pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Make<'a> {
    type Error = ProgramError;
    
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = MakeAccounts::try_from(accounts)?;
        let instruction_data = MakeInstructionData::try_from(data)?;
        
        // 初始化所需账户
        let (_, bump) = find_program_address(
            &[b"escrow", accounts.maker.key(), &instruction_data.seed.to_le_bytes()], 
            &crate::ID
        );
        
        let seed_binding = instruction_data.seed.to_le_bytes();
        let bump_binding = [bump];
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        
        ProgramAccount::init::<Escrow>(
            accounts.maker,
            accounts.escrow,
            &escrow_seeds,
            Escrow::LEN
        )?;
        
        // 初始化 vault
        AssociatedTokenAccount::init(
            accounts.vault,
            accounts.mint_a,
            accounts.maker,
            accounts.escrow,
            accounts.system_program,
            accounts.token_program,
        )?;
        
        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }
}
```

现在我们可以专注于逻辑本身，这将仅是填充托管账户，然后将代币转移到 Vault。

```rust
impl<'a> Make<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
    
    pub fn process(&mut self) -> ProgramResult {
        // 填充 escrow 账户
        let mut data = self.accounts.escrow.try_borrow_mut_data()?;
        let escrow = Escrow::load_mut(data.as_mut())?;
        
        escrow.set_inner(
            self.instruction_data.seed,
            *self.accounts.maker.key(),
            *self.accounts.mint_a.key(),
            *self.accounts.mint_b.key(),
            self.instruction_data.receive,
            [self.bump],
        );
        
        // 转移代币到 vault
        Transfer {
            from: self.accounts.maker_ata_a,
            to: self.accounts.vault,
            authority: self.accounts.maker,
            amount: self.instruction_data.amount
        }.invoke()?;
        
        Ok(())
    }
}
```

---

## Take 指令（接受托管）

`take` 指令完成交换操作：

1. 关闭托管记录，将其租金 lamports 返还给创建者
2. 将 Token A 从保管库转移到接受者，然后关闭保管库
3. 将约定数量的 Token B 从接受者转移到创建者

### 所需账户

以下是上下文所需的账户：

- **taker**：希望接受交易的人。必须是签名者且可变
- **maker**：托管的创建者。必须可变
- **escrow**：我们正在初始化的托管账户。必须可变
- **mint_a**：我们存入托管的代币
- **mint_b**：我们希望接收的代币
- **vault**：由托管拥有的关联代币账户。必须可变
- **taker_ata_a**：由接受者拥有的 `mint_a` 的关联代币账户。必须可变
- **taker_ata_b**：由接受者拥有的 `mint_b` 的关联代币账户。必须可变
- **maker_ata_b**：由创建者拥有的 `mint_b` 的关联代币账户。必须可变
- **system_program**：系统程序。必须可执行
- **token_program**：代币程序。必须可执行

### 账户结构

```rust
pub struct TakeAccounts<'a> {
    pub taker: &'a AccountInfo,
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub taker_ata_a: &'a AccountInfo,
    pub taker_ata_b: &'a AccountInfo,
    pub maker_ata_b: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for TakeAccounts<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [taker, maker, escrow, mint_a, mint_b, vault, 
             taker_ata_a, taker_ata_b, maker_ata_b, 
             system_program, token_program, _] = accounts 
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        
        // 基本账户检查
        SignerAccount::check(taker)?;
        ProgramAccount::check(escrow)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(taker_ata_b, taker, mint_b, token_program)?;
        AssociatedTokenAccount::check(vault, escrow, mint_a, token_program)?;
        
        // 返回账户
        Ok(Self {
            taker,
            maker,
            escrow,
            mint_a,
            mint_b,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            vault,
            system_program,
            token_program,
        })
    }
}
```

### 指令数据

执行逻辑所需的所有数据已经存在于托管账户或我们反序列化的账户中。因此，对于此指令，我们不需要任何 `instruction_data`。

### 指令逻辑

我们通过在 `TryFrom` 实现中初始化所需账户开始操作，在此之前我们已经对账户进行了反序列化。

在此步骤中，我们使用 `AssociatedTokenAccount::init_if_needed` 确保接收方的 Token A 账户和提供方的 Token B 账户均已初始化，因为我们无法确定它们是否已经存在。

```rust
pub struct Take<'a> {
    pub accounts: TakeAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Take<'a> {
    type Error = ProgramError;
    
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = TakeAccounts::try_from(accounts)?;
        
        // 初始化必要的账户
        AssociatedTokenAccount::init_if_needed(
            accounts.taker_ata_a,
            accounts.mint_a,
            accounts.taker,
            accounts.taker,
            accounts.system_program,
            accounts.token_program,
        )?;
        
        AssociatedTokenAccount::init_if_needed(
            accounts.maker_ata_b,
            accounts.mint_b,
            accounts.taker,
            accounts.maker,
            accounts.system_program,
            accounts.token_program,
        )?;
        
        Ok(Self {
            accounts,
        })
    }
}
```

现在我们可以专注于以下逻辑：

1. 将代币从 `taker_ata_b` 转移到 `maker_ata_b`
2. 将代币从 `vault` 转移到 `taker_ata_a`
3. 关闭现在已为空的 `vault` 并提取账户中的租金

```rust
impl<'a> Take<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;
    
    pub fn process(&mut self) -> ProgramResult {
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;
        
        // 检查 escrow 是否有效
        let escrow_key = create_program_address(
            &[b"escrow", self.accounts.maker.key(), &escrow.seed.to_le_bytes(), &escrow.bump], 
            &crate::ID
        )?;
        
        if &escrow_key != self.accounts.escrow.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = escrow.bump;
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        let signer = Signer::from(&escrow_seeds);
        
        let amount = TokenAccount::from_account_info(self.accounts.vault)?.amount();
        
        // 从 Vault 转移到 Taker
        Transfer {
            from: self.accounts.vault,
            to: self.accounts.taker_ata_a,
            authority: self.accounts.escrow,
            amount,
        }.invoke_signed(&[signer.clone()])?;
        
        // 关闭 Vault
        CloseAccount {
            account: self.accounts.vault,
            destination: self.accounts.maker,
            authority: self.accounts.escrow,
        }.invoke_signed(&[signer.clone()])?;
        
        // 从 Taker 转移到 Maker
        Transfer {
            from: self.accounts.taker_ata_b,
            to: self.accounts.maker_ata_b,
            authority: self.accounts.taker,
            amount: escrow.receive,
        }.invoke()?;
        
        // 关闭 Escrow
        drop(data);
        ProgramAccount::close(self.accounts.escrow, self.accounts.taker)?;
        
        Ok(())
    }
}
```

---

## Refund 指令（退款）

`refund` 指令允许创建者取消一个未完成的报价：

1. 关闭托管 PDA，并将其租金 lamports 返还给创建者
2. 将代币 A 的全部余额从保险库转回创建者，然后关闭保险库账户

### 所需账户

以下是所需的账户：

- **maker**：托管的创建者。必须是签名者且可变
- **escrow**：我们正在初始化的托管账户。必须是可变的
- **mint_a**：我们存入托管的代币
- **vault**：由托管拥有的关联代币账户。必须是可变的
- **maker_ata_a**：由创建者拥有的关联代币账户。必须是可变的
- **system_program**：系统程序。必须是可执行的
- **token_program**：代币程序。必须是可执行的

> **提示**：我们将让您创建自己的账户结构，因为现在您应该已经熟悉了这个过程。

### 指令数据

我们需要执行逻辑的所有数据已经存在于托管账户或我们正在反序列化的账户中。因此，对于此指令，我们不需要任何 `instruction_data`。

### 指令逻辑

在这里，我们将再次让您自由发挥，创建自己的逻辑！不要害怕回到前面的章节查看我们如何实现创建和接受指令，或者如果遇到困难，可以在 Discord 上寻求帮助。

---

## 测试和部署

### 构建程序

现在，您可以使用我们的单元测试来测试您的程序并领取您的 NFT！

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

- ✅ 使用 Pinocchio 框架构建 Solana 程序
- ✅ 实现无需信任的代币托管交换
- ✅ 处理 PDA（Program Derived Address）
- ✅ 执行 CPI（Cross-Program Invocation）调用
- ✅ 管理关联代币账户（ATA）
- ✅ 实现安全的账户验证和错误处理

**准备接受挑战了吗？** 🚀

---

## 参考资料

- [Pinocchio 文档](https://docs.rs/pinocchio/)
- [Solana 程序库](https://github.com/solana-labs/solana-program-library)
- [SPL Token 文档](https://spl.solana.com/token)
