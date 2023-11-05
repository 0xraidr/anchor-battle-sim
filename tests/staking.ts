import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
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
import { devWallet } from "./dev-wallet";

describe("staking testing!", () => {

const player1KeypairArray = new Uint8Array(secretKey);
const player2KeypairArray = new Uint8Array(importedPlayer2Keypair)

// Configure the client to use the cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.PixelGame as Program<Staking>;

// Create Keypairs from the secret key
const devKeypair = Keypair.fromSecretKey(player1KeypairArray);
const player2Keypair = Keypair.fromSecretKey(player2KeypairArray);

// Extract the public key as a string
const devPubkey = devKeypair.publicKey.toString();

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


  it("what am i even testing!", async () => {
    // Add your test here.

    const tx = await program.methods.initialize().accounts({
      playerStats: player1State,
      signer: devKeypair.publicKey,
      systemProgram: SystemProgram.programId
    }).signers([devKeypair])
    .rpc()
    .then(confirmTx);

  });

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
  console.log("tester:", signature);
  return signature;
};
})
