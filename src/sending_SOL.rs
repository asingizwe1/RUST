use solana_client::nonblocking::rpc_client::RpcClient;
//This brings in the non-blocking (async) RPC client from the Solana library.
//This lets you interact with the Solana blockchain
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,// LAMPORTS_PER_SOL: Constant value of how many lamports (smallest unit) make 1 SOL (like cents in a dollar).
    //CommitmentConfig: Used to tell Solana how "final" a transaction should be before it's accepted.
    //Keypair: Represents a wallet (a public/private key pair).
    signer::Signer, system_instruction::transfer, transaction::Transaction,
    //Signer: Trait that allows us to "sign" transactions with our wallet.
    //transfer: A helper function to create a transfer instruction.
    //Transaction: Represents a set of instructions sent to the blockchain.
};

#[tokio::main]//macro sets up an asynchronous runtime.
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:8899"),
        CommitmentConfig::confirmed(),
    );//connects to your local Solana node running at 127.0.0.1:8899
//You're saying: "I want to interact with Solana and wait until transactions are confirmed before I consider them successful."
//Creating Two Wallets
let from_keypair = Keypair::new();//The wallet that sends 1 SOL.
    let to_keypair = Keypair::new();
//“Create an instruction to send 1 SOL (aka 1,000,000,000 lamports) from from_keypair to to_keypair.”
    let transfer_ix = transfer(
        &from_keypair.pubkey(),
        &to_keypair.pubkey(),
        LAMPORTS_PER_SOL,
    );


/*
This wraps the instruction into a Transaction and says:
“This transaction should be paid for by from_keypair.”
*/
    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&from_keypair.pubkey()));
    transaction.sign(&[&from_keypair], client.get_latest_blockhash().await?);
//Sign the transaction with the private key of the sender.
    
    /*
This sends the transaction to the blockchain. If successful:
You get the transaction signature (like a receipt). If it fails:
It prints an error.
     */
    match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    Ok(())
}