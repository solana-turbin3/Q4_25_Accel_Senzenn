use anchor_lang::prelude::*;

use crate::state::UserWhitelist;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The user whose whitelist status we're managing
    /// CHECK: This is the user we want to add/remove from whitelist
    pub user: UncheckedAccount<'info>,

    /// CHECK: User's whitelist PDA - derived as [b"whitelist", user.key()]
    #[account(
        mut,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub user_whitelist: Account<'info, UserWhitelist>,

    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self) -> Result<()> {
        // Check if user is already whitelisted (PDA already exists)
        let account_info = self.user_whitelist.to_account_info();
        if !account_info.data_is_empty() {
            return Ok(()); // Already whitelisted
        }

        // Initialize the user's whitelist PDA
        let bump = self.user_whitelist.bump;
        let user_whitelist = &mut self.user_whitelist;
        user_whitelist.user = self.user.key();
        user_whitelist.bump = bump;

        Ok(())
    }

    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        // Close the user's whitelist PDA and refund rent to admin
        let user_whitelist_account = self.user_whitelist.to_account_info();
        let admin_account = self.admin.to_account_info();

        // Calculate refund amount (rent for the account)
        let rent_balance = user_whitelist_account.lamports();
        **admin_account.try_borrow_mut_lamports()? += rent_balance;
        **user_whitelist_account.try_borrow_mut_lamports()? -= rent_balance;

        Ok(())
    }

}