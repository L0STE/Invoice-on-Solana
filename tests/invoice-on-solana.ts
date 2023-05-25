import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InvoiceOnSolana } from "../target/types/invoice_on_solana";
import { Connection, clusterApiUrl, ConfirmOptions, PublicKey, SystemProgram, LAMPORTS_PER_SOL} from "@solana/web3.js";

describe("invoice-on-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  anchor.AnchorProvider.env().opts.commitment = "finalized";

  const program = anchor.workspace.InvoiceOnSolana as Program<InvoiceOnSolana>;

  const connection = new Connection("https://api.devnet.solana.com");

  // Generate a new keypair for the the Project PDA
  const projectKeypair = anchor.web3.Keypair.generate();
  console.log("\nProject Public Key: ", projectKeypair.publicKey.toBase58());
  // Create the PDA
  const projectSeeds = [Buffer.from("project"), projectKeypair.publicKey.toBuffer()];
  const [projectKey, _bump] = PublicKey.findProgramAddressSync(projectSeeds, program.programId);

  // Generate a new keypair for the the Employee PDA
  //const employeePda = anchor.web3.Keypair.generate();
  //console.log("employeePda Public Key: ", employeePda.publicKey.toBase58());

  // Generate a new keypair for the the Invoice PDA
  //const invoicePda = anchor.web3.Keypair.generate();
  //console.log("invoicePda Public Key: ", invoicePda.publicKey.toBase58());

  ///////////////////////////////////////////////////////////////////////////////////////////

  const keypair = anchor.web3.Keypair.generate(); //This is the project Manager

  //INPUT FROM THE USER
  const project = Buffer.from("WBA", "utf8");
  let employee = anchor.web3.Keypair.generate(); //This will be fetched when clicking on the account available in the Project
  

  //Input from the React Part
  let from = new Date(Date.UTC(2024, 4, 27)); //Put your from from date here
  let to = new Date(Date.UTC(2024, 4, 30)); //Put your to from date here 
  let amount = 3*LAMPORTS_PER_SOL; //Put your amount here


  //Funding the wallet
  it("Starts an airdrop and confirms it", async () => {
    const signature = await provider.connection.requestAirdrop(keypair.publicKey, 100 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized"
    );  
  })

  /*
  it("Starts an airdrop and confirms it", async () => {
    const signature = await provider.connection.requestAirdrop(employee.publicKey, 100 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized"
    );  
  })

  it ("Check Balance", async () => {
    let accountBalance = await provider.connection.getBalance(keypair.publicKey);
    let employeeBalance = await provider.connection.getBalance(employee.publicKey);
    console.log("\nMain Wallet Balance: ", accountBalance/LAMPORTS_PER_SOL);
    console.log("\nEmployee Wallet Balance: ", employeeBalance/LAMPORTS_PER_SOL);
  })
  */

///////////////////////////////////////////////////////////////////////////////////////////

//PROJECT

it("EMPLOYER - Create the Project", async () => {
    try {
    const tx = await program.methods
    .createProject(
      project
    )
    .accounts({
      projectPda: projectKeypair.publicKey,
      pdaAuth: projectKey,
      owner: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  let projectAccount = await program.account.project.fetch(projectKey);
  console.log(`You created ${projectAccount.owner}`);
  } catch (e) {
    console.log(e)
  }
})

///////////////////////////////////////////////////////////////////////////////////////////

//INVOICE

/*
  it("EMPLOYER - Create the Invoice", async () => {
    let fromUnixTimestamp = Math.floor(from.getTime() / 1000);
    let toUnixTimestamp = Math.floor(to.getTime() / 1000);

    try {
    const tx = await program.methods
    .createInvoice(
      employee.publicKey,
      new anchor.BN(fromUnixTimestamp),
      new anchor.BN(toUnixTimestamp),
      new anchor.BN(amount),
    )
    .accounts({
      employer: keypair.publicKey,
      invoiceState: invoiceState.publicKey,
      invoice: invoice_key,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  console.log("\nSuccess! You Sent 3 Sol to Escrow\n");
  let accountBalance = await provider.connection.getBalance(invoice_key);
  console.log("\nNEW Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
  } catch (e) {
    console.log(e)
  }
  })

/*  it("EMPLOYER - Change Amount for Invoice", async () => {
    let amount = 6*LAMPORTS_PER_SOL
    let time = new Date();
    let timeUnixTimestamp = Math.floor(time.getTime() / 1000);

    try {
    const tx = await program.methods
    .changeInvoiceAmount(
      new anchor.BN(amount),
      new anchor.BN(timeUnixTimestamp),
    )
    .accounts({
      employer: keypair.publicKey,
      invoiceState: invoiceState.publicKey,
      invoice: invoice_key,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  console.log("\nSuccess! Changed the Amount\n");
  let accountBalance = await provider.connection.getBalance(invoice_key);
  console.log("\nNEW Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
  } catch (e) {
    console.log(e)
  }
  })

  it("EMPLOYER - Change Date for Invoice", async () => {
    const from = new Date(Date.UTC(2024, 4, 27)); 
    const to = new Date(Date.UTC(2024, 4, 30));
    let fromUnixTimestamp = Math.floor(from.getTime() / 1000);
    let toUnixTimestamp = Math.floor(to.getTime() / 1000);
    let time = new Date();
    let timeUnixTimestamp = Math.floor(time.getTime() / 1000);

    try {
    const tx = await program.methods
    .changeInvoiceDate(
      new anchor.BN(fromUnixTimestamp),
      new anchor.BN(toUnixTimestamp),
      new anchor.BN(timeUnixTimestamp),
    )
    .accounts({
      employer: keypair.publicKey,
      invoiceState: invoiceState.publicKey,
      invoice: invoice_key,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  console.log("\nSuccess! Changed the Date\n");
  let accountBalance = await provider.connection.getBalance(invoice_key);
  console.log("\nNEW Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
  } catch (e) {
    console.log(e)
  }
  })

  it("EMPLOYER - Change Employee for Invoice", async () => {
    let employee = anchor.web3.Keypair.generate();
    let time = new Date();
    let timeUnixTimestamp = Math.floor(time.getTime() / 1000);

    try {
    const tx = await program.methods
    .changeInvoiceEmployee(
      employee.publicKey,
      new anchor.BN(timeUnixTimestamp),
    )
    .accounts({
      employer: keypair.publicKey,
      invoiceState: invoiceState.publicKey,
      invoice: invoice_key,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  console.log("\nSuccess! Changed the Date\n");
  let accountBalance = await provider.connection.getBalance(invoice_key);
  console.log("\nNEW Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
  } catch (e) {
    console.log(e)
  }
  })

  it("EMPLOYEE - Claim your SOL", async () => {
    let time = new Date();
    let timeUnixTimestamp = Math.floor(time.getTime() / 1000);

    try {
    const tx = await program.methods
    .claimInvoice(
      new anchor.BN(timeUnixTimestamp)
    )
    .accounts({
      employee: employee.publicKey,
      invoiceState: invoiceState.publicKey,
      invoice: invoice_key,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      keypair
    ]).rpc();
  console.log("\nSuccess! You Sent 3 Sol to Escrow\n");
  let accountBalance = await provider.connection.getBalance(invoice_key);
  console.log("\nNEW Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
  } catch (e) {
    console.log(e)
  }
  })
*/









});

