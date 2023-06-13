use anchor_lang::prelude::*;

use crate::state::*;

//PROBLEM:
//WHEN I CHANGE THE NAME OF THE PROJECT, CAN I PAY TO INTEGRATE THE STATE?

//ToDo Global: See if in ogni different function i changed all the state like project_state in employee and invoice stuff
//ToDo Global: Cancel all the space that isn't needed in the different state.
//ToDo Global: Set Every state and vualt as mut

///////////////////////////////////////////////////////////////////////////////////////////

//PROJECT

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct  CreateProject<'info> {
    #[account(
        init,
        payer = owner,
        space = Project::space() + project_name.len() + 100, //WHEN I'M SURE I NEED TO TAKE OFF THE 100
    )]
    pub project: Account<'info, Project>,
    #[account(seeds = [b"project", project.key().as_ref()], bump)]
    pub project_vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(auth: Pubkey)]
pub struct  ProjectChangeAuth<'info> {
    #[account(
        mut,
        constraint = *owner.key != auth,
        constraint = *owner.key == project.owner,
    )]
    pub project: Account<'info, Project>,
    #[account(seeds = [b"project", project.key().as_ref()], bump)]
    pub project_vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct  ProjectChangeName<'info> {
    #[account(
        mut,
        constraint = project.project_name != project_name,
        constraint = *owner.key == project.owner,
    )]
    pub project: Account<'info, Project>,
    #[account(seeds = [b"project", project.key().as_ref()], bump)]
    pub project_vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct  ProjectDeposit<'info> {
    #[account(
        mut,
        constraint = amount > 0,
        constraint = *owner.key == project.owner,
    )]
    pub project: Account<'info, Project>,
    #[account(mut, seeds = [b"project", project.key().as_ref()], bump = project.project_bump)]
    pub project_vault: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct  ProjectWithdraw<'info> {
    #[account(
        mut,
        constraint = amount < project.balance,
        constraint = *owner.key == project.owner,
    )]
    pub project: Account<'info, Project>,
    #[account(mut, seeds = [b"project", project.key().as_ref()], bump = project.project_bump)]
    pub project_vault: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

///////////////////////////////////////////////////////////////////////////////////////////

//EMPLOYEE

//ToDo: CALCULATE MINIMUM RENT for create_employee & employee_activate function

#[derive(Accounts)]
#[instruction(employee_title: String)]
pub struct  CreateEmployee<'info> {
    #[account(
        init,
        payer = project_vault,
        space = Employee::space() + employee_title.len() + 100,
    )]
    pub employee: Account<'info, Employee>,
    #[account(mut, seeds = [b"employee", project.key().as_ref(), employee.key().as_ref()], bump = employee.employee_bump)]
    pub employee_vault: SystemAccount<'info>,

    #[account(
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,
    #[account(mut, seeds = [b"project", project.key().as_ref()], bump = project.project_bump)]
    pub project_vault: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(employee_wallet: Pubkey)]
pub struct  EmployeeChangeWallet<'info> {
    #[account(
        mut,
        constraint = employee_wallet != employee.employee,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,

    #[account(
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(employee_title: String)]
pub struct  EmployeeChangeTitle<'info> {
    #[account(
        mut,
        constraint = employee_title != employee.employee_title,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,

    #[account(
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(monthly_pay: u64)]
pub struct  EmployeeChangePay<'info> {
    #[account(
        mut,
        constraint = monthly_pay != employee.monthly_pay,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,

    #[account(
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  EmployeeActivate<'info> {
    #[account(
        init,
        payer = employee_vault,
        space = Invoice::space() + 100,
    )]
    pub invoice: Account <'info, Invoice>,
    #[account(mut, seeds = [b"invoice", employee.key().as_ref(), invoice.key().as_ref()], bump)]
    pub invoice_vault: SystemAccount<'info>,
    
    #[account(
        mut,
        constraint = project.balance > employee.monthly_pay,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,
    #[account(mut, seeds = [b"employee", project.key().as_ref(), employee.key().as_ref()], bump = employee.employee_bump)]
    pub employee_vault: SystemAccount<'info>,

    #[account(
        mut,
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,
    #[account(mut, seeds = [b"project", project.key().as_ref()], bump = project.project_bump)]
    pub project_vault: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct  EmployeeDeactivate<'info> {
    #[account(
        mut,
        constraint = employee.is_active == true,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,

    #[account(
        constraint = project.owner == owner.key(),
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

///////////////////////////////////////////////////////////////////////////////////////////

//INVOICE - Employee as AUTH

//ToDo: Find a way to create an Invoice for the 1st of the month later on the TS side everytime i claim my invoice

#[derive(Accounts)]
pub struct ClaimInvoice<'info> {
    #[account(
        mut,
        constraint = invoice.employee == employee.employee,
        constraint = invoice.employee == owner.key(),
        constraint = invoice.project == project.key(),

    )]
    pub invoice: Account <'info, Invoice>,
    #[account(mut, seeds = [b"invoice", employee.key().as_ref(), invoice.key().as_ref()], bump = invoice.invoice_bump)]
    pub invoice_vault: SystemAccount<'info>,

    #[account(
        init,
        payer = employee_vault,
        space = Invoice::space() + 100,
    )]
    pub new_invoice: Account <'info, Invoice>,
    #[account(mut, seeds = [b"invoice", employee.key().as_ref(), invoice.key().as_ref()], bump)]
    pub new_invoice_vault: SystemAccount<'info>,
    
    #[account(
        mut,
        constraint = project.key() == employee.project,
    )]
    pub employee: Account<'info, Employee>,
    #[account(mut, seeds = [b"employee", project.key().as_ref(), employee.key().as_ref()], bump = employee.employee_bump)]
    pub employee_vault: SystemAccount<'info>,

    #[account(
        mut,
    )]
    pub project: Account<'info, Project>,
    #[account(mut, seeds = [b"project", project.key().as_ref()], bump = project.project_bump)]
    pub project_vault: SystemAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}