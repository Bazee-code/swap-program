use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::TokenAccount, token_interface::{Mint, TokenInterface}};

#[derive(Accounts)]
#[instruction(id: u64)]
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
    pub offer: Account<'info, Offer>,

    #[account(
        init, 
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer, 
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

// pub fn send_offered_tokens_to_vault(ctx: Context<MakeOffer>, token_a_offered_amount: u64,) -> Result<()> {
//     transfer_tokens(
//         from: &context.accounts.maker_token_account_a,
//     )
//     Ok(())
// }
