import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { IdentityProgram } from "../../target/types/identity_program";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("identity-manager", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.getProvider();

  const program = anchor.workspace.IdentityProgram as Program<IdentityProgram>;

  const authority = (program.provider as any).wallet
    .payer as anchor.web3.Keypair;

  let identityManager: anchor.web3.PublicKey;
  let otherIdentity: anchor.web3.PublicKey;
  let treasury = new anchor.web3.PublicKey(
    "4reqRtKEqXggb7xAFobTnPpAXtoSi5zEBK8WLL5qyn1B"
  );

  const otherKeypair = Keypair.generate();

  before(async () => {
    const airdrop = await provider.connection.requestAirdrop(
      otherKeypair.publicKey,
      LAMPORTS_PER_SOL
    );

    [identityManager] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity-manager"), authority.publicKey.toBuffer()],
      program.programId
    );

    [otherIdentity] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity"), otherKeypair.publicKey.toBuffer()],
      program.programId
    );
  });

  it("should set allowInitIdentity to false", async () => {
    console.log(authority.publicKey.toBase58());
    await program.methods
      .setAllowInitIdentity({ allowInitIdentity: false })
      .accounts({
        admin: authority.publicKey,
        identityManager: identityManager,
      })
      .rpc();

    let identityManagerWithFalse = await program.account.identityManager.fetch(
      identityManager
    );

    try {
      let tx = await program.methods
        .initIdentity()
        .accounts({
          authority: otherKeypair.publicKey,
          identity: otherIdentity,
          identityOwner: otherKeypair.publicKey,
          identityManager: identityManager,
          treasury: treasury,
        })
        .signers([otherKeypair])
        .rpc();
      // Fail, this call is supposed to fail!
      assert.isTrue(false);
    } catch (err) {
      // Expected to fail
      if (!err.toString().includes("InitIdentityDisabled.")) {
        // err message must include the above message. Otherwise fail!
        assert.isTrue(false);
      }
    }

    await program.methods
      .setAllowInitIdentity({ allowInitIdentity: true })
      .accounts({
        admin: authority.publicKey,
        identityManager: identityManager,
      })
      .rpc();
    let identityManagerWithTrue = await program.account.identityManager.fetch(
      identityManager
    );
    assert.isTrue(!identityManagerWithFalse.allowInitIdentity);
    assert.isTrue(identityManagerWithTrue.allowInitIdentity);

    let tx = await program.methods
      .initIdentity()
      .accounts({
        authority: otherKeypair.publicKey,
        identity: otherIdentity,
        identityOwner: otherKeypair.publicKey,
        identityManager: identityManager,
        treasury: treasury,
      })
      .signers([otherKeypair])
      .rpc();
  });
});
