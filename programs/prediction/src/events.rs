use anchor_lang::prelude::*;

#[event]
pub struct GlobalInitialized {
    pub global_id: Pubkey,
    pub fee_recipient: Pubkey,
    pub fee_percentage: u16,
}
