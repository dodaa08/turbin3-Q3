import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq_new";

import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

const provider = new AnchorProvider(connection, new Wallet(keypair), { 
    commitment: "confirmed"
});

// Create the program instance - Alternative approach
const programId = new PublicKey(IDL.address);
const program = new Program<Turbin3Prereq>(
    IDL,
    provider
);

const accountSeeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key] = PublicKey.findProgramAddressSync(accountSeeds, program.programId);

const mintTs = Keypair.generate();
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

const authoritySeeds = [Buffer.from("collection"), mintCollection.toBuffer()];
const [authority] = PublicKey.findProgramAddressSync(authoritySeeds, program.programId);

const SYSTEM_PROGRAM_ID = SystemProgram.programId;
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

// Initialize function (commented out)
(async () => {
    try {
        const txhash = await program.methods
        .initialize("dodaa08")
        .accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([keypair])
        .rpc({ skipPreflight: false });
        console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();

// Submit TypeScript prerequisite
(async () => {
    try {
        console.log("Preparing to send submitTs transaction...");
        
        const tx = await program.methods.submitTs()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: authority,
                mplCoreProgram: MPL_CORE_PROGRAM_ID,
                systemProgram: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair, mintTs])
            .rpc({ skipPreflight: true });
        
        console.log(`✅ Success! TX: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (e) {
        console.error("Oops, something went wrong:");
        console.error(e);
    }
})();