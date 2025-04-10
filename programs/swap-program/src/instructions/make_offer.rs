use anchor_lang::prelude::*;
use anchor_spl::{token::TokenAccount, token_interface::Mint};

#[derive(Accounts)]
pub struct MakeOffer<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub toke_mint_a : InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub toke_mint_b : InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = token_mint_a, associated_token::authority = maker, associated_token::token_program = token_program)]
    pub maker_token_account_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id_to_le_bytes()],
        bump
    )]
    pub offer: Account<'info, System>,

    #[account(
        init, 
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer, 
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>
}

pub fn send_offered_tokens_to_vault(ctx: Context<MakeOffer>) -> Result<()> {
    msg!("Greetings from: {{:?}}", ctx.program_id);
    Ok(())
}
