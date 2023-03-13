import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { assert } from "chai";
import { IdentityProgram } from "../../target/types/identity_program";

describe("identity-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.IdentityProgram as Program<IdentityProgram>;

  const authority = (program.provider as any).wallet
    .payer as anchor.web3.Keypair;

  let identityManager: anchor.web3.PublicKey;
  let identity: anchor.web3.PublicKey;
  let treasury = new anchor.web3.PublicKey("4reqRtKEqXggb7xAFobTnPpAXtoSi5zEBK8WLL5qyn1B");
  before(async () => {
    [identityManager] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity-manager"), authority.publicKey.toBuffer()],
      program.programId
    );

    [identity] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  // it("should init identity manager", async () => {
  //   let tx = await program.methods
  //     .initIdentityManager()
  //     .accounts({
  //       admin: authority.publicKey,
  //       identityManager: identityManager,
  //     })
  //     .rpc();
  //   console.log("Your transaction signature", tx);

  //   const identityManagers = await program.account.identityManager.all();
  //   assert.lengthOf(identityManagers, 1);
  // });

  it("should init identity account", async () => {
    let tx = await program.methods
      .initIdentity()
      .accounts({
        authority: authority.publicKey,
        identity: identity,
        identityOwner: authority.publicKey,
        identityManager: identityManager,
        treasury: treasury
      })
      .rpc();
    console.log("Tx sig", tx);
    const identities = await program.account.identity.all();
    assert.lengthOf(identities, 1);
    console.log(identities[0]);
  });
});
