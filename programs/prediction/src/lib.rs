use anchor_lang::prelude::*;

declare_id!("FW9KvGkRcnibqm5LSE4J8sq3homgVizKGBoNA511gR2s");

#[program]
pub mod prediction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
