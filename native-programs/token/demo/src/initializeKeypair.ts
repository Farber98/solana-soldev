import * as web3 from "@solana/web3.js"
import * as bs58 from 'bs58';
import dotenv from "dotenv"
dotenv.config()

export function initializeKeyPairFromPrivateKey(private_key: string): web3.Keypair {
  return web3.Keypair.fromSecretKey(
    bs58.decode(private_key)
  );
}
