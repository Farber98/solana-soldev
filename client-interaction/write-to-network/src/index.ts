import * as web3 from '@solana/web3.js'
import * as bs58 from 'bs58';
import Dotenv from 'dotenv'
Dotenv.config()


async function main() {
    const payer = initializeKeyPairFromPrivateKey(process.env.PHANTOM_PRIVATE_KEY ?? "")
    const conn = new web3.Connection(web3.clusterApiUrl('devnet'))
    const programId = new web3.PublicKey(process.env.PROGRAM_ADDRESS ?? "")
    const programData = new web3.PublicKey(process.env.PROGRAM_DATA_ADDRESS ?? "")
    const receiver = new web3.PublicKey(process.env.RECEIVER_PUBLIC_KEY ?? "")
    // await pingProgram(conn, payer, programId, programData)
    await transferSOLLamports(conn, payer, receiver, 0.01);
}

async function pingProgram(conn: web3.Connection, payer: web3.Keypair, programId: web3.PublicKey, programDataAccount: web3.PublicKey) {

    //  create a transaction
    //  create an instruction
    //  add the instruction to the transaction
    //  send the transaction.

    const instruction = new web3.TransactionInstruction({
        keys: [
            {
                pubkey: programDataAccount,
                isSigner: false,
                isWritable: true
            },
        ],
        programId
    })

    const signature = await web3.sendAndConfirmTransaction(
        conn,
        new web3.Transaction().add(instruction),
        [payer]
    )

    console.log(`You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
}

async function transferSOLLamports(conn: web3.Connection, payer: web3.Keypair, receiver: web3.PublicKey, amount: number) {
    const instruction = web3.SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: receiver,
        lamports: web3.LAMPORTS_PER_SOL * amount
    })

    const signature = await web3.sendAndConfirmTransaction(
        conn,
        new web3.Transaction().add(instruction),
        [payer]
    )
    console.log(`You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
}

function generateAndPrintKeyPair() {
    const keyPair = web3.Keypair.generate();
    console.log(keyPair.secretKey.toString())
}

function initializeKeyPairFromPrivateKey(private_key: string): web3.Keypair {
    return web3.Keypair.fromSecretKey(
        bs58.decode(private_key)
    );
}

main().then(() => {
    console.log("Finished successfully")
}).catch((error) => {
    console.error(error);
})