use anchor_lang::prelude::*;

use crate::state::*;

///////////////////////////////////////////////////////////////////////////////////////////

//PROJECT

#[derive(Accounts)]
pub struct  CreateProject<'info> {
    #[account(
        init,
        payer = owner,
        space = 200
    )]
    pub project_state: Box<Account<'info, Project>>,
    #[account(seeds = [b"project", project_state.key().as_ref()], bump)]
    ///CHECK
    pub project: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/* 
#[derive(Accounts)]
#[instruction(amount: u8)]
pub struct  ProjectDeposit<'info> {
    #[account(
        mut,
        constraint = amount > 0,
        constraint = *owner.key == project_pda.owner,
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u8)]
pub struct  ProjectWithdraw<'info> {
    #[account(
        mut,
        constraint = amount - project_pda.balance >= 0,
        constraint = *owner.key == project_pda.owner,
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


/*  ToDo          
#[derive(Accounts)] //HOW TO DO THAT
pub struct  ChangeProjectManager<'info> {
    #[account(mut)]
    pub project_manager: Signer<'info>,
    #[account(init, payer = project_manager, space = 8 + 32 + 1 + 1 + 1)]
    pub project_state: Account<'info, ProjectState>,
    #[account(seeds = [b"project", project_state.key().as_ref()], bump)]
    pub project: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
*/ 

#[derive(Accounts)]
pub struct ProjectChangeName <'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        constraint = *owner.key == project_pda.owner,
    )]
    pub project_pda: Box<Account<'info, Project>>,

    pub system_program: Program<'info, System>,
}

///////////////////////////////////////////////////////////////////////////////////////////

//EMPLOYEE

#[derive(Accounts)]
pub struct  CreateEmployee<'info> {
    #[account(
        seeds = [
            b"employee",
            project_pda.key().as_ref(),
        ],
        bump,
        init,
        payer = owner,
        space = Employee::space()
    )]
    pub employee_pda: Box<Account<'info, Employee>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  EmployeeChangeWallet<'info> {
    #[account(
        mut,
        constraint = project_pda.key() == employee_pda.project,
    )]
    pub employee_pda: Box<Account<'info, Employee>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  EmployeeChangeTitle<'info> {
    #[account(
        mut,
        constraint = project_pda.key() == employee_pda.project,
    )]
    pub employee_pda: Box<Account<'info, Employee>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}

//ToDo EMPLOYEE ACCEPT

///////////////////////////////////////////////////////////////////////////////////////////

//INVOICE

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct CreateInvoice<'info> {
    #[account(
        seeds = [
            b"invoice",
            project_pda.key().as_ref(),
        ],
        bump,
        init,
        payer = owner,
        space = Invoice::space(), 
        constraint = amount > 0
    )]
    pub invoice_pda: Box<Account<'info, Invoice>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(time: u8, amount: u8)]
pub struct InvoiceChangeAmount<'info> {
    #[account(
        mut,
        constraint = invoice_pda.project == project_pda.key(),
        constraint = time < invoice_pda.from,
        constraint = amount > 0,
    )]
    pub invoice_pda: Box<Account<'info, Invoice>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,
    
    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(time: u8)]
pub struct InvoiceChangeDate<'info> {
    #[account(
        mut,
        constraint = invoice_pda.project == project_pda.key(),
        constraint = time < invoice_pda.from,
    )]
    pub invoice_pda: Box<Account<'info, Invoice>>,
    #[account(
        mut,
        constraint = project_pda.owner == owner.key()
    )]
    pub project_pda: Box<Account<'info, Project>>,
    
    #[account(mut)]
    pub owner: Signer<'info>,   //Should be the project_pda authority
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(time: u8)]
pub struct ClaimInvoice<'info> {
    #[account(
        mut,
        constraint = invoice_pda.project == project_pda.key(),
        constraint = invoice_pda.employee_wallet == employee.key(),
        constraint = time < invoice_pda.from,
    )]
    pub invoice_pda: Box<Account<'info, Invoice>>,
    #[account(mut)]
    pub project_pda: Box<Account<'info, Project>>,

    #[account(mut)]
    pub employee: Signer<'info>,
    pub system_program: Program<'info, System>,
}

*/