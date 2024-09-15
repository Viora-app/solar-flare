import { AnchorProvider, Program, Idl, setProvider, Wallet, BN } from "@coral-xyz/anchor";
import idl from "../New_Blockchain/target/idl/crowdfunding.json";
import { Connection, PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

// Your secret key array
const secretKey = Uint8Array.from([
  40, 183, 189, 38, 23, 48, 7, 206, 254, 70, 79, 234, 51, 118, 234, 185, 196, 156, 74, 
  81, 203, 140, 203, 35, 192, 69, 147, 167, 159, 19, 230, 85, 39, 160, 186, 33, 213, 
  86, 199, 54, 78, 74, 206, 47, 83, 100, 184, 66, 188, 125, 170, 57, 240, 235, 43, 
  209, 124, 49, 178, 226, 194, 22, 136, 232
]);

// Generate Keypair from secret key
const keypair = Keypair.fromSecretKey(secretKey);

// Create a new Wallet instance that wraps the Keypair
const wallet = new Wallet(keypair);

// Set up the Solana connection to localhost
const connection = new Connection("http://localhost:8899");

// Create the provider using the connection and wallet
const provider = new AnchorProvider(connection, wallet, {});
setProvider(provider);

// Specify the program ID
const programId = new PublicKey("6FdUicaFVWJ3oTRPsAgQR3PAXi6qm2b7Wtx3R9kfVd7e");  // Ensure this matches your declared program ID in lib.rs

// Initialize the program with the IDL and program ID
const program = new Program(idl as Idl, provider);

(async () => {
    try {
        // Prepare the project title (max length 20 bytes)
        const title = "My First Project";
		
        // Find Program-Derived Address (PDA) for the project
        const [projectPda, bump] = await PublicKey.findProgramAddress(
            [Buffer.from(title), wallet.publicKey.toBuffer()],  // Same seeds as in Rust program
            program.programId
        );

        // Call the create_project instruction (if project not yet created)
        const txCreate = await program.methods.createProject(title).accounts({
            project: projectPda,
            artist: wallet.publicKey,
            systemProgram: SystemProgram.programId,
        }).signers([keypair]).rpc();

        console.log("Project creation transaction signature:", txCreate);

        // Call the contribute function to contribute to the project
        const contributionAmount = 1000000;  // Example: contributing 1 SOL (1,000,000 lamports)
		const txContribute = await program.methods.contribute(new BN(contributionAmount)).accounts({
            project: projectPda,
            contributor: wallet.publicKey,
            systemProgram: SystemProgram.programId,
        }).signers([keypair]).rpc();

        console.log("Contribution transaction signature:", txContribute);

        // Fetch and log the transaction details
        const txDetails = await connection.getTransaction(txContribute, { commitment: "confirmed" });
        console.log("Transaction details:", txDetails);

    } catch (err) {
        console.error("Error in transaction:", err);
    }
})();
