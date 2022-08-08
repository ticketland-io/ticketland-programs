import anchor from '@project-serum/anchor'
import {getAssociatedTokenAddress} from '@solana/spl-token'
import {writeFile} from 'fs/promises'
import * as pda from './helpers/pda.js'
import deploymentConfig from './.config.json' assert { type: 'json' }

const {SystemProgram, SYSVAR_RENT_PUBKEY, Keypair, PublicKey} = anchor.web3

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

const initializeEventRegistry = async (deploymentConfig) => {
  const state = Keypair.generate()
  const treasury = new PublicKey(deploymentConfig.depositTreasury)
  const program = anchor.workspace.EventRegistry
  const deployer = provider.wallet.publicKey

  const supportedCurrencies = await deploymentConfig.supportedMintAccounts.reduce(async (prom, curr) => {
    const acc = await prom
    const mintAccount = new PublicKey(curr.mintAccount)

    return [...acc, {
      mintAccount,
      treasuryAta: await getAssociatedTokenAddress(mintAccount, treasury, true),
      depositAmount: new anchor.BN(curr.depositAmount),
      serviceFee: new anchor.BN(curr.serviceFee),
    }]
  }, [])

  await program.rpc.initialize(
    supportedCurrencies,
    new anchor.BN(deploymentConfig.sellerFeeBasisPoint),
    {
      accounts: {
        state: state.publicKey,
        eventNftAuthority: (await pda.eventNftAuthority(state.publicKey, program.programId))[0],
        cpiAuthority: (await pda.cpiAuthority(state.publicKey, program.programId))[0],
        deployer,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [state, provider.wallet.payer]
    }
  )

  return state.publicKey
}

const initializeTicketSale = async (deploymentConfig, eventRegistryState) => {
  const state = Keypair.generate()
  const treasury = new PublicKey(deploymentConfig.serviceFeeTreasury)
  const program = anchor.workspace.TicketSale
  const deployer = provider.wallet.publicKey

  await program.rpc.initialize(
    treasury,
    {
      accounts: {
        state: state.publicKey,
        eventRegistryState,
        eventRegistryProgram: anchor.workspace.EventRegistry.programId,
        cpiAuthority: (await pda.ticketSaleCpiAuthority(state.publicKey, program.programId))[0],
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
  const program = anchor.workspace.TicketNft
  const deployer = provider.wallet.publicKey

  await program.rpc.initialize(
    {
      accounts: {
        state: state.publicKey,
        nftAuthority: (await pda.nftAuthority(state.publicKey, program.programId))[0],
        ticketSaleState,
        ticketSaleProgram: anchor.workspace.TicketSale.programId,
        deployer,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [state, provider.wallet.payer]
    }
  )

  return state.publicKey
}

export const main = async (deploymentConfig) => {
  const eventRegistryState = await initializeEventRegistry(deploymentConfig)
  const ticketSaleState = await initializeTicketSale(deploymentConfig, eventRegistryState)
  const ticketNftState = await initializeTicketNft(ticketSaleState)

  await writeFile(
    `./deployments/event-registry-${process.env.ENV}.json`,
    JSON.stringify({
      eventRegistryState: eventRegistryState.toBase58(),
      ticketSaleState: ticketSaleState.toBase58(),
      ticketNftState: ticketNftState.toBase58(),
    }, null, 2)
  )
}

main(deploymentConfig)
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
