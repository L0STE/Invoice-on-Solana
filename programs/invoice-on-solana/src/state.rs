use anchor_lang::prelude::*;

#[account]
pub struct Project {
    pub owner: Pubkey,
    pub project_name: Vec<u8>,
    pub balance: u8,
}

impl Project {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the owner
        1 +     //  Project Name (u8)
        1       //  Balance (u8)
    }
}

#[account]
pub struct Employee {
    pub project: Pubkey,
    pub employee: Pubkey,
    pub title: Vec<u8>,
    pub day_worked: u8,
    pub money_earned: u8,
    pub has_accepted: bool
}

impl Employee {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the project PDA
        32 +    //  Wallet address of the Employee
        1 +     //  Title (u8)
        1 +     //  Working Day (u8)
        1 +     //  Earned Money (u8)
        1       //  Has Accepted (bool)
    }
}

//Da cazzo cambiare i valori perchè u8 costano troppo
#[account]
pub struct Invoice {
    pub project: Pubkey,
    pub employee_wallet: Pubkey,
    pub from: u8,
    pub to: u8,
    pub amount: u8,
    pub has_claimed: bool,
}

impl Invoice {
    pub fn space() -> usize {
        8 +     //  Discriminator
        32 +    //  Wallet address of the project PDA
        32 +    //  Wallet address of the Employee
        1 +     //  From (u8)
        1 +     //  To (u8)
        1 +     //  Amount Due (u8)
        1       //  Has Claimed (bool)
    }
}