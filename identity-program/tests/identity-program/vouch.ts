import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { IdentityProgram } from "../../target/types/identity_program";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("identity-program-vouch", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();

  const program = anchor.workspace.IdentityProgram as Program<IdentityProgram>;

  const authority = (program.provider as any).wallet
    .payer as anchor.web3.Keypair;

  let identityManager: anchor.web3.PublicKey;
  let identity: anchor.web3.PublicKey;
  let otherIdentity: anchor.web3.PublicKey;
  let treasury = new anchor.web3.PublicKey("4reqRtKEqXggb7xAFobTnPpAXtoSi5zEBK8WLL5qyn1B");

  const otherKeypair = Keypair.generate();
  before(async () => {
    const airdrop = await provider.connection.requestAirdrop(
      otherKeypair.publicKey,
      LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(airdrop);

    [identityManager] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity-manager"), authority.publicKey.toBuffer()],
      program.programId
    );

    identity = new anchor.web3.PublicKey("3KmR3UtFpkUJWyi6WBARBRMpmcH5ZeBd8AZ4uBGcmrWT");
    // [identity] = await anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("identity"), authority.publicKey.toBuffer()],
    //   program.programId
    // );

    [otherIdentity] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity"), otherKeypair.publicKey.toBuffer()],
      program.programId
    );
  });

  it("should init other identity", async () => {
    let tx = await program.methods
      .initIdentity()
      .accounts({
        authority: otherKeypair.publicKey,
        identity: otherIdentity,
        identityOwner: otherKeypair.publicKey,
        identityManager: identityManager,
        treasury: treasury
      })
      .signers([otherKeypair])
      .rpc();
    console.log("Tx sig", tx);
  });

  it("should vouch for primary identity", async () => {
    const identityManagers = await program.account.identityManager.all();
    const before_identities = await program.account.identity.all();
    const before_primary_identity = before_identities.find((elem) =>
      elem.account.id.eq(new anchor.BN(1))
    );

    let tx = await program.methods
      .vouch()
      .accounts({
        authority: otherKeypair.publicKey,
        voucher: otherIdentity,
        vouchee: identity,
        identityManager: identityManager,
      })
      .signers([otherKeypair])
      .rpc();

    const after_identities = await program.account.identity.all();
    const after_primary_identity = after_identities.find((elem) =>
      elem.account.id.eq(new anchor.BN(1))
    );

    console.log(before_primary_identity);
    console.log(after_primary_identity);

    assert.isTrue(
      before_primary_identity.account.numOfVouches
        .add(new anchor.BN(1))
        .eq(after_primary_identity.account.numOfVouches)
    );

    assert.isTrue(
      before_primary_identity.account.ptsVouches
        .add(identityManagers[0].account.vouchPtsReward)
        .eq(after_primary_identity.account.ptsVouches)
    );
  });
});
