import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PixelGame } from "../target/types/pixel_game";
import {
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { secretKey, player2SecretKey } from "./keypair";
import * as bs58 from 'bs58';

describe("pixel-game", () => {

const programId = new PublicKey('9swm7FNBS5GxMG6es1yoEYQhNmRjhbNozDp2u1ogybgd'); // Your program ID

const player1 = new Uint8Array(secretKey);
const player1Keypair = Keypair.fromSecretKey(player1);
const player1Pubkey = new PublicKey("DGnYpUq8cA5iHhLcban6RexM8h4bZmcgQ1qTzFTth58y")

const player2 = new Uint8Array(player2SecretKey);
const player2Keypair = Keypair.fromSecretKey(player2)
const player2Pubkey = new PublicKey("C4euDc2nahAbh7HAuX8Bm5ie5Mx57ETird3bCqsvXZFo")

// Configure the client to use the cluster.
anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.PixelGame as Program<PixelGame>;

// Finds the PLAYER1/DEVWALLET seed we generated on the rust side so we can use it here.
  const player1State = PublicKey.findProgramAddressSync(
    [Buffer.from("player_stats"), player1Pubkey.toBytes()],
    program.programId
  )[0];

// Finds the PLAYER2 seed we generated on the rust side so we can use it here.
    const player2State = PublicKey.findProgramAddressSync(
      [Buffer.from("player_stats"), player2Pubkey.toBytes()],
      program.programId
    )[0];

// IF YOU NEED TO AIRDROP TOKENS
it("Airdrop tokens to new walet!", async () => {
  await anchor
    .getProvider()
    .connection.requestAirdrop(
      player2Pubkey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    )
    .then(confirmTx);
  console.log("player2 pub:", player2Pubkey);
  console.log("player2 secret:", player2Keypair);
});

it("INITIALIZNG PLAYER1!", async () => {
  // Add your test here.
  await program.methods
    .initialize()
    .accounts({
      playerStats: player1State,
      signer: player1Pubkey,
      systemProgram: SystemProgram.programId,
    })
    .signers([player1Keypair])
    .rpc()
    .then(confirmTx);

 // Fetch the data account and log results
  const player1Data = await program.account.playerStats.fetch(player1State);
  // const player2Data = await program.account.playerStats.fetch(player2State)

  console.log(`Player1 Before Battle Health:` ,player1Data.health.toString());
  console.log("Player1 Attack Stat:" ,player1Data.attack.toString());
  console.log("Player1 Level:" ,player1Data.level.toString());
  console.log("Player1 Energy:" ,player1Data.energy.toString());
});

it("INITIALIZNG PLAYER2!", async () => {
  // Add your test here.
  await program.methods
    .initialize()
    .accounts({
      playerStats: player2State,
      signer: player2Pubkey,
      systemProgram: SystemProgram.programId,
    })
    .signers([player2Keypair])
    .rpc()
    .then(confirmTx);

 // Fetch the data account and log results
  const player2Data = await program.account.playerStats.fetch(player2State);

  console.log(`Player2 Before Battle Health:` ,player2Data.health.toString());
  console.log("Player2 Attack Stat:" ,player2Data.attack.toString());
  console.log("Player2 Level:" ,player2Data.level.toString());
  console.log("Player2 Energy:" ,player2Data.energy.toString());
});

it("Attack Player2!", async () => {
  // Add your test here.
  await program.methods
    .attack(player2Pubkey)
    .accounts({
      playerStats: player1State,
      defender: player2State,
      attacker: player1Pubkey,
      systemProgram: SystemProgram.programId,
    })
    .signers([player1Keypair])
    .rpc()
    .then(confirmTx);

      // Fetch the updated data for both attacker and defender
  const attackerData = await program.account.playerStats.fetch(player1State)
  const defenderData = await program.account.playerStats.fetch(player2State)

  // Log the updated health values
  console.log(`Attacker's health after battle: `, attackerData.health.toString());
  console.log(`Defender's health after battle: `, defenderData.health.toString());
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
  console.log("Tx Signature:", signature);
  return signature;
};
})
