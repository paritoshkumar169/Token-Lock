import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenLock } from "../target/types/token_lock";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("token-lock", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TokenLock as Program<TokenLock>;

  const authority = provider.wallet;
  let vaultPda: PublicKey;

  it("Initializes vault", async () => {
    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), authority.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.initialize().accounts({
      vault: vaultPda,
      authority: authority.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("✅ Vault initialized at:", vaultPda.toBase58());
    console.log("🔗 Tx:", tx);
  });

  it("Deposits 0.5 SOL into vault", async () => {
    const depositAmount = 0.5 * LAMPORTS_PER_SOL;

    const tx = await program.methods.deposit(new anchor.BN(depositAmount)).accounts({
      vault: vaultPda,
      user: authority.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("✅ Deposited 0.5 SOL into vault");
    console.log("🔗 Tx:", tx);
  });

  it("Withdraws 0.2 SOL from vault", async () => {
    const withdrawAmount = 0.2 * LAMPORTS_PER_SOL;

    const tx = await program.methods.withdraw(new anchor.BN(withdrawAmount)).accounts({
      vault: vaultPda,
      authority: authority.publicKey,
      recipient: authority.publicKey,
    }).rpc();

    console.log("✅ Withdrew 0.2 SOL from vault to authority");
    console.log("🔗 Tx:", tx);
  });
});
