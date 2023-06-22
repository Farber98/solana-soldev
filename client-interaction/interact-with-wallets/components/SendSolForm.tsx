import { FC } from 'react'
import styles from '../styles/Home.module.css'
import * as web3 from '@solana/web3.js'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'


export const SendSolForm: FC = () => {
    const { publicKey, sendTransaction } = useWallet()
    const { connection } = useConnection()

    const sendSol = event => {
        event.preventDefault()

        const recipientPubKey = new web3.PublicKey(event.target.recipient.value)

        const sendSolInstruction = web3.SystemProgram.transfer({
            fromPubkey: publicKey,
            toPubkey: recipientPubKey,
            lamports: web3.LAMPORTS_PER_SOL * 0.1
        })

        const transaction = new web3.Transaction().add(sendSolInstruction);
        sendTransaction(transaction, connection).then(sig => {
            console.log(`Sent ${event.target.amount.value} SOL to ${event.target.recipient.value}`)
            console.log(`You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/${sig}?cluster=devnet`)
        })
    }

    return (
        <div>
            <form onSubmit={sendSol} className={styles.form}>
                <label htmlFor="amount">Amount (in SOL) to send:</label>
                <input id="amount" type="text" className={styles.formField} placeholder="e.g. 0.1" required />
                <br />
                <label htmlFor="recipient">Send SOL to:</label>
                <input id="recipient" type="text" className={styles.formField} placeholder="e.g. 4Zw1fXuYuJhWhu9KLEYMhiPEiqcpKd6akw3WRZCv84HA" required />
                <button type="submit" className={styles.formButton}>Send</button>
            </form>
        </div>
    )
}