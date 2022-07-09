use anchor_lang::prelude::*;

pub fn is_wrapped_sol(mint: Pubkey) -> bool {
  mint == "So11111111111111111111111111111111111111112".try_into().unwrap()
}
