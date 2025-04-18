import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AlyraSign } from "../target/types/alyra_sign";
import { SystemProgram } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
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


  it("Create Global Regitry testing ...", async () => {

    const [registryPda, _bumpRegistry] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("alyra_sign"), provider.wallet.publicKey.toBuffer()],
      program.programId
    ); 

    const tx = await program.methods
      .createRegistry()
      .accountsPartial({      /// bien mettre AccountsPartial depuis la version 0.30 car elle tente de résoudre les noms automatiquement
        registry: registryPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();


    const registryAccount = await program.account.registry.fetch(registryPda);

    expect(registryAccount.authority.toString()).to.equal(provider.wallet.publicKey.toString());
    expect(registryAccount.eventsCount.toString()).to.equal("0");

  });




  it("Create event testing ...", async () => {
    const eventId = new anchor.BN(1);
    const eventCode = "EV01";
    const eventTitle = "Développeur SOLANA";

    const [registryPda, _bumpRegistry] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("alyra_sign"), provider.wallet.publicKey.toBuffer()],
      program.programId
    ); 


    const registryAccount = await program.account.registry.fetch(registryPda);

    const [eventPda, _bumpEvent] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event"), eventId.toBuffer("le", 8)],
      program.programId
    );

    const tx = await program.methods
      .createEvent(eventId, eventCode, eventTitle)
      .accountsPartial({      /// bien mettre AccountsPartial depuis la version 0.30 car elle tente de résoudre les noms automatiquement
        authority: provider.wallet.publicKey,
        event: eventPda,
        registry: registryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const newRegistryAccount = await program.account.registry.fetch(registryPda);

    const eventAccount = await program.account.event.fetch(eventPda);
    expect(eventAccount.eventId.toString()).to.equal(eventId.toString());
    expect(eventAccount.eventCode).to.equal(eventCode);
    expect(eventAccount.title).to.equal(eventTitle);
    expect(eventAccount.authority.toString()).to.equal(registryAccount.authority.toString());
    expect((registryAccount.eventsCount.add(new anchor.BN(1))).toString()).to.equal(newRegistryAccount.eventsCount.toString());
  });



  it("Create session testing ...", async () => {
    const eventId = new anchor.BN(1);
    const sessionId = new anchor.BN(1);
    const sessionTitle = "Smart contracts - part 1";

    const startAtDate = new Date("2025-01-14T18:00:00Z");
    const sessionStartAt = new anchor.BN(Math.floor(startAtDate.getTime() / 1000));

    const endAtDate = new Date("2025-01-14T20:00:00Z");
    const sessionEndAt = new anchor.BN(Math.floor(endAtDate.getTime() / 1000));

    const [eventPda, bump_event] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event"), eventId.toBuffer("le", 8)],
      program.programId
    );

    const eventAccount = await program.account.event.fetch(eventPda);

    const [sessionPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("session"), eventPda.toBuffer(), sessionId.toBuffer("le", 8)],
      program.programId
    );

    const tx = await program.methods
      .createSession(sessionId, sessionTitle, sessionStartAt, sessionEndAt)
      .accountsPartial({      /// bien mettre AccountsPartial depuis la version 0.30 car elle tente de résoudre les noms automatiquement
        session: sessionPda,
        event: eventPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const newEventAccount = await program.account.event.fetch(eventPda);

    const sessionAccount = await program.account.session.fetch(sessionPda);
    expect(sessionAccount.sessionId.toString()).to.equal(sessionId.toString());
    expect(sessionAccount.title).to.equal(sessionTitle);
    expect(sessionAccount.authority.toString()).to.equal(eventAccount.authority.toString());
    expect((eventAccount.sessionsCount.add(new anchor.BN(1))).toString()).to.equal(newEventAccount.sessionsCount.toString());
  }); 


  
});
