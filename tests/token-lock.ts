import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenLock } from "../target/types/token_lock";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("token-lock", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TokenLock as Program<TokenLock>;

  const authority = provider.wallet;
  let vaultPda: PublicKey;

  it("Initializes vault", async () => {
    [vaultPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), authority.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.initialize().accounts({
      vault: vaultPda,
      authority: authority.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("Vault created at:", vaultPda.toBase58());
    console.log("Transaction:", tx);
  });
});
