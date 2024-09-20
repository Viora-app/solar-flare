import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfunding } from "../target/types/crowdfunding";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("crowdfunding", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

  it("Initializes the project, adds a contribution tier, sets the project live, and makes a contribution with SOL transfer", async () => {
    // Define test parameters
    const projectId = new anchor.BN(2); // Sample project ID
    const softCap = new anchor.BN(1000); // Sample soft cap (in lamports)
    const hardCap = new anchor.BN(5000); // Sample hard cap (in lamports)
    const deadline = new anchor.BN(Date.now() / 1000 + 60 * 60 * 24); // Deadline 24 hours from now
    const walletAddress = provider.wallet.publicKey; // Owner's wallet address
    const muzikieAddress = new PublicKey("3fh3nfHi22i93zq971bJFEC5o1NCaQYND4g33yMQS2ko"); // Replace with Muzikie wallet address

    // Use the synchronous version to find the program-derived address (PDA)
    const [projectPDA, bump] = PublicKey.findProgramAddressSync(
      [projectId.toArrayLike(Buffer, "le", 8)], // seed is the project ID
      program.programId
    );

    // Initialize the project
    const initTx = await program.methods
      .initProject(
        projectId,         // Project ID
        softCap,           // Soft Cap
        hardCap,           // Hard Cap
        deadline,          // Deadline
        walletAddress,     // Owner Wallet
        muzikieAddress     // Muzikie Wallet
	)
	.accounts({
	//   project: projectPDA,  // Use the PDA as the project account
	  owner: muzikieAddress,  // Owner/payer (not to be initialized as the program account)
	//   systemProgram: SystemProgram.programId,  // System program is needed for creating accounts
	})
	// .signers([provider.wallet.peyer])  // This ensures that the owner's wallet signs the transaction
	.rpc();

    console.log("Project initialized. Transaction signature:", initTx);

    // Fetch the project account to verify the state
    const projectAccount = await program.account.projectState.fetch(projectPDA);
    console.log("Project initialized with details:", projectAccount);

    // Add a contribution tier
    const tierId = new anchor.BN(1); // Tier ID
    const tierAmount = new anchor.BN(500); // Tier amount (in lamports)

    const addTierTx = await program.methods
      .addContributionTier(
        tierId,    // Contribution tier ID
        tierAmount // Contribution amount
      )
      .accounts({
        project: projectPDA,    // Project account (PDA)
        // owner: walletAddress,   // Owner's wallet (signer)
      })
      .rpc();

    console.log("Contribution tier added. Transaction signature:", addTierTx);

    // Fetch the project account again to verify the new state
    const updatedProjectAccount = await program.account.projectState.fetch(projectPDA);
    console.log("Updated project state after adding tier:", updatedProjectAccount);

    // Set the project live
    const setLiveTx = await program.methods
      .setLive()
      .accounts({
        project: projectPDA,
        owner: walletAddress,
      })
      .rpc();

    console.log("Project set to live. Transaction signature:", setLiveTx);

    // Fetch the project account again to verify the new state
    const liveProjectAccount = await program.account.projectState.fetch(projectPDA);
    console.log("Updated project state after setting live:", liveProjectAccount);

    // Make a contribution with SOL transfer
    const contributionAmount = new anchor.BN(500); // Contribution amount (should match the tier amount)
    
    const preBalance = await provider.connection.getBalance(walletAddress); // Fetch contributor's balance before transfer

    const contributeTx = await program.methods
      .contribute(tierId, contributionAmount)
      .accounts({
        project: projectPDA,
        contributor: walletAddress,
      })
      .rpc();

    console.log("Contribution made with SOL transfer. Transaction signature:", contributeTx);

    // Fetch the project account again to verify the contribution
    const contributedProjectAccount = await program.account.projectState.fetch(projectPDA);
    console.log("Updated project state after contribution:", contributedProjectAccount);

    // Check the contribution
    if (contributedProjectAccount.currentFunding.toNumber() !== contributionAmount.toNumber()) {
      throw new Error("Contribution amount does not match the expected value");
    }

    if (contributedProjectAccount.contributions.length !== 1) {
      throw new Error("Contribution was not recorded successfully");
    }

    const contribution = contributedProjectAccount.contributions[0];
    if (contribution.contributionTierId.toNumber() !== tierId.toNumber() || 
        contribution.senderAddress.toBase58() !== walletAddress.toBase58()) {
      throw new Error("Contribution data does not match expected values");
    }

    // Verify the contributor's balance decreased
    const postBalance = await provider.connection.getBalance(walletAddress); // Fetch balance after transfer
    if (postBalance >= preBalance) {
      throw new Error("Contribution failed, no SOL was transferred");
    } else {
      console.log(`Contributor's balance decreased by ${preBalance - postBalance} lamports`);
    }

    console.log("SOL transfer successful!");

    // Check final status
    const finalStatus = contributedProjectAccount.status;
    if (!finalStatus.live) {
      throw new Error("Project is not live after the contribution");
    }
  });
});
