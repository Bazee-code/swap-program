pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Gw2WCB4w76D7J7ukTZLWy5RfNtkp9BYQRmPvDU7JpCNJ");

#[program]
pub mod swap_program {
    use crate::instruction::MakeOffer;

    use super::*;

    pub fn make_offer(context: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault();
        instructions::make_offer::save_offer();
    }
}

