import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Crowdfunding } from '../target/types/crowdfunding';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe('Refund Scenarios', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.crowdfunding as Program<Crowdfunding>;
  const owner = provider.wallet.publicKey;

  let projectId: anchor.BN;
  let projectPDA: PublicKey;
  let softCap: anchor.BN;
  let hardCap: anchor.BN;
  let shortDeadline: anchor.BN;
  const escrow = anchor.web3.Keypair.generate();
  const muzikieAddress = anchor.web3.Keypair.generate();
  const contributor = anchor.web3.Keypair.generate();

  beforeEach(async () => {
    projectId = new anchor.BN(Math.floor(Math.random() * 1000));
    softCap = new anchor.BN(100000000); // 0.1 SOL
    hardCap = new anchor.BN(200000000); // 0.2 SOL
    shortDeadline = new anchor.BN(Date.now() / 1000 + 5); // 5 seconds from now

    [projectPDA] = PublicKey.findProgramAddressSync(
      [projectId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Airdrop SOL to the escrow and contributor
    const airdropEscrow = await provider.connection.requestAirdrop(
      escrow.publicKey,
      10 * LAMPORTS_PER_SOL // 10 SOL to escrow
    );
    await provider.connection.confirmTransaction(airdropEscrow);

    const airdropContributor = await provider.connection.requestAirdrop(
      contributor.publicKey,
      2 * LAMPORTS_PER_SOL // 2 SOL to contributor
    );
    await provider.connection.confirmTransaction(airdropContributor);

    // Initialize the new project
    await program.methods
      .initProject(projectId, softCap, hardCap, shortDeadline, owner, muzikieAddress.publicKey)
      .accounts({
        owner: owner,
      })
      .rpc();
      
	      // Add a contribution tier
    await program.methods
      .addContributionTier(new anchor.BN(1), new anchor.BN(100000000)) // 0.1 SOL
      .accounts({
        project: projectPDA,
        owner: owner,
      })
      .rpc();
	  
	  await program.methods
      .addContributionTier(new anchor.BN(2), new anchor.BN(50000000)) // 0.1 SOL
      .accounts({
        project: projectPDA,
        owner: owner,
      })
      .rpc();
	  
	  await program.methods
	  .setPublish()
	  .accounts({
		  project: projectPDA,
		  owner: owner,
	  })
	  .rpc();

  });

  it('Successfully refunds a contributor when project is failing', async () => {
    const refundAmount = new anchor.BN(100000000); 

await new Promise((resolve) => setTimeout(resolve, 6000));

      await program.methods
      .contribute(new anchor.BN(2), new anchor.BN(50000000)) // Contribute 0.05 SOL
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
        escrow: escrow.publicKey,
      })
      .signers([contributor])
      .rpc();
	  
      await new Promise((resolve) => setTimeout(resolve, 6000));

       await program.methods
      .finalizeProject()
      .accounts({
        project: projectPDA,
        muzikieAddress: muzikieAddress.publicKey,
        escrow: escrow.publicKey,
        owner: owner,
      })
      .signers([escrow])
      .rpc();

    await program.methods
      .refund(refundAmount)
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
		escrow: escrow.publicKey,
      })
      .signers([escrow])
      .rpc();

    // Assert the contributor's balance has increased by the refund amount
    const contributorBalance = await provider.connection.getBalance(contributor.publicKey);
    expect(contributorBalance).to.be.greaterThan(2 * LAMPORTS_PER_SOL); // check if balance is greater than initial
  });

  it('Fails refund when project is not failing', async () => {
    // Simulate successful project status
    const project = await program.account.projectState.fetch(projectPDA);
    // project.status = { successful: {} }; // Update project status to successful

    const refundAmount = new anchor.BN(100000000); // 0.1 SOL

    // Attempt to refund the contribution and check for rejection
    try {
      await program.methods
        .refund(refundAmount)
        .accounts({
          project: projectPDA,
          contributor: contributor.publicKey,
          escrow: escrow.publicKey,
        })
        .signers([escrow])
        .rpc();
      // If the transaction succeeds, fail the test
      expect.fail("Expected transaction to fail, but it succeeded");
    } catch (error) {
      expect(error).to.exist;
      expect(error.message).to.include("ProjectNotFailing");
    }
  });

//   it('Fails refund when escrow does not have enough funds', async () => {
//     const refundAmount = new anchor.BN(200000000); // 0.2 SOL, greater than the escrow balance
// 
//     const project = await program.account.projectState.fetch(projectPDA);
//     project.status = { failing: {} }; 
//     // Attempt to refund the contribution and check for rejection
//     try {
//       await program.methods
//         .refund(refundAmount)
//         .accounts({
//           project: projectPDA,
//           contributor: contributor.publicKey,
//           escrow: escrow.publicKey,
//         })
//         .signers([escrow])
//         .rpc();
//       // If the transaction succeeds, fail the test
//       expect.fail("Expected transaction to fail, but it succeeded");
//     } catch (error) {
//       expect(error).to.exist;
// 	  console.log(error);
// 	  console.log(error.message);
//       expect(error.message).to.include("InsufficientFunds");
//     }
//   });
});
