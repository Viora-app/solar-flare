import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Crowdfunding } from '../target/types/crowdfunding';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe('Finalize Project Scenarios', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.crowdfunding as Program<Crowdfunding>;
  const owner = provider.wallet.publicKey;

  // Generate new project parameters for each test
  let projectId: anchor.BN;
  let projectPDA: PublicKey;
  let softCap: anchor.BN;
  let hardCap: anchor.BN;
  let shortDeadline: anchor.BN;
  const escrow = anchor.web3.Keypair.generate();
  const muzikieAddress = anchor.web3.Keypair.generate();
  const contributor = anchor.web3.Keypair.generate();

  beforeEach(async () => {
    // Generate new project details for each test
    projectId = new anchor.BN(Math.floor(Math.random() * 1000));
    softCap = new anchor.BN(100000000); // 0.1 SOL in lamports
    hardCap = new anchor.BN(200000000); // 0.2 SOL in lamports
    shortDeadline = new anchor.BN(Date.now() / 1000 + 5); // 5 seconds from now

    // Find PDA for the project
    [projectPDA] = PublicKey.findProgramAddressSync(
      [projectId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Airdrop SOL to the contributor
    const airdropSignature = await provider.connection.requestAirdrop(
      contributor.publicKey,
      10 * LAMPORTS_PER_SOL, // 10 SOL airdrop
    );
    await provider.connection.confirmTransaction(airdropSignature);

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

  it('Finalizes a project with soft cap reached', async () => {
    // Contribute to reach the soft cap
    await program.methods
      .contribute(new anchor.BN(1), new anchor.BN(100000000)) // Contribute 0.1 SOL
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
        escrow: escrow.publicKey,
      })
      .signers([contributor])
      .rpc();

    // Wait for the deadline to pass (5 seconds)
    await new Promise((resolve) => setTimeout(resolve, 6000));

    // Finalize the project
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

    // Assert that the project status is 'Successful'
    const project = await program.account.projectState.fetch(projectPDA);
    expect(project.status).to.deep.equal({ successful: {} });
  });

  it('Finalizes a project that failed to reach soft cap', async () => {
    // Contribute but do not reach the soft cap
    await program.methods
      .contribute(new anchor.BN(2), new anchor.BN(50000000)) // Contribute 0.05 SOL
      .accounts({
        project: projectPDA,
        contributor: contributor.publicKey,
        escrow: escrow.publicKey,
      })
      .signers([contributor])
      .rpc();

    // Wait for the deadline to pass (5 seconds)
    await new Promise((resolve) => setTimeout(resolve, 6000));

    // Finalize the project
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

    // Assert that the project status is 'Failing'
    const project = await program.account.projectState.fetch(projectPDA);
    expect(project.status).to.deep.equal({ failing: {} });
  });
});
