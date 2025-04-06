use anchor_lang::prelude::*;

declare_id!("DNeSEvYcNVwGYruy756J5C2rDMGuTQ2yFhnTkjRHrwYi");

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}

#[program]
pub mod token_lock {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = ctx.bumps.vault;
        Ok(())
    }
}    

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 1
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
