import * as anchor from '@project-serum/anchor'

const {PublicKey} = anchor.web3
const utf8 = anchor.utils.bytes.utf8

export const eventNftAuthority = async (eventRegistryState, eventRegistryProgramId) => await PublicKey.findProgramAddress(
  [utf8.encode('event_nft_authority'), eventRegistryState.toBuffer()],
  eventRegistryProgramId
)

export const cpiAuthority = async (eventRegistryState, eventRegistryProgramId) => await PublicKey.findProgramAddress(
  [utf8.encode('cpi_authority'), eventRegistryState.toBuffer()],
  eventRegistryProgramId
)

export const ticketSaleCpiAuthority = async (ticketSaleSate, ticketSaleProgramId) => await PublicKey.findProgramAddress(
  [utf8.encode('ticket_sale:cpi_authority'), ticketSaleSate.toBuffer()],
  ticketSaleProgramId
)

export const nftAuthority = async (ticketNftState, ticketNftProgramId) => await PublicKey.findProgramAddress(
  [utf8.encode('nft_authority'), ticketNftState.toBuffer()],
  ticketNftProgramId
)
