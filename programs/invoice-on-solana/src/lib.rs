mod account;
mod state;
mod error;

use crate::account::*;
use crate::error::*;
pub use crate::state::*;

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod invoice_on_solana {
    use super::*;

///////////////////////////////////////////////////////////////////////////////////////////

    //PROJECT

    pub fn create_project (
        ctx: Context<CreateProject>,
        project_name: String, 
        project_bump: u8,
    ) -> Result<()> {
        ctx.accounts.project.owner = *ctx.accounts.owner.key;
        ctx.accounts.project.project_name = project_name;
        ctx.accounts.project.balance = 0;
        ctx.accounts.project.monthly_spending = 0;
        ctx.accounts.project.project_bump = project_bump;

        Ok(())
    }
    
    pub fn project_change_auth (
        ctx: Context<ProjectChangeAuth>,
        auth: Pubkey, 
    ) -> Result<()> {
        ctx.accounts.project.owner = auth;

        Ok(())
    }

    pub fn project_change_name (
        ctx: Context<ProjectChangeName>,
        project_name: String, 
    ) -> Result<()> {
        ctx.accounts.project.project_name = project_name;
        
        Ok(())
    }

    pub fn project_deposit (
        ctx: Context<ProjectDeposit>,
        amount: u64, 
    ) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: ctx.accounts.owner.to_account_info(), 
            to: ctx.accounts.project_vault.to_account_info()
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        system_program::transfer(cpi_context, amount)?;

        ctx.accounts.project.balance += amount;
        Ok(())
    }

    pub fn project_withdraw (
        ctx: Context<ProjectWithdraw>,
        amount: u64, 
    ) -> Result<()> {
        let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.project.key().clone().to_bytes(),
            &[ctx.accounts.project.project_bump] 
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the variables for the CPI : 
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        //To use a PDA to sign the CPI instead of using CpiContext::new you need CpiContext::new_with_signer
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, amount)?;   
        
        ctx.accounts.project.balance -= amount;
        Ok(())
    }

///////////////////////////////////////////////////////////////////////////////////////////

    //EMPLOYEE  

    //ToDo HERE: Minimum Rent for creating an Invoice_State
    pub fn create_employee (
        ctx: Context<CreateEmployee>,
        title: String, 
        employee_wallet: Pubkey,
        monthly_pay: u64,
        employee_bump: u8,
    ) -> Result<()> {
        ctx.accounts.employee.project = ctx.accounts.project.key();
        ctx.accounts.employee.employee = employee_wallet;
        ctx.accounts.employee.employee_title = title;
        ctx.accounts.employee.monthly_pay = monthly_pay;
        ctx.accounts.employee.is_active = false;
        ctx.accounts.employee.employee_bump = employee_bump;

    let minimum_rent = 1 * 1_000_000; //MADE UP 

    //Project_Vault pay Employee_Vault the minimum for creating an Invoice_Vault
    let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.project.key().clone().to_bytes(),
            &[ctx.accounts.project.project_bump] 
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_vault.to_account_info(),
            to: ctx.accounts.employee_vault.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, minimum_rent)?;   
    
        Ok(())
    }

    pub fn employee_change_wallet (
        ctx: Context<EmployeeChangeWallet>,
        employee_wallet: Pubkey,
    ) -> Result<()> {
        ctx.accounts.employee.employee = employee_wallet;

        Ok(())
    }

    pub fn employee_change_title (
        ctx: Context<EmployeeChangeTitle>,
        title: String, 
    ) -> Result<()> {
        ctx.accounts.employee.employee_title = title;

        Ok(())
    }

    pub fn employee_change_pay (
        ctx: Context<EmployeeChangePay>,
        monthly_pay: u64,
    ) -> Result<()> {
        //Project_state
        if ctx.accounts.employee.monthly_pay > monthly_pay {
            ctx.accounts.project.monthly_spending -= ctx.accounts.employee.monthly_pay - monthly_pay;
        }

        if ctx.accounts.employee.monthly_pay < monthly_pay {
            ctx.accounts.project.monthly_spending += monthly_pay - ctx.accounts.employee.monthly_pay;
        }

        //Employee_state
        ctx.accounts.employee.monthly_pay = monthly_pay;

        Ok(())
    }

    //ToDo HERE
    pub fn employee_activate (
        ctx: Context<EmployeeActivate>,
        from: i64,
        to: i64,
        invoice_bump: u8,
    ) -> Result<()> {
        //Project_state
        ctx.accounts.project.monthly_spending += ctx.accounts.employee.monthly_pay;
        ctx.accounts.project.balance -= ctx.accounts.employee.monthly_pay;

        //Employee_state
        ctx.accounts.employee.is_active = true;

        //Invoice_state
        ctx.accounts.invoice.project = ctx.accounts.project.key();
        ctx.accounts.invoice.employee = ctx.accounts.employee.key();
        ctx.accounts.invoice.from = from;
        ctx.accounts.invoice.to = to;
        ctx.accounts.invoice.balance = ctx.accounts.employee.monthly_pay;
        ctx.accounts.invoice.has_claimed = false;
        ctx.accounts.invoice.invoice_bump = invoice_bump;

        //Recharge the employee_vault
        let minimum_rent = 1 * 1_000_000; //MADE UP 

        let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.project.key().clone().to_bytes(),
            &[ctx.accounts.project.project_bump] 
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_vault.to_account_info(),
            to: ctx.accounts.employee_vault.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, minimum_rent)?;   

        ctx.accounts.project.balance -= minimum_rent;

        //Send to Escrow the monthly_pay;
        let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.project.key().clone().to_bytes(),
            &[ctx.accounts.project.project_bump] 
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_vault.to_account_info(),
            to: ctx.accounts.invoice_vault.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, ctx.accounts.employee.monthly_pay)?;

        Ok(())
    }

    pub fn employee_deactivate (
        ctx: Context<EmployeeDeactivate>,
    ) -> Result<()> {
        //Project_state
        ctx.accounts.project.monthly_spending -= ctx.accounts.employee.monthly_pay;

        //Employee_state
        ctx.accounts.employee.is_active = false;

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    
    //INVOICE - Employee as AUTH

    pub fn claim_invoice(
        ctx: Context<ClaimInvoice>,
        from: i64,
        to: i64,
        invoice_bump: u8,
    ) -> Result<()> {

        if Clock::get()?.unix_timestamp < to {
            return Err(InvoiceError::ExpirationError.into());
        }
        
        //Claim the Invoice
        let seeds = &[
            "invoice".as_bytes(),
            &ctx.accounts.employee.key().clone().to_bytes(),
            &ctx.accounts.invoice.key().clone().to_bytes(),
            &[ctx.accounts.invoice.invoice_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.invoice_vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);            
        system_program::transfer(cpi_context, (ctx.accounts.invoice.balance))?;
        
        ctx.accounts.invoice.has_claimed = true;

        //Set a new Invoice for the next month 
        if ctx.accounts.employee.is_active == true {
            //Project_state
            ctx.accounts.project.balance -= ctx.accounts.employee.monthly_pay;

            //Invoice_state
            ctx.accounts.new_invoice.project = ctx.accounts.project.key();
            ctx.accounts.new_invoice.employee = ctx.accounts.employee.key();
            ctx.accounts.new_invoice.from = from;
            ctx.accounts.new_invoice.to = to;
            ctx.accounts.new_invoice.balance = ctx.accounts.employee.monthly_pay;
            ctx.accounts.new_invoice.has_claimed = false;
            ctx.accounts.new_invoice.invoice_bump = invoice_bump;

            //Recharge the employee_vault
            let minimum_rent = 1 * 1_000_000; //MADE UP 

            let seeds = &[
                "project".as_bytes(),
                &ctx.accounts.project.key().clone().to_bytes(),
                &[ctx.accounts.project.project_bump] 
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_program = ctx.accounts.system_program.to_account_info();
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.project_vault.to_account_info(),
                to: ctx.accounts.employee_vault.to_account_info(),
            };

            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            system_program::transfer(cpi_context, minimum_rent)?;   

            ctx.accounts.project.balance -= minimum_rent;

            //Send to Escrow the monthly_pay;
            let seeds = &[
                "project".as_bytes(),
                &ctx.accounts.project.key().clone().to_bytes(),
                &[ctx.accounts.project.project_bump] 
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_program = ctx.accounts.system_program.to_account_info();
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.project_vault.to_account_info(),
                to: ctx.accounts.new_invoice_vault.to_account_info(),
            };

            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            system_program::transfer(cpi_context, ctx.accounts.employee.monthly_pay)?;
        }

        Ok(())
    }



}


