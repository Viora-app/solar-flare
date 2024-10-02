import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfunding } from "../target/types/crowdfunding";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai"; 

describe("Crowdfunding Tests", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

  let projectPDA: PublicKey;
  let owner = provider.wallet.publicKey;
  let muzikieAddress = new PublicKey("3fh3nfHi22i93zq971bJFEC5o1NCaQYND4g33yMQS2ko");
  let projectId: anchor.BN;

  const softCap = new anchor.BN(1000); // Soft Cap
  const hardCap = new anchor.BN(5000); // Hard Cap
  const deadline = new anchor.BN(Date.now() / 1000 + 60 * 60 * 24);

  beforeEach(async () => {
    projectId = new anchor.BN(Math.floor(Math.random() * 1000));

    [projectPDA] = PublicKey.findProgramAddressSync(
      [projectId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initProject(
        projectId, softCap, hardCap, deadline, owner, muzikieAddress
      )
      .accounts({
        owner: provider.wallet.publicKey,
      })
      .rpc();
  });

  // 1. Test Initialization of the Contract
  it("Test Smart Contract Initialization with Correct Parameters", async () => {
    const projectAccount = await program.account.projectState.fetch(projectPDA);

    expect(projectAccount.projectId.toNumber()).to.equal(projectId.toNumber());
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

  // 3. Test Transitioning Contract Status to Published
  it("Test Transitioning Contract Status to Published", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    await program.methods
      .setPublish()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.status).to.have.property('published');
  });

  // 4. Contribute Within Published Status
  it("Test Contributions Within Published Contract", async () => {
    const tierId = new anchor.BN(1);
    const tierAmount = new anchor.BN(500);

    await program.methods
      .addContributionTier(tierId, tierAmount)
      .accounts({ project: projectPDA })
      .rpc();

    await program.methods
      .setPublish()
      .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
      .rpc();

    await program.methods
      .contribute(tierId, tierAmount)
      .accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
      .rpc();

    const projectAccount = await program.account.projectState.fetch(projectPDA);
    expect(projectAccount.currentFunding.toNumber()).to.equal(tierAmount.toNumber());
    expect(projectAccount.contributions.length).to.equal(1); // One contribution
    expect(projectAccount.contributions[0].contributionTierId.toNumber()).to.equal(tierId.toNumber());
  });

// 5. Test Contributions After SoftCap and HardCap Are Met
it("Test Contributions After SoftCap and HardCap Reached", async () => {
	const tierId = new anchor.BN(1);
	const tierAmount = new anchor.BN(2500); // Each contribution is 2500, 2 contributions to reach hard cap
  
	await program.methods
	  .addContributionTier(tierId, tierAmount)
	  .accounts({ project: projectPDA })
	  .rpc();
  
	// Publish the project
	await program.methods
	  .setPublish()
	  .accounts({ project: projectPDA, owner: provider.wallet.publicKey })
	  .rpc();
  
	// Fetch project account to verify the status
	let projectAccount = await program.account.projectState.fetch(projectPDA);
	console.log("Project status after publishing:", projectAccount.status); // Add this line to check status
  
	expect(projectAccount.status).to.have.property('published'); // Ensure the project is published
  
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
  
	projectAccount = await program.account.projectState.fetch(projectPDA);
  
	// Check if the project current funding equals the hard cap
	expect(projectAccount.currentFunding.toNumber()).to.equal(hardCap.toNumber());
  
	// Try contributing again, expect failure due to reaching the hard cap
	try {
	  await program.methods
		.contribute(tierId, tierAmount)
		.accounts({ project: projectPDA, contributor: provider.wallet.publicKey })
		.rpc();
	  throw new Error("Contribution succeeded but should have failed.");
	} catch (err) {
	  console.log("Actual error message:", err.message); // Log the error message
	  expect(err.message).to.include("HardCapReached");
	}
  });
  
  
});
