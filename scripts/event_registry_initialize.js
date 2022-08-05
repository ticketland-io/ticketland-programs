import {getAssociatedTokenAddress} from '@solana/spl-token'
import {writeFile} from 'fs/promises'
import * as pda from './helpers/pda.js'
import deploymentConfig from '../config.json'

const {SystemProgram, SYSVAR_RENT_PUBKEY, Keypair, PublicKey, BN} = anchor.web3
const utf8 = anchor.utils.bytes.utf8

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case 'dev':
      return 'http://localhost:8899'
    case 'testnet':
      return 'https://api.testnet.solana.com'
    case 'mainnet':
      return 'https://api.mainnet-beta.solana.com'
  }
}

const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: 'confirmed'}
)

anchor.setProvider(provider)

const initializeEventRegistry = async () => {
  const state = Keypair.generate()
  const treasury = new PublicKey('')
  const program = anchor.workspace.EventRegistry
  const deployer = provider.wallet.publicKey

  const supportedCurrencies = deploymentConfig.supportedMintAccounts.reduce((acc, curr) => {
    [...acc, {
      mintAccount: new PublicKey(curr.mintAccount),
      treasuryAta: await getAssociatedTokenAddress(mintAccount, treasury, true),
      depositAmount: BN(curr.depositAmount),
      serviceFee: BN(curr.serviceFee),
    }]
  }, [])

  await program.rpc.initialize(
    supportedCurrencies,
    BN(deploymentConfig.sellerFeeBasisPoint),
    {
      accounts: {
        state: state.publicKey,
        eventNftAuthority: (await pda.eventNftAuthority(state.publicKey, program.programId))[0],
        cpiAuthority: (await pda.cpiAuthority(state.publicKey, program.programId))[0],
        masterEdition,
        collectionAuthority,
        collectionAuthorityAta,
        deployer,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [state, provider.wallet.payer]
    }
  )

  return state.publicKey
}

const initializeTicketSale = async (eventRegistryState) => {
  const state = Keypair.generate()
  const treasury = new PublicKey('')
  const program = anchor.workspace.TicketSale
  const deployer = provider.wallet.publicKey

  await program.rpc.initialize(
    treasury,
    {
      accounts: {
        state: state.publicKey,
        eventRegistryState,
        event_registry_program: anchor.workspace.EventRegistry.programId,
        cpi_authority: (await pda.ticketSaleCpiAuthority(state, program.programId))[0],
        deployer,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [state, provider.wallet.payer]
    }
  )

  return state.publicKey
}

const initializeTicketNft = async (ticketSaleState) => {
  const state = Keypair.generate()
  const treasury = new PublicKey('')
  const program = anchor.workspace.TicketNft
  const deployer = provider.wallet.publicKey

  await program.rpc.initialize(
    treasury,
    {
      accounts: {
        state: state.publicKey,
        nftAuthority: (await pda.nftAuthority(state, program.programId))[0],
        ticketSaleState,
        ticket_sale_program: anchor.workspace.TicketSale.programId,
        deployer,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [state, provider.wallet.payer]
    }
  )

  return state.publicKey
}

const main = async () => {
  const eventRegistryState = await initializeEventRegistry()
  const ticketSaleState = await initializeTicketSale(eventRegistryState)
  const ticketNftState = await initializeTicketNft(ticketSaleState)

  await writeFile(
    `./deployments/event-registry-${process.env.ENV}.json`,
    JSON.stringify({
      eventRegistryState: eventRegistryState.publicKey.toBase58(),
      ticketSaleState: ticketSaleState.publicKey.toBase58(),
      ticketNftState: ticketNftState.publicKey.toBase58(),
    }, null, 2)
  )
}

main()
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
