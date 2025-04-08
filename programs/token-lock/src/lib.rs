use anchor_lang::prelude::*;

declare_id!("DNeSEvYcNVwGYruy756J5C2rDMGuTQ2yFhnTkjRHrwYi");

#[program]
pub mod token_lock {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        recipient: Pubkey,
        cancel_permission: u8,
        change_recipient_permission: u8,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.balance = 0;
        vault.recipient = recipient;
        vault.cancel_permission = cancel_permission;
        vault.change_recipient_permission = change_recipient_permission;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key,
            &vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                vault.to_account_info(),
            ],
        )?;

        vault.balance += amount;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(recipient: Pubkey, cancel_permission: u8, change_recipient_permission: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"vault", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Vault::LEN
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
    pub recipient: Pubkey,
    pub cancel_permission: u8,
    pub change_recipient_permission: u8,
}

impl Vault {
    pub const LEN: usize = 32 + 8 + 32 + 1 + 1;
}
