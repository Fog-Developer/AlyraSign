import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AlyraSign } from "../target/types/alyra_sign";
import { SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("Testing alyra_sign", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.AlyraSign as Program<AlyraSign>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Create event testing ...", async () => {
    const eventCode = "EV01";
    const eventTitle = "Développeur SOLANA";
    const [eventPda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event"), Buffer.from(eventCode)],
      program.programId
    );

    const tx = await program.methods
      .createEvent(eventCode, eventTitle)
      .accountsPartial({      /// bien metrte AccountsPartial depuis la version 0.30 car elle tente de résoudre les noms automatiquement
        signer: provider.wallet.publicKey,
        event: eventPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const eventAccount = await program.account.event.fetch(eventPda);
    expect(eventAccount.eventCode).to.equal(eventCode);
    expect(eventAccount.title).to.equal(eventTitle);
    //expect(eventAccount.signer.toString()).to.equal(authority.publicKey.toString());
  });



  it("Create session testing ...", async () => {
    const eventCode = "EV01";
    const sessionId = "S1";
    const sessionTitle = "Smart contracts - part 1";
    const sessionStartAt = new anchor.BN(1500383340);
    const sessionEndAt = new anchor.BN(1500384340);
    const [eventPda, bump_event] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event"), Buffer.from(eventCode)],
      program.programId
    );

    // const eventAccount = await program.account.event.fetch(eventPda);
    // const sessionsCount = (eventAccount.sessionsCount).add(new anchor.BN(1));
    // console.log(`sessionsCount: ${sessionsCount.toString()}`);

    const [sessionPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("session"), eventPda.toBuffer(), Buffer.from(sessionId)],
      program.programId
    );

    const tx = await program.methods
      .createSession(sessionId, sessionTitle, sessionStartAt, sessionEndAt)
      .accountsPartial({      /// bien metrte AccountsPartial depuis la version 0.30 car elle tente de résoudre les noms automatiquement
        session: sessionPda,
        event: eventPda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const sessionAccount = await program.account.session.fetch(sessionPda);
    expect(sessionAccount.sessionId).to.equal(sessionId);
    expect(sessionAccount.title).to.equal(sessionTitle);
    //expect(eventAccount.signer.toString()).to.equal(authority.publicKey.toString());
  }); 


  
});
