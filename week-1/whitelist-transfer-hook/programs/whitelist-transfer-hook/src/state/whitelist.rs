

use anchor_lang::prelude::*;

#[account]
pub struct UserWhitelist {
    pub user: Pubkey,  // The user this whitelist entry belongs to
    pub bump: u8,      // PDA bump seed
}

// Keep the old struct name for backwards compatibility during migration
pub type Whitelist = UserWhitelist;