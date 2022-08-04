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

const main = async () => {
  const state = Keypair.generate()
  const program = anchor.workspace.EventRegistry
  const deployer = provider.wallet.publicKey
  const treasury = new PublicKey('')

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

  await writeFile(
    `./deployments/event-registry-${process.env.ENV}.json`,
    JSON.stringify({
      state: state.publicKey.toBase58(),
    }, null, 2)
  )
}

main()
.then(() => console.log('Success'))
.catch(error => console.log('Error: ', error))
