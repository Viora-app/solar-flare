import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfunding } from "../target/types/crowdfunding";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai"; 

describe("Crowdfunding Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

  let projectPDA: PublicKey;
  let owner = provider.wallet.publicKey;
  let muzikieAddress = new PublicKey("3fh3nfHi22i93zq971bJFEC5o1NCaQYND4g33yMQS2ko");
  let projectId: anchor.BN;

  // Sample parameters for all tests
  const softCap = new anchor.BN(1000); // Soft Cap
  const hardCap = new anchor.BN(5000); // Hard Cap
  const deadline = new anchor.BN(Date.now() / 1000 + 60 * 60 * 24);

  beforeEach(async () => {
    projectId = new anchor.BN(Math.floor(Math.random() * 1000));

    [projectPDA] = PublicKey.findProgramAddressSync(
      [projectId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Initialize project before each test
    await program.methods
      .initProject(
        projectId, softCap, hardCap, deadline, owner, muzikieAddress
      )
      .accounts({
        owner: provider.wallet.publicKey,
        // systemProgram: SystemProgram.programId,
      })
      .rpc();
  });

  // 1. Test Initialization of the Contract
  it("Test Smart Contract Initialization with Correct Parameters", async () => {
    const projectAccount = await program.account.projectState.fetch(projectPDA);

    expect(projectAccount.projectId.toNumber()).to.equal(projectId.toNumber());  // Update this line
    expect(projectAccount.softCap.toNumber()).to.equal(softCap.toNumber());
    expect(projectAccount.hardCap.toNumber()).to.equal(hardCap.toNumber());
    expect(projectAccount.deadline.toNumber()).to.equal(deadline.toNumber());
    expect(projectAccount.status).to.have.property('draft');
    expect(projectAccount.currentFunding.toNumber()).to.equal(0); // Initial funding should be 0
    expect(projectAccount.contributionTiers.length).to.equal(0); // No contribution tiers at init
  });

  // 2. Add Contribution Tier Successfully
  it("Test Adding Contribution Tiers in Draft Status", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    // Add a contribution tier
    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.contributionTiers.length).to.equal(1); // Should have one tier
    const tier = projectAccount.contributionTiers[0];
    expect(tier.tierId.toNumber()).to.equal(tierId.toNumber());
    expect(tier.amount.toNumber()).to.equal(tierAmount.toNumber());
  });

  // 3. Test Transitioning Contract Status to Live
  it("Test Transitioning Contract Status to Live", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    // Add a contribution tier
    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    // Set the project live
    await program.methods
      .setLive()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    // Fetch project and check the status
    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.status).to.have.property('live');
  });

  // 4. Contribute Within Constraints
  it("Test Contributions Within Live Contract", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    // Add a contribution tier
    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    // Set the project live
    await program.methods
      .setLive()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    // Contribute
    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    // Fetch the project and check funding and contributions
    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.currentFunding.toNumber()).to.equal(tierAmount.toNumber());
    expect(projectAccount.contributions.length).to.equal(1); // One contribution
    expect(projectAccount.contributions[0].contributionTierId.toNumber()).to.equal(tierId.toNumber());
  });

// Test Contributions After Deadline or HardCap Reached
 // 5. Test Contributions After Deadline or HardCap Reached
 it("Test Contributions After Deadline or HardCap Reached", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(2500); // Each contribution is 2500, 2 contributions to reach hard cap

    // Add a contribution tier
    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    // Set the project live before contributing
    await program.methods
      .setLive()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    // Contribute first half of the hard cap
    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    // Contribute second half of the hard cap to reach it
    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    // Fetch the project state after reaching the hard cap
    const projectAccount = await program.account.projectState.fetch(projectPDA);
    
    // Check if the hard cap was hit and status is "successful"
    expect(projectAccount.status).to.have.property('successful');

    // Try contributing again, expect failure due to reaching the hard cap
    try {
      await program.methods
        .contribute(tierId, tierAmount)
        .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
        .rpc();
      throw new Error("Contribution succeeded but should have failed.");
    } catch (err) {
      // Check for the correct error message
      if (err.message.includes("HardCapReached")) {
        expect(err.message).to.include("HardCapReached");
      } else if (err.message.includes("ProjectNotLive")) {
        // The project might no longer be live after the hard cap is reached
        expect(err.message).to.include("ProjectNotLive");
      } else {
        // Fail the test if neither expected error is found
        throw new Error(`Unexpected error message: ${err.message}`);
      }
    }
  });
  
  

  // 6. Test Contract Status Set to Successful When HardCap is Met
  it("Test Contract Status Set to Successful When HardCap is Met", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(2500); // Each contribution is 2500

    // Add a contribution tier
    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    // Set the project live
    await program.methods
      .setLive()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    // Contribute twice to reach the hard cap
    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    // Fetch project and check the status
    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.status).to.have.property('successful');
  });

 // 7. Test Failure to Add Contribution Tier When Contract is Not in Draft Status
it("Test Failure to Add Contribution Tier When Contract is Not in Draft Status", async () => {
	const tierId = new anchor.BN(1);
	const tierAmount = new anchor.BN(500);
  
	// Add a contribution tier while in draft status
	await program.methods
	  .addContributionTier(tierId, tierAmount)
	  .accounts({ project: projectPDA })
	  .rpc();
  
	// Set the project live
	await program.methods
	  .setLive()
	  .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
	  .rpc();
    
	// Try adding a tier after status change, expect failure
	try {
	  await program.methods
		.addContributionTier(tierId, tierAmount)
		.accounts({ project: projectPDA })
		.rpc();
	  throw new Error("Adding tier succeeded but should have failed.");
	} catch (err) {
	  // Ensure the error is related to the project not being in draft status
	  expect(err.message).to.include("ProjectNotInDraft");
	}
  });
  
  // 8. Test Failure to Add More Than 5 Contribution Tiers
  it("Test Failure to Add More Than 5 Contribution Tiers", async () => {
    const tierAmount = new anchor.BN(500);

    // Add 5 tiers
    for (let i = 1; i <= 5; i++) {
      await program.methods
        .addContributionTier(new anchor.BN(i), tierAmount)
        .accounts({ project: projectPDA })
        .rpc();
    }

    // Try adding a 6th tier, expect failure
    try {
      await program.methods
        .addContributionTier(new anchor.BN(6), tierAmount)
        .accounts({ project: projectPDA })
        .rpc();
      throw new Error("Adding 6th tier succeeded but should have failed.");
    } catch (err) {
      expect(err.message).to.include("MaxContributionTiersReached");
    }
  });

  // 9. Test Failure to Set Contract Live Without Contribution Tiers
  it("Test Failure to Set Contract Live Without Contribution Tiers", async () => {
    try {
      await program.methods
        .setLive()
        .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
        .rpc();
      throw new Error("Setting live succeeded but should have failed.");
    } catch (err) {
      expect(err.message).to.include("NoContributionTiers");
    }
  });

  // 10. Test Failure to Contribute if Contract is Not in Live Status
  it("Test Failure to Contribute if Contract is Not in Live Status", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    // Try contributing when project is still in draft, expect failure
    try {
      await program.methods
        .contribute(tierId, tierAmount)
        .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
        .rpc();
      throw new Error("Contribution succeeded but should have failed.");
    } catch (err) {
      expect(err.message).to.include("ProjectNotLive");
    }
  });
});
