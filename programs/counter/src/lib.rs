use anchor_lang::prelude::*;

declare_id!("AtJHqZUwBkp7SMzi6HtJ3zsTcdT4MrFwvgnmQzPqeVk");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; //store bump seed in  `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // increment the value stored in the counter account
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // the account payung to create the counter account
    #[account(mut)]
    pub user: Signer<'info>, //signer on the transaction

    // the counter account created and initialized in the instruction
    #[account(init, seeds=[b"counter"], bump, payer = user, space = 8 + Counter::INIT_SPACE)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

// Account required by the increment
#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    #[account(mut, seeds=[b"counter"], bump=counter.bump)] //specify that the account is mutable
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}
