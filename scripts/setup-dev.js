// Create a test USDC mint account to test multiple currencies
import anchor from '@project-serum/anchor'
import {readFile} from 'fs/promises'
import * as spl from '@solana/spl-token'
import {main as initMain} from './initialize.js'
import deploymentConfig from './.config.json' assert { type: 'json' }
import usdc from '../wallets-dev/usdc.json' assert { type: 'json' }

const {PublicKey, Keypair} = anchor.web3

const USDC = new PublicKey('AxmXHEFuCBmLfeVvEnD2mQxXx8bvmiSGZSMDGByK21k2')

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
  const feePayer = provider.wallet.payer
  // Create the USDC Mint account
  try {
    const mintAccount = await spl.createMint(
      provider.connection,
      feePayer,
      provider.wallet.publicKey,
      null,
      6,
      Keypair.fromSecretKey(new Uint8Array(usdc)) // matches the constant USDC account above
    )
  } catch {}

  for (let i = 0; i < deploymentConfig.testAccounts.length; i++) {
    const testAccount = new PublicKey(deploymentConfig.testAccounts[i])
    const usdcAta = await spl.getOrCreateAssociatedTokenAccount(
      provider.connection,
      feePayer,
      USDC,
      testAccount,
      false
    )
    
    await spl.mintTo(
      provider.connection,
      feePayer,
      USDC,
      usdcAta.address,
      provider.wallet.publicKey,
      100_000_000000 
    ) 
  }
}

const main = async () => {
   await createMintAccount()
  
  const config = JSON.parse(await readFile('./scripts/.config.json'))
  await initMain(config)
}

main()
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
