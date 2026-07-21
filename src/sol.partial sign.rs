/*
Partial Sign Transaction
When a transaction requires multiple signatures, you can partially sign it. The other signers can then sign and broadcast it on the network.

Some examples of when this is useful:

Send an SPL token in return for payment
Sign a transaction so that you can later verify its authenticity
Call custom programs in a transaction that require your signature
*/
/*
 1. Multisig Wallets
In multisignature wallets (e.g., a 3-of-5 multisig), you need multiple owners to approve a transaction. Partial signing allows:

One owner to create the transaction and sign it.

Others to receive the partially signed transaction and add their signatures.

Once enough signatures are collected, the transaction can be submitted.

✅ Use Case: DAO treasury movement, team fund control.

🤝 2. Delegated Authorization
A transaction might require:

A delegate (e.g., a custodian, broker, or front-end) to prepare the transaction.

The actual authority (e.g., wallet owner) to co-sign before it's valid.

✅ Use Case: UX-friendly dApps where backend prepares transactions and sends to user for signing.

📄 3. Off-Chain Transaction Coordination
Transactions can be:

Composed and partially signed by an application/server.

Passed to a user to finish signing in a secure environment (like a hardware wallet or wallet app).

✅ Use Case: Secure dApp designs where the frontend/backend helps with transaction assembly but final approval is user-controlled.

🛡️ 4. Custodial Services
In scenarios like:

Crypto exchanges or custodians managing user funds.

Compliance or manual checks before broadcasting.

Partial signing lets the exchange prepare the transaction, and an internal approval system completes the signing later.

✅ Use Case: Institutional custody or hot/cold wallet management.

🔄 5. Atomic Multi-Party Operations
In certain contracts:

Multiple parties must perform actions atomically (e.g., escrow, swaps).

Each one signs their part, and once all agree, the transaction becomes valid.

✅ Use Case: OTC trades, Solana-based DeFi protocols requiring coordinated updates.

📬 6. Offline or Air-gapped Signing
When one signer is on a cold wallet or offline device, the transaction can be:

Composed and partially signed online.

Transferred via QR code/USB to the offline device.

Signed offline, then brought back to broadcast.

✅ Use Case: High-security setups.

🧠 BONUS: Dev Tools & Libraries
You’ll commonly use:

Transaction.partialSign() in JavaScript.

signers vector in Rust.

Staging/signature collection via tools like solana/web3.js, Anchor, or xNFTs.
*/

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
    system_program,
};
use spl_token::{
    instruction::transfer_checked,
    state::Mint,
};
use spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account};
use std::str::FromStr;
use anyhow::Result;
use bs58;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Parse Alice's public key
    let alice_pubkey = Pubkey::from_str("5YNmS1R9nNSCDzb5a7mMJ1dwK9uHeAAF4CmPEwKgVWr8")?;

    // Decode Bob's keypair from base58 string
    let bob_keypair_bytes = bs58::decode("4NMwxzmYj2uvHuq8xoqhY8RXg63KSVJM1DXkpbmkUY7YQWuoyQgFnnzn6yo3CMnqZasnNPNuAT2TLwQsCaKkUddp").into_vec()?;
    let bob_keypair = Keypair::from_bytes(&bob_keypair_bytes)?;

    // Token mint address (USDC devnet)
    let token_mint = Pubkey::from_str("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")?;

    // Derive token account addresses
    let bob_token_account = get_associated_token_address(&bob_keypair.pubkey(), &token_mint);
    let alice_token_account = get_associated_token_address(&alice_pubkey, &token_mint);

    // Create associated token account for Alice if it doesn't exist
    let alice_token_info = connection.get_account(&alice_token_account).await;
    let mut instructions = vec![];

    if alice_token_info.is_err() {
        instructions.push(create_associated_token_account(
            &bob_keypair.pubkey(),
            &alice_pubkey,
            &token_mint,
        ));
    }

    // Get token mint info
    let mint_account = connection.get_account(&token_mint).await?;
    let mint_data = Mint::unpack(&mint_account.data)?;
    let decimals = mint_data.decimals;

    // Transfer 0.01 SOL from Alice to Bob
    instructions.push(system_instruction::transfer(
        &alice_pubkey,
        &bob_keypair.pubkey(),
        (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64,
    ));

    // Transfer 1 token (10^decimals units) from Bob to Alice
    instructions.push(transfer_checked(
        &spl_token::id(),
        &bob_token_account,
        &token_mint,
        &alice_token_account,
        &bob_keypair.pubkey(),
        &[],
        1 * 10u64.pow(decimals as u32),
        decimals,
    )?);

    // Get a recent blockhash
    let recent_blockhash = connection.get_latest_blockhash().await?;

    // Construct transaction
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&alice_pubkey));
    transaction.partial_sign(&[&bob_keypair], recent_blockhash);

    // Serialize to base64 for Alice to sign and send
    let serialized = transaction.serialize();
    let base64 = base64::encode(serialized);

    println!("Base64 Transaction for Alice to sign:\n{}", base64);

    // (Optional) Alice can later recover it with:
    // let tx = Transaction::from(Vec::<u8>::from_base64(base64_str)?);

    Ok(())
}
