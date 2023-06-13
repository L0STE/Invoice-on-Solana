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
  const keypair = anchor.web3.Keypair.generate(); //This is the project Manager

  ///////////////////////////////////////////////////////////////////////////////////////////

  //INPUT
  const project = "WBA";
  let employee = anchor.web3.Keypair.generate(); //This will be fetched when clicking on the account available in the Project
  let from = new Date(Date.UTC(2024, 4, 27)); //Put your from from date here
  let to = new Date(Date.UTC(2024, 4, 30)); //Put your to from date here 
  let amount = new anchor.BN(30*LAMPORTS_PER_SOL); //Put your amount here
  let employeeTitle = "Developer"; //Put your employee title here
  let monthlypay = new anchor.BN(3*LAMPORTS_PER_SOL); //Put your employee monthly pay here


  ///////////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////////

//PROJECT

const projectState = anchor.web3.Keypair.generate();
// Deriving the PDAs
const projectSeeds = [Buffer.from("project"), projectState.publicKey.toBuffer()];
const [projectKey, _bump] = PublicKey.findProgramAddressSync(projectSeeds, program.programId);

it("EMPLOYER - Create the Project Account", async () => {
  try {
    const tx = await program.methods
    .createProject(
      project,
      _bump
    )
    .accounts({
      project: projectState.publicKey,
      projectVault: projectKey,
      owner: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      projectState,
      keypair
    ]).rpc();
    let projectAccount = await program.account.project.fetch(projectState.publicKey);
    console.log(`\nOwner: ${projectAccount.owner}`);
    console.log(`\nProject Name: ${projectAccount.projectName}`);
    console.log(`\nBalance: ${projectAccount.balance}`);
    console.log(`\nMonthly Spending: ${projectAccount.monthlySpending}`);
  } catch (err) {
    console.log(err);
  }
})

it("EMPLOYER - Change the Auth of the Project", async () => {
  const tx = await program.methods
  .projectChangeAuth(
    employee.publicKey,
  )
  .accounts({
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let projectAccount = await program.account.project.fetch(projectState.publicKey);
console.log(`The new Owner is: ${projectAccount.owner}`);
})

it("EMPLOYER - Change the Auth of the Project", async () => {
  const tx = await program.methods
  .projectChangeAuth(
    keypair.publicKey,
  )
  .accounts({
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: employee.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    employee
  ]).rpc();
let projectAccount = await program.account.project.fetch(projectState.publicKey);
console.log(`The new Owner is: ${projectAccount.owner}`);
})

it("EMPLOYER - Change the Name of the Project", async () => {
  let project = "WBAA"
  const tx = await program.methods
  .projectChangeName(
    project,
  )
  .accounts({
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let projectAccount = await program.account.project.fetch(projectState.publicKey);
console.log(`You created ${projectAccount.projectName}`);
})

it("EMPLOYER - Deposit sol in the Escrow", async () => {
  const tx = await program.methods
  .projectDeposit(
    amount,
  )
  .accounts({
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let accountBalance = await provider.connection.getBalance(projectKey);
console.log("\nNew Balance: ", accountBalance/LAMPORTS_PER_SOL);
})

it("EMPLOYER - Withdraw sol from the Escrow", async () => {
  amount = new anchor.BN(2*LAMPORTS_PER_SOL)
  try {
  const tx = await program.methods
  .projectWithdraw(
    amount,
  )
  .accounts({
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
  let accountBalance = await provider.connection.getBalance(projectKey);
  console.log("\nNew Balance: ", accountBalance/LAMPORTS_PER_SOL);
  }catch(e){
    console.log(e)
  }
})


///////////////////////////////////////////////////////////////////////////////////////////

//Employee

const employeeState = anchor.web3.Keypair.generate();
// Deriving the PDAs
const employeeSeeds = [Buffer.from("project"), employeeState.publicKey.toBuffer()];
const [employeeKey, employeeBump] = PublicKey.findProgramAddressSync(employeeSeeds, program.programId);

it("EMPLOYER - Create the Employee Account", async () => {
  const tx = await program.methods
  .createEmployee(
    employeeTitle,
    employee.publicKey,
    monthlypay,
    employeeBump
  )
  .accounts({
    employee: employeeState.publicKey,
    employeeVault: employeeKey,
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    employeeState,
    keypair
  ]).rpc();
let employeeAccount = await program.account.employee.fetch(employeeState.publicKey);
console.log(`\nProject: ${employeeAccount.project}`);
console.log(`\nEmployee: ${employeeAccount.employee}`);
console.log(`\nEmployee Title: ${employeeAccount.employeeTitle}`);
console.log(`\nDay Worked: ${employeeAccount.monthlyPay}`);
console.log(`\nMoney Earned: ${employeeAccount.isActive}`);
})

it("EMPLOYER - Change the Employee Wallet", async () => {

  employee = anchor.web3.Keypair.generate();

  const tx = await program.methods
  .employeeChangeWallet(
    keypair.publicKey,
  )
  .accounts({
    employee: employeeState.publicKey,
    project: projectState.publicKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let employeeAccount = await program.account.employee.fetch(employeeState.publicKey);
console.log(`\nEmployee: ${employeeAccount.employee}`);
})

it("EMPLOYER - Change the Employee Wallet", async () => {

  employee = anchor.web3.Keypair.generate();

  const tx = await program.methods
  .employeeChangeWallet(
    employee.publicKey,
  )
  .accounts({
    employee: employeeState.publicKey,
    project: projectState.publicKey,
    owner: employee.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let employeeAccount = await program.account.employee.fetch(employeeState.publicKey);
console.log(`\nEmployee: ${employeeAccount.employee}`);
})

it("EMPLOYER - Change the Employee Title", async () => {

  employeeTitle = "MARKETING"

  const tx = await program.methods
  .employeeChangeTitle(
    employeeTitle,
  )
  .accounts({
    employee: employeeState.publicKey,
    project: projectState.publicKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    keypair
  ]).rpc();
let employeeAccount = await program.account.employee.fetch(employeeState.publicKey);
console.log(`\nTitle: ${employeeAccount.employeeTitle}`);
})

/*
it("EMPLOYEE - Accept the job", async () => {

  const tx = await program.methods
  .employeeAccept()
  .accounts({
    employee: employeeState.publicKey,
    project: projectState.publicKey,
    owner: employee.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    employee
  ]).rpc();
let employeeAccount = await program.account.employee.fetch(employeeState.publicKey);
console.log(`\nHas Accepted: ${employeeAccount.hasAccepted}`);
})

///////////////////////////////////////////////////////////////////////////////////////////

//INVOICE

//ToDo - and to fix the instruction to be more precise

const invoiceState = anchor.web3.Keypair.generate();
// Deriving the PDAs
const invoiceSeeds = [Buffer.from("project"), invoiceState.publicKey.toBuffer()];
const [invoiceKey, _anotherBump] = PublicKey.findProgramAddressSync(invoiceSeeds, program.programId);

it("EMPLOYER - Create the Invoice", async () => {
  let fromUnixTimestamp = Math.floor(from.getTime() / 1000);
  let toUnixTimestamp = Math.floor(to.getTime() / 1000);

  const tx = await program.methods
  .createInvoice(
    amount,
    fromUnixTimestamp,
    toUnixTimestamp,
    _anotherBump,
  )
  .accounts({
    invoice: invoiceState.publicKey,
    invoiceVault: invoiceKey,
    employee: employeeState.publicKey,
    project: projectState.publicKey,
    projectVault: projectKey,
    owner: keypair.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([
    invoiceState,
    keypair
  ]).rpc();
  let invoiceAccount = await program.account.invoice.fetch(invoiceState.publicKey);
  console.log(`\nProject: ${invoiceAccount.project}`);
  console.log(`\nEmployee: ${invoiceAccount.employeeWallet}`);
  console.log(`\nFrom: ${invoiceAccount.from}`);
  console.log(`\nTo: ${invoiceAccount.to}`);
  console.log(`\nAmount: ${invoiceAccount.amount}`);
  console.log(`\nHas Claimed?: ${invoiceAccount.hasClaimed}`);
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

