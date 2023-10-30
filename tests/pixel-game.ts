import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PixelGame } from "../target/types/pixel_game";
import {
  PublicKey,
  Connection,
  Commitment,
  Keypair,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { BN } from "bn.js";
// import { devWallet, } from "./dev-wallet";
import { secretKey, importedPlayer2Keypair } from "./keypair";
import * as bs58 from 'bs58';

describe("pixel-game", () => {

const programId = new PublicKey('GK1fv7iaZFijE9YreSqoLy35CVUuBGLeKrmrniBfVT1C'); // Your program ID

const player1KeypairArray = new Uint8Array(secretKey);
const player2KeypairArray = new Uint8Array(importedPlayer2Keypair)

// Configure the client to use the cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.PixelGame as Program<PixelGame>;

// Create Keypairs from the secret key
const devKeypair = Keypair.fromSecretKey(player1KeypairArray);
const player2Keypair = Keypair.fromSecretKey(player2KeypairArray);

// Extract the public key as a string
const devPubkey = devKeypair.publicKey.toString();

// Transfer tokens to generated wallet
// *Only had to do this once when first time initializing P2*
// it("Transfer tokens to new walet!", async () => {
//   const web3 = require("@solana/web3.js");
//   const { Connection, SystemProgram, Transaction, clusterApiUrl, LAMPORTS_PER_SOL } = web3;

//   let connection = new Connection(clusterApiUrl("devnet"), "confirmed");

//   // Create the transfer instruction
//   const transferInstruction = SystemProgram.transfer({
//       fromPubkey: devKeypair.publicKey,
//       toPubkey: player2Keypair.publicKey,
//       lamports: LAMPORTS_PER_SOL,  // 1 SOL
//   });

//   // Get the recent blockhash
//   const { blockhash } = await connection.getRecentBlockhash();

//   // Create and populate the transaction
//   let transaction = new Transaction({ recentBlockhash: blockhash }).add(transferInstruction);

//   // Sign the transaction
//   transaction.sign(devKeypair);

//   // Send the transaction
//   const txid = await connection.sendRawTransaction(transaction.serialize());
//   console.log("Transaction ID:", txid);

//   // Wait for the transaction to be confirmed
//   await connection.confirmTransaction(txid);
//   console.log("Transaction confirmed!");
//   console.log("updated player2 pubkey: ", player2Keypair.publicKey.toBase58())
//   console.log("updated player2 keypair: ", player2Keypair)
// });

  // Finds the PLAYER1/DEVWALLET seed we generated on the rust side so we can use it here.
  const player1State = PublicKey.findProgramAddressSync(
    [Buffer.from("player_stats"), devKeypair.publicKey.toBytes()],
    program.programId
  )[0];

    // Finds the PLAYER2 seed we generated on the rust side so we can use it here.
    const player2State = PublicKey.findProgramAddressSync(
      [Buffer.from("player_stats"), player2Keypair.publicKey.toBytes()],
      program.programId
    )[0];


  it("Initialize Player1!", async () => {
    // Add your test here.

    const tx = await program.methods.initialize().accounts({
      playerStats: player2State,
      signer: player2Keypair.publicKey,
      systemProgram: SystemProgram.programId
    }).signers([player2Keypair])
    .rpc()
    .then(confirmTx);

    console.log("Your transaction signature", tx);
    console.log("Player1 Pubkey: ", devKeypair.publicKey);
    // console.log("Player2 Pubkey: ",player2Key.publicKey);
    console.log("DevStat Pubkey: ",player1State);
    console.log("Player2 Pubkey: ",player2Keypair.publicKey);

    // Fetch the data account and log results
  const data = await program.account.playerStats.fetch(player2State)
  console.log(`Reviewer: `,data.health.toString());
  });

  it("Check Health!", async () => {
    // Add your test here.
    console.log('Test has started');
  logPlayerHealth(player1State.toBase58());
  });

  async function logPlayerHealth(playerKey: string) {

    try {
      const playerStatsAccount = await program.account.playerStats.fetch(new PublicKey(playerKey));
      console.log(devKeypair.publicKey);
      console.log(`Player's Health: ${playerStatsAccount.health}`);
      console.log(`Player's Energy: ${playerStatsAccount.energy}`);

  } catch (error) {
      console.error('Failed to log player health:', error);
  }
}

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  );
  console.log(signature);
  return signature;
};
})
