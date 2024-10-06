import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Crowdfunding } from '../target/types/crowdfunding';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe('Crowdfunding Tests', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.crowdfunding as Program<Crowdfunding>;
  const owner = provider.wallet.publicKey;
  let projectPDA: PublicKey, projectId: anchor.BN;

  const softCap = new anchor.BN(100000000); // 0.1 SOL in lamports
  const hardCap = new anchor.BN(500000000); // 0.5 SOL in lamports
  const shortDeadline = new anchor.BN(Date.now() / 1000 + 600); // 10 minutes from now
  const artistKeypair = anchor.web3.Keypair.generate();

  beforeEach(async () => {
	projectId = new anchor.BN(Math.floor(Math.random() * 1000));
	[projectPDA] = PublicKey.findProgramAddressSync(
		[projectId.toArrayLike(Buffer, "le", 8)],
		program.programId
	  );
  
	// Fetch minimum rent-exempt balance for the account
	// const lamports = await provider.connection.getMinimumBalanceForRentExemption( /* add size of the account */ );
  
	// Initialize the project PDA account with the correct space and rent exemption
	// await program.provider.connection.requestAirdrop(owner, lamports);  // Ensure owner has enough SOL for rent
  
	await program.methods
	  .initProject(projectId, softCap, hardCap, shortDeadline, owner, artistKeypair.publicKey)
	  .accounts({
		// project: projectPDA,
		owner: owner,
		// systemProgram: anchor.web3.SystemProgram.programId, // Add this line to initialize PDA
	  })
	  .rpc();
  
	// Publish the project
	await program.methods
	  .setPublish()
	  .accounts({
		project: projectPDA,
		owner: owner,
	  })
	  .rpc();
  
	// Add contribution tiers
	await program.methods
	  .addContributionTier(new anchor.BN(1), new anchor.BN(100000000)) // Tier 1: 0.1 SOL
	  .accounts({
		project: projectPDA,
		owner: owner,
	  })
	  .rpc();
  
	await program.methods
	  .addContributionTier(new anchor.BN(2), new anchor.BN(200000000)) // Tier 2: 0.2 SOL
	  .accounts({
		project: projectPDA,
		owner: owner,
	  })
	  .rpc();
  });
  

  it('Distributes funds to the artist after the project is finalized with tier contribution', async () => {
    const contributor = anchor.web3.Keypair.generate();
	
	const muzikie_address = anchor.web3.Keypair.generate();


    // Airdrop SOL to the contributor
    const airdropSignature = await provider.connection.requestAirdrop(
      contributor.publicKey,
      30 * LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(airdropSignature);

    for (let i = 0; i < 3; i++) {
      // Make contributions to the project
      await program.methods
        .contribute(new anchor.BN(1), new anchor.BN(100000000))
        .accounts({
          project: projectPDA,
          contributor: contributor.publicKey,
		  muzikieAddress: muzikie_address.publicKey,
        })
        .signers([contributor])
        .rpc();

	  // Check if the contribution was successful
	  const projectState = await program.account.projectState.fetch(projectPDA);
		console.log('Project State:', projectState.currentFunding.toNumber());
		// get muzikie address balance
		const muzikieBalance = await provider.connection.getBalance(muzikie_address.publicKey);
		console.log('Muzikie Balance:', muzikieBalance);
	  }

	  const artistBalanceBefore = await provider.connection.getBalance(owner);
	  console.log('Artist artistBalanceBefore:', artistBalanceBefore);
    // Finalize the project
    await program.methods
      .finalizeProject()
      .accounts({
		project: projectPDA,
		muzikieAddress: muzikie_address.publicKey,
        owner: owner,
      })
	  .signers([muzikie_address])
      .rpc();

    // Assert the project state and balances
    const updatedProjectState = await program.account.projectState.fetch(projectPDA);
    expect(updatedProjectState.currentFunding.toNumber()).to.equal(0); // Check if funding has been reset

	const muzikieBalance = await provider.connection.getBalance(muzikie_address.publicKey);
		console.log('Muzikie Balance:', muzikieBalance);
    const artistBalanceAfter = await provider.connection.getBalance(owner);
    console.log('Artist Balance After:', artistBalanceAfter);

    // Additional assertions can be added here
  });
  
  
  it('Refunds contributor when project fails', async () => {
    const contributor = anchor.web3.Keypair.generate();
	const muzikie_address = anchor.web3.Keypair.generate();

    // Airdrop SOL to the contributor
    const airdropSignature = await provider.connection.requestAirdrop(
      contributor.publicKey,
      2 * LAMPORTS_PER_SOL, // Airdrop enough SOL for the contribution
    );
    await provider.connection.confirmTransaction(airdropSignature);

	
	
    // Contributor makes a contribution
    await program.methods
      .contribute(new anchor.BN(1), new anchor.BN(100000000)) // Contributing 0.1 SOL
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
		muzikieAddress: muzikie_address.publicKey,
      })
      .signers([contributor])
      .rpc();

    // Simulate project failure by directly updating the project's status to "Failed"
	const projectState = await program.account.projectState.fetch(projectPDA);

	
    // Manually setting the project status to Failed (this is allowed in test setup)
    projectState.status = { failed: {} }; // Simulate the "Failed" state in the test
    console.log('Project State:', projectState.status);
    // Contributor requests refund after project failure
    const contributorBalanceBefore = await provider.connection.getBalance(contributor.publicKey);

	const muzikieBalanceBefore = await provider.connection.getBalance(muzikie_address.publicKey);
	console.log('Muzikie Balance: muzikieBalanceBefore', muzikieBalanceBefore);
	
    await program.methods
      .refund(new anchor.BN(100000000)) // Request refund of 0.1 SOL
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
        muzikieAddress: muzikie_address.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([muzikie_address])
      .rpc();
	  const muzikieBalanceAfter= await provider.connection.getBalance(muzikie_address.publicKey);
	  console.log('Muzikie Balance: muzikieBalanceAfter', muzikieBalanceAfter);
    // Check contributor's balance after refund
    const contributorBalanceAfter = await provider.connection.getBalance(contributor.publicKey);
    expect(contributorBalanceAfter).to.be.greaterThan(contributorBalanceBefore); // Verify that refund was successful

    // Assert the project state to confirm that the funding has been decreased or is reset
    // const updatedProjectState = await program.account.projectState.fetch(projectPDA);
    // expect(updatedProjectState.currentFunding.toNumber()).to.be.lessThan(softCap.toNumber());
    // console.log('Refund Successful:', contributorBalanceAfter - contributorBalanceBefore);
  });
});
