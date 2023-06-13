use anchor_lang::prelude::*;

#[account]
pub struct Project {
    pub owner: Pubkey,
    pub project_name: String,
    pub balance: u64,
    pub monthly_spending: u64,
    pub project_bump: u8,
}

impl Project {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the owner
        4 +     //  Project Name
        8 +     //  Balance (u64)
        8 +     //  Monthly Spending (u64)
        1       //  Bump (u8)
    }
}

#[account]
pub struct Employee {
    pub project: Pubkey,
    pub employee: Pubkey,
    pub employee_title: String,
    pub monthly_pay: u64,
    pub is_active: bool,
    pub employee_bump: u8,
}

impl Employee {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the project PDA
        32 +    //  Wallet address of the Employee
        4 +     //  Title 
        8 +     //  Monthly Pay (u64)
        1 +     //  Is Active (bool)
        1       //  Bump (u8)
    }
}

#[account]
pub struct Invoice {
    pub project: Pubkey,
    pub employee: Pubkey,
    pub from: i64,
    pub to: i64,
    pub balance: u64,
    pub has_claimed: bool,
    pub invoice_bump: u8
}

impl Invoice {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the project PDA
        32 +    //  Wallet address of the Employee
        8 +     //  From (u64)
        8 +     //  To (u64)
        8 +     //  Amount Due (u64)
        1 +     //  Has Claimed (bool)
        1       //  Invoice Bump (u8)
    }
}