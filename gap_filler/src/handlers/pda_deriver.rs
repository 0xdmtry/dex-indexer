use crate::models::consts::{PUMPFUN_PROGRAM_ID, PUMPSWAP_PROGRAM_ID, SOL_MINT};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn derive_bonding_curve_pda(mint: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let mint_pubkey = Pubkey::from_str(mint)?;
    let pumpfun_program = Pubkey::from_str(PUMPFUN_PROGRAM_ID)?;

    let (pda, _bump) =
        Pubkey::find_program_address(&[b"bonding-curve", mint_pubkey.as_ref()], &pumpfun_program);

    Ok(pda)
}

pub fn derive_pool_pda(mint: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let base_mint = Pubkey::from_str(mint)?;
    let quote_mint = Pubkey::from_str(SOL_MINT)?;
    let pump_program = Pubkey::from_str(PUMPFUN_PROGRAM_ID)?;
    let pumpswap_program = Pubkey::from_str(PUMPSWAP_PROGRAM_ID)?;

    // Derive creator PDA from Pump program
    let (creator, _) =
        Pubkey::find_program_address(&[b"pool-authority", base_mint.as_ref()], &pump_program);

    let index = 0u16;

    let (pda, _bump) = Pubkey::find_program_address(
        &[
            b"pool",
            &index.to_le_bytes(),
            creator.as_ref(),
            base_mint.as_ref(),
            quote_mint.as_ref(),
        ],
        &pumpswap_program,
    );

    Ok(pda)
}
