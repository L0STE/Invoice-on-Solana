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
        project_name: Vec<u8>, 
    ) -> Result<()> {
        ctx.accounts.project_pda.owner = *ctx.accounts.owner.key;
        ctx.accounts.project_pda.project_name = project_name;
        ctx.accounts.project_pda.balance = 0;

        Ok(())
    }

    pub fn project_deposit (
        ctx: Context<ProjectDeposit>,
        amount: u8, 
    ) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: ctx.accounts.owner.to_account_info(), 
            to: ctx.accounts.project_pda.to_account_info()
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        system_program::transfer(cpi_context, amount as u64)?;

        ctx.accounts.project_pda.balance = amount;
        Ok(())
    }

    pub fn project_withdraw (
        ctx: Context<ProjectWithdraw>,
        amount: u8, 
    ) -> Result<()> {
        let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.owner.key().clone().to_bytes(),
            &[*ctx.bumps.get("project").unwrap()]
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the variables for the CPI : 
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_pda.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        //To use a PDA to sign the CPI instead of using CpiContext::new you need CpiContext::new_with_signer
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, amount as u64)?;   
        
        ctx.accounts.project_pda.balance = ctx.accounts.project_pda.balance - amount;
        Ok(())
    }

/* 
    //ToDo
    pub fn change_project_manager ( //HOW THE FUCK DO YOU DO THAT
        ctx: Context<ChangeProjectManager>,
    ) -> Result<()> {
        Ok(())
    }
*/

    pub fn project_change_name (
        ctx: Context<ProjectChangeName>,
        project_name: Vec<u8>, 
    ) -> Result<()> {
        ctx.accounts.project_pda.project_name = project_name;
        Ok(())
    }

///////////////////////////////////////////////////////////////////////////////////////////

    //EMPLOYEE  

    pub fn create_employee (
        ctx: Context<CreateEmployee>,
        employee_wallet: Pubkey,
        title: Vec<u8>, 
    ) -> Result<()> {
        ctx.accounts.employee_pda.project = ctx.accounts.project_pda.key();
        ctx.accounts.employee_pda.employee = employee_wallet;
        ctx.accounts.employee_pda.title = title;
        ctx.accounts.employee_pda.day_worked = 0;
        ctx.accounts.employee_pda.money_earned = 0;
        ctx.accounts.employee_pda.has_accepted = false;

        Ok(())
    }

    pub fn employee_change_wallet (
        ctx: Context<EmployeeChangeWallet>,
        employee_wallet: Pubkey,
    ) -> Result<()> {
        ctx.accounts.employee_pda.employee = employee_wallet;

        Ok(())
    }

    pub fn employee_change_title (
        ctx: Context<EmployeeChangeTitle>,
        title: Vec<u8>, 
    ) -> Result<()> {
        ctx.accounts.employee_pda.title = title;

        Ok(())
    }

    //ToDo EMPLOYEE ACCEPT

    ///////////////////////////////////////////////////////////////////////////////////////////
    
    //INVOICE

    pub fn create_invoice(
        ctx: Context<CreateInvoice>,
        amount: u8,
        employee: Pubkey, 
        start_date: u8, 
        end_date: u8,
    ) -> Result<()> {

        //Populate the Invoice Account
        ctx.accounts.invoice_pda.project = ctx.accounts.project_pda.key();
        ctx.accounts.invoice_pda.employee_wallet = employee;
        ctx.accounts.invoice_pda.from = start_date;
        ctx.accounts.invoice_pda.to = end_date;
        ctx.accounts.invoice_pda.amount = amount;
        ctx.accounts.invoice_pda.has_claimed = false;

        //Send the amount to the escrow wallet
        let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.owner.key().clone().to_bytes(),
            &[*ctx.bumps.get("project").unwrap()]
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the variables for the CPI : 
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_pda.to_account_info(),
            to: ctx.accounts.invoice_pda.to_account_info(),
        };

        //To use a PDA to sign the CPI instead of using CpiContext::new you need CpiContext::new_with_signer
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, amount as u64)?;   

        Ok(())
    }

    pub fn invoice_change_amount(
        ctx: Context<InvoiceChangeAmount>,
        _time: u8, 
        amount: u8 
    ) -> Result<()> {

        if ctx.accounts.invoice_pda.amount > amount {
            let seeds = &[
                "invoice".as_bytes(),
                &ctx.accounts.invoice_pda.key().clone().to_bytes(),
                &[*ctx.bumps.get("invoice").unwrap()]
            ];

            let signer_seeds = &[&seeds[..]];

            //Create the variables for the CPI : 
            let cpi_program = ctx.accounts.system_program.to_account_info();
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.invoice_pda.to_account_info(),
                to: ctx.accounts.project_pda.to_account_info(),
            };

            //To use a PDA to sign the CPI instead of using CpiContext::new you need CpiContext::new_with_signer
            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            system_program::transfer(cpi_context, (ctx.accounts.invoice_pda.amount-amount) as u64)?;   
        }

        if ctx.accounts.invoice_pda.amount < amount {
            let seeds = &[
            "project".as_bytes(),
            &ctx.accounts.owner.key().clone().to_bytes(),
            &[*ctx.bumps.get("project").unwrap()]
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the variables for the CPI : 
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.project_pda.to_account_info(),
            to: ctx.accounts.invoice_pda.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);            
            system_program::transfer(cpi_context, (amount-ctx.accounts.invoice_pda.amount) as u64)?;   
        }

        ctx.accounts.invoice_pda.amount = amount;

        Ok(())
    } 

    pub fn invoice_change_date(
        ctx: Context<InvoiceChangeDate>, 
        _time: u8,
        from: u8, 
        to: u8, 
    ) -> Result<()> {
        ctx.accounts.invoice_pda.from = from;
        ctx.accounts.invoice_pda.to = to;

        Ok(())
    } 

    //EMPLOYEE SIDE
    pub fn claim_invoice(
        ctx: Context<ClaimInvoice>, 
        _time: u8
    ) -> Result<()> {

        let seeds = &[
            "invoice".as_bytes(),
            &ctx.accounts.invoice_pda.key().clone().to_bytes(),
            &[*ctx.bumps.get("invoice").unwrap()]
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the variables for the CPI : 
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.invoice_pda.to_account_info(),
            to: ctx.accounts.employee.to_account_info(),
        };

        //To use a PDA to sign the CPI instead of using CpiContext::new you need CpiContext::new_with_signer
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_context, ctx.accounts.invoice_pda.amount as u64)?;   

        Ok(())
    }

}


