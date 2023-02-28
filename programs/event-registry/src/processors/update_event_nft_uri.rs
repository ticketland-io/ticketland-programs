use anchor_lang::prelude::*;
use anchor_metaplex::{
  UpdateMetadata,
  update_metadata,
  mpl_token_metadata::state::{DataV2, Metadata, TokenMetadataAccount},
};
use crate::{
  context::update_event_nft_uri::UpdateEventNftUri,
};

fn update_uri(
  ctx: &Context<UpdateEventNftUri>,
  new_uri: String
) -> Result<()> {
  let state = &ctx.accounts.state;
  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"event_nft_authority", state_key.as_ref(),
    &[state.bumps.event_nft_authority,]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  let metadata = Metadata::from_account_info(&ctx.accounts.event_nft_metadata)?;

  // TODO: We can check if there is a URI already set; if so we can revert
  let cpi_accounts = UpdateMetadata {
    metadata_account: ctx.accounts.event_nft_metadata.clone(),
    update_authority: ctx.accounts.event_nft_authority.clone(),
  };

  let data = Some(DataV2 {
		name: metadata.data.name,
		symbol: metadata.data.symbol,
		uri: new_uri,
		seller_fee_basis_points: metadata.data.seller_fee_basis_points,
		creators: None,
		collection: None,
		uses: None,
  });

  update_metadata(
    cpi_accounts,
    signer_seeds,
    None,
    data,
    Some(true),
    None,
  )
}

pub fn exec(
  ctx: Context<UpdateEventNftUri>,
  new_uri: String,
) -> Result<()> {
  update_uri(&ctx, new_uri)?;

  Ok(())
}
