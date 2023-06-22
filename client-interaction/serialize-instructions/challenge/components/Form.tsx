import { FC } from 'react'
import { StudentIntro } from '../models/StudentIntro'
import { useState } from 'react'
import { Box, Button, FormControl, FormLabel, Input, Textarea } from '@chakra-ui/react'
import * as web3 from '@solana/web3.js'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'

const STUDENT_INTRO_PROGRAM_ID = 'HdE95RSVsdb315jfJtaykXhXY478h53X6okDupVfY9yf'

export const Form: FC = () => {
    const [name, setName] = useState('')
    const [message, setMessage] = useState('')
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();


    const handleSubmit = (event: any) => {
        event.preventDefault()
        const studentIntro = new StudentIntro(name, message)
        handleTransactionSubmit(studentIntro)
    }

    const handleTransactionSubmit = async (studentIntro: StudentIntro) => {
        // Check that publicKey exists to ensure that the user has connected their wallet.
        if (!publicKey) {
            alert('Please connect your wallet!')
            return
        }
        // Get all of the accounts that the transaction will read from or write to. pda is the program data account
        const [pda] = await web3.PublicKey.findProgramAddress(
            [publicKey.toBuffer()],
            new web3.PublicKey(STUDENT_INTRO_PROGRAM_ID)
        )

        // Create a new Instruction object that includes all of these accounts in the keys argument, includes the buffer in the data argument, and includes the program’s public key in the programId argument.
        const instruction = new web3.TransactionInstruction({
            keys: [
                {
                    pubkey: publicKey,
                    isSigner: true,
                    isWritable: false
                },
                {
                    pubkey: pda,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: web3.SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                }
            ],
            data: studentIntro.serialize(),
            programId: new web3.PublicKey(STUDENT_INTRO_PROGRAM_ID)
        })

        try {
            // Add the instruction from the last step to the transaction.
            // Call sendTransaction, passing in the assembled transaction.
            let txid = await sendTransaction(new web3.Transaction().add(instruction), connection)
            console.log(`Transaction submitted: https://explorer.solana.com/tx/${txid}?cluster=devnet`)
        } catch (e) {
            alert(JSON.stringify(e))
        }
    }

    return (
        <Box
            p={4}
            display={{ md: "flex" }}
            maxWidth="32rem"
            borderWidth={1}
            margin={2}
            justifyContent="center"
        >
            <form onSubmit={handleSubmit}>
                <FormControl isRequired>
                    <FormLabel color='gray.200'>
                        What do we call you?
                    </FormLabel>
                    <Input
                        id='name'
                        color='gray.400'
                        onChange={event => setName(event.currentTarget.value)}
                    />
                </FormControl>
                <FormControl isRequired>
                    <FormLabel color='gray.200'>
                        What brings you to Solana, friend?
                    </FormLabel>
                    <Textarea
                        id='message'
                        color='gray.400'
                        onChange={event => setMessage(event.currentTarget.value)}
                    />
                </FormControl>
                <Button width="full" mt={4} type="submit">
                    Submit
                </Button>
            </form>
        </Box>
    );
}