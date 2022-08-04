import * as anchor from '@project-serum/anchor'

const {PublicKey} = anchor.web3
const utf8 = anchor.utils.bytes.utf8

export const eventNftAuthority = async (state, programId) => await PublicKey.findProgramAddress(
  [utf8.encode('event_nft_authority'), state.toBuffer()],
  programId
)

export const cpiAuthority = async (state, programId) => await PublicKey.findProgramAddress(
  [utf8.encode('cpi_authority'), state.toBuffer()],
  programId
)
