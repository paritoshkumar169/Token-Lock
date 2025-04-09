use anchor_lang::prelude::*;

declare_id!("GBpt6Vk31WJfh6DsnTRGy585yF4zMLv8QD2TwX3GJQvM");



#[program]
pub mod token_lock {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        recipient: Pubkey,
        cancel_permission: u8,
        change_recipient_permission: u8,
        lock_duration: i64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        vault.authority = ctx.accounts.authority.key();
        vault.balance = 0;
        vault.recipient = recipient;
        vault.cancel_permission = cancel_permission;
        vault.change_recipient_permission = change_recipient_permission;
        vault.lock_until = clock.unix_timestamp + lock_duration;

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

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= vault.lock_until, ErrorCode::UnlockTimeNotReached);

        **vault.to_account_info().try_borrow_mut_lamports()? -= vault.balance;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += vault.balance;

        vault.balance = 0;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        match vault.cancel_permission {
            0 => return Err(ErrorCode::CancelNotAllowed.into()), // None
            1 => require_keys_eq!(ctx.accounts.requester.key(), vault.recipient, ErrorCode::Unauthorized),
            2 => require_keys_eq!(ctx.accounts.requester.key(), vault.authority, ErrorCode::Unauthorized),
            3 => require!(
                ctx.accounts.requester.key() == vault.authority || ctx.accounts.requester.key() == vault.recipient,
                ErrorCode::Unauthorized
            ),
            _ => return Err(ErrorCode::InvalidCancelMode.into()),
        }

        **vault.to_account_info().try_borrow_mut_lamports()? -= vault.balance;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += vault.balance;

        vault.balance = 0;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(recipient: Pubkey, cancel_permission: u8, change_recipient_permission: u8, lock_duration: i64)]
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

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut, address = vault.recipient)]
    pub recipient: Signer<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub requester: Signer<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
    pub recipient: Pubkey,
    pub cancel_permission: u8,
    pub change_recipient_permission: u8,
    pub lock_until: i64,
}

impl Vault {
    pub const LEN: usize = 32 + 8 + 32 + 1 + 1 + 8;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Current time is less than the lock period.")]
    UnlockTimeNotReached,
    #[msg("Cancel not allowed.")]
    CancelNotAllowed,
    #[msg("Unauthorized cancellation.")]
    Unauthorized,
    #[msg("Invalid cancel permission mode.")]
    InvalidCancelMode,
}
