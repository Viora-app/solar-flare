import { AnchorProvider, Program, Idl, setProvider, Wallet, BN } from "@coral-xyz/anchor";
import idl from "./target/idl/crowdfunding.json";
import { Connection, PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

// Your secret key array
const secretKey = Uint8Array.from([205, 169, 103, 140, 221, 106, 201, 148, 115, 177, 107, 210, 4, 211, 230, 142, 21, 97, 151, 116, 116, 135, 159, 156, 175, 231, 111, 166, 26, 138, 39, 218, 184, 65, 162, 203, 103, 133, 215, 216, 218, 118, 135, 141, 135, 199, 71, 79, 77, 157, 245, 182, 122, 68, 56, 113, 120, 56, 125, 238, 219, 131, 150, 161]);
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
const programId = new PublicKey("3mDN8aNgwdM6BUBDiaxbmpRSwWoLeyrMtak5fbLsWtkW");  // Ensure this matches your declared program ID in lib.rs

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
