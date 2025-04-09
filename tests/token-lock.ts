import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenLock } from "../target/types/token_lock";
import { assert } from "chai";

describe("token-lock", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenLock as Program<TokenLock>;
  const authority = provider.wallet;
  const recipient = anchor.web3.Keypair.generate();

  let vaultPda: anchor.web3.PublicKey;
  let bump: number;

  it("Initializes the vault", async () => {
    [vaultPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), authority.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initialize(
        recipient.publicKey,
        2, // cancel_permission = creator
        0, // change_recipient_permission = none
        new anchor.BN(5) // lock duration = 5 seconds
      )
      .accounts({
        vault: vaultPda,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);
    assert.strictEqual(vault.authority.toBase58(), authority.publicKey.toBase58());
    assert.strictEqual(vault.recipient.toBase58(), recipient.publicKey.toBase58());
  });
});
