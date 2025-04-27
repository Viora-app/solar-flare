import * as anchor from '@coral-xyz/anchor';
// import { Program } from '@coral-xyz/anchor';
import { Wallet, Program } from '@coral-xyz/anchor';

import { Crowdfunding } from '../target/types/crowdfunding';
import {
  Connection,
  Keypair,
  PublicKey,
} from '@solana/web3.js';

import {
    TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

const TOKEN_PROGRAM: typeof TOKEN_PROGRAM_ID = TOKEN_PROGRAM_ID;

// describe('Finalize Project Scenarios', () => {
// //   const provider = anchor.AnchorProvider.env();
// const connection = new Connection('https://api.devnet.solana.com/', 'confirmed');

// const privateKey =
// '3MS8Swu7GYARShEGr21qkgzBEb7ghs9xQievW116rZHnL9Sqq1ASnsSBq1K8RTZ13vUWyrCmKney5XoaY117tx3P';
// const base58privatekey = bs58.decode(privateKey);
// const keyPair = Keypair.fromSecretKey(base58privatekey);
// const provider = new anchor.AnchorProvider(connection, new Wallet(keyPair), {
//     preflightCommitment: 'confirmed',
//   });