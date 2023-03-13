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
  let treasury = new anchor.web3.PublicKey(
    "4reqRtKEqXggb7xAFobTnPpAXtoSi5zEBK8WLL5qyn1B"
  );

  before(async () => {
    [identityManager] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity-manager"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  it("should set identity price", async () => {
    await program.methods
      .setIdentityPrice({ price: new anchor.BN(0) })
      .accounts({
        admin: authority.publicKey,
        identityManager: identityManager,
      })
      .rpc();
  });
});
