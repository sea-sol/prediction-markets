use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
use instructions::init::*;

declare_id!("FW9KvGkRcnibqm5LSE4J8sq3homgVizKGBoNA511gR2s");

#[program]
pub mod prediction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        init(ctx)
    }
}
