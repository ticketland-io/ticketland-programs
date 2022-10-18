// Create a test USDC mint account to test multiple currencies
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

const createMintAccount = async () => {
  // Create the Wrapped SOL Mint account
  const mintAccount = await spl.createMint(
    provider.connection,
    provider.wallet.payer,
    provider.wallet.publicKey,
    null,
    9
  )

  return mintAccount.toBase58()
}

const main = async () => {
  const usdc = await createMintAccount()
  console.log('usdc ', usdc)
  
  const config = JSON.parse(await readFile('./scripts/.config.json'))
  config.supportedMintAccounts.push({
    mintAccount: usdc,
    depositAmount: 1000000000,
    serviceFee: 1000
  })

  await initMain(config)
}

main()
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
