import { Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
    "@solana/web3.js";

import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const to = new PublicKey("FXtW7PAK8KtFKBsBiyFMK2BrrU8dt9gHPzoVrBDNkWgf");

const connection = new Connection("https://api.devnet.solana.com");


// First Transfer  

// (async () => {
//     try {
//     const transaction = new Transaction().add(
//     SystemProgram.transfer({
//     fromPubkey: from.publicKey,
//     toPubkey: to,
//     lamports: LAMPORTS_PER_SOL / 100,
//     })
//     );
//     transaction.recentBlockhash = (
//     await connection.getLatestBlockhash('confirmed')
//     ).blockhash;
//     transaction.feePayer = from.publicKey;
//     // Sign transaction, broadcast, and confirm
//     const signature = await sendAndConfirmTransaction(
//     connection,
//     transaction,
//     [from]
//     );
//     console.log(`Success! Check out your TX here:
//     https://explorer.solana.com/tx/${signature}?cluster=devnet`);
//     } catch (e) {
//     console.error(`Oops, something went wrong: ${e}`);
//     }
// })();



// Second Transfer Clean the balance

(async () => {
    try {
      const balance = await connection.getBalance(from.publicKey);
  
      const tempTx = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance,
        })
      );
  
      const blockhash = await connection.getLatestBlockhash("confirmed");
      tempTx.recentBlockhash = blockhash.blockhash;
      tempTx.feePayer = from.publicKey;
  
      const fee =
        (
          await connection.getFeeForMessage(tempTx.compileMessage(), "confirmed")
        ).value || 0;
  
      // Create the final transaction with balance - fee
      const finalTx = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance - fee,
        })
      );
  
      const sig = await sendAndConfirmTransaction(connection, finalTx, [from]);
      console.log(`✅ All SOL transferred!`);
      console.log(
        `Explorer: https://explorer.solana.com/tx/${sig}?cluster=devnet`
      );
    } catch (e) {
      console.error("❌ Transfer failed:", e);
    }
  })();