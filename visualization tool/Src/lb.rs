use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

declare_id!("YourProgramIDHere");

#[program]
pub mod collaboration_tool {
    use super::*;
    pub fn initialize_document(ctx: Context<InitializeDocument>, title: String) -> ProgramResult {
        let document = &mut ctx.accounts.document;
        document.title = title;
        document.content = String::new();
        Ok(())
    }

    pub fn update_content(ctx: Context<UpdateContent>, new_content: String) -> ProgramResult {
        let document = &mut ctx.accounts.document;
        document.content = new_content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDocument<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Document {
    pub title: String,
    pub content: String,
}
