import * as anchor from "@coral-xyz/anchor";

describe("user_profile", () =>{
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.UserProfile;
  const user = provider.wallet.publicKey;

  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), user.toBuffer()],
    program.programId
  );

  it("initialize profile", async () => {
    await program.methods
      .initializeProfile("Adarsh", "Solana Auditor")
      .accounts({
        profile: profilePda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

      const acc = await program.account.profile.fetch(profilePda);
      console.log(acc);
  });

  it("updates profile", async () => {
    await program.methods
      .updateProfile("romy", "now leveling up")
      .accounts({
        profile: profilePda,
        user: user,
      })
      .rpc();

      const acc = await program.account.profile.fetch(profilePda);
      console.log(acc);
  });
})