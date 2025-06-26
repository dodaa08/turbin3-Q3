import bs58 from "bs58";
import promptSync from "prompt-sync";

const prompt = promptSync();

// Convert Base58 → Wallet Bytes
function base58ToWallet() {
  const base58Key = prompt("Enter base58 private key: ");
  try {
    const walletBytes = bs58.decode(base58Key);
    console.log("✅ Wallet bytes:\n", walletBytes);
    console.log("\nSolana CLI array format:\n[", walletBytes.toString(), "]");
  } catch (err) {
    console.error("❌ Invalid base58 input:", err);
  }
}

// Convert Wallet Bytes → Base58
function walletToBase58() {
  const jsonInput = prompt("Enter wallet byte array (e.g. [12,34,...]): ");
  try {
    const byteArray = JSON.parse(jsonInput) as number[];
    const base58Key = bs58.encode(Uint8Array.from(byteArray));
    console.log("✅ Base58 private key:\n", base58Key);
  } catch (err) {
    console.error("❌ Invalid input array:", err);
  }
}

const option = prompt("Choose:\n1. Base58 → Wallet Bytes\n2. Wallet Bytes → Base58\n> ");
if (option === "1") base58ToWallet();
else if (option === "2") walletToBase58();
else console.log("❌ Invalid option.");
