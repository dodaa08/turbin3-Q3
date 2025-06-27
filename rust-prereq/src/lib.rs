#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}};
    use solana_client::rpc_client::RpcClient;
    use solana_program::pubkey::Pubkey;
    use solana_sdk::system_instruction::transfer;
    use solana_sdk::transaction::Transaction;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::system_program;
    use bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;

    use solana_sdk::{message::Message};

    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey());
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Your wallet file format is:");
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let base58 = bs58::encode(wallet).into_string();
        println!("Your Base58-encoded private key is:");
        println!("{}", base58);
    }

    #[test]
    fn claim_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("FXtW7PAK8KtFKBsBiyFMK2BrrU8dt9gHPzoVrBDNkWgf").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

        let tx = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        match rpc_client.send_and_confirm_transaction(&tx) {
            Ok(sig) => println!("Success! TX: https://explorer.solana.com/tx/{}?cluster=devnet", sig),
            Err(err) => eprintln!("Transfer failed: {}", err),
        }
    }

    #[test]
    fn verify_signature() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let message = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message);

        if sig.verify(&keypair.pubkey().to_bytes(), message) {
            println!("Signature verified!");
        } else {
            println!("Signature verification failed!");
        }
    }

    #[test]
    fn transfer_all_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("FXtW7PAK8KtFKBsBiyFMK2BrrU8dt9gHPzoVrBDNkWgf").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
    
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get blockhash");
    
        // Fetch balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
    
        // Build a dummy message to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        
    
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");
    
        if balance <= fee {
            println!("Not enough balance to cover transaction fee.");
            return;
        }
    
        // Now transfer (balance - fee)
        let tx = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );
    
        match rpc_client.send_and_confirm_transaction(&tx) {
            Ok(sig) => println!(
                "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
                sig
            ),
            Err(err) => eprintln!("Transfer failed: {}", err),
        }
    }


    #[test]
fn send_raw_txn() {
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("dev-wallet.json")
        .expect("Couldn't find wallet file");
    
    let mint = Keypair::new();

    let turbin3_prereq_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection =
        Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    // Generate PDA for prerequisite account
    let signer_pubkey = signer.pubkey(); 
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

    // Generate authority PDA for collection
    let authority_seeds = &[b"collection", collection.as_ref()];
    let (authority, _authority_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

    // Instruction discriminator for submit_rs
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),        // user signer
        AccountMeta::new(prereq_pda, false),            // PDA account
        AccountMeta::new(mint.pubkey(), true),          // mint keypair
        AccountMeta::new(collection, false),            // collection
        AccountMeta::new_readonly(authority, false),    // authority (PDA)
        AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
        AccountMeta::new_readonly(system_program, false),   // system program
    ];

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    
    println!(
        "🎉 Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    
    println!("🎨 Your Rust NFT mint: {}", mint.pubkey());
    println!("📝 Prereq account: {}", prereq_pda);
}

}
