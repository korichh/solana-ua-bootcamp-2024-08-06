import {
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    clusterApiUrl
} from '@solana/web3.js'

const connection = new Connection(clusterApiUrl('devnet'))
console.log('⚡ Connected to devnet')

const publicKey = new PublicKey('GCWUHj5LqLvMH4bx3KoSG978UmNX5N78usyXMg43JcMS')
const balanceInLamports = await connection.getBalance(publicKey)

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL

console.log(
    `💰 The balance for the wallet at address ${publicKey} is: ${balanceInSOL}`
)