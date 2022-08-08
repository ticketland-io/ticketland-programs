// We need to create new Mint account that will be the Wrapped SOL mint we'll be using in the test validator for dev purposes
// We can't unfortunately use the So11111111111111111111111111111111111111112 account because it's not available in the test validator
// and we can't create it either since we don't possess the private key.
import anchor from '@project-serum/anchor'
import {readFile} from 'fs/promises'
import * as spl from '@solana/spl-token'
import {main as initMain} from './initialize.js'

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

const createWrappedSol = async () => {
  // Create the Wrapped SOL Mint account
  const wrappedSol = await spl.createMint(
    provider.connection,
    provider.wallet.payer,
    provider.wallet.publicKey,
    null,
    9
  )

  return wrappedSol.toBase58()
}

const main = async () => {
  const wrappedSol = await createWrappedSol()
  console.log('wrappedSol ', wrappedSol)
  
  const config = JSON.parse(await readFile('./scripts/.config.json'))
  config.supportedMintAccounts.push({
    mintAccount: wrappedSol,
    depositAmount: 1000000000,
    serviceFee: 1000
  })

  await initMain(config)
}

main()
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
