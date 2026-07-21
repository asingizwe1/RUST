use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
/*
Keeping accounts alive on Solana incurs a data storage cost called rent.
Rent is charged in lamports, the native currency of Solana. The amount of rent charged depends on the size of the account and the current rent rate. If an account's balance falls below a certain threshold, it becomes "rent-exempt" and is not charged rent.
For the calculation, you need to consider the amount of data you intend to store in the account
Rent can be reclaimed in full if the account is closed.
*/
//rent is a mechanism designed to prevent the blockchain's storage from becoming bloated with unused accounts.

/*
1. Budgeting for Account Creation
Before creating a new account, you need to know how much SOL to send to that account so:

It’s rent-exempt ✅

It won’t get deleted due to low balance ❌

Example: You’re building a DApp that creates token accounts for users. If you underfund them, they could vanish. Not good!

🧮 2. Preventing Errors in Transactions
If you try to create an account with insufficient lamports, the transaction will fail, wasting time and network fees.

Calculating the required SOL ahead of time ensures your transaction works smoothly.

🛠️ 3. Dynamic Account Sizes
Some accounts (e.g. for NFTs, PDAs, or custom programs) have variable sizes, and rent depends on that size.

So, you need to calculate:

text
Copy code
rent_exemption_cost = rent_rate * account_data_size
And fund accordingly.

🧪 4. Testing and Simulations
When building programs or running tests, you often need to:

Create temporary accounts

Allocate exact amounts of SOL

Avoid overpaying or underpaying

A calculator helps automate that.

🔐 5. Wallets and Frontends
Wallets and DApps often use rent calculations to:

Show users how much SOL will be used when they approve a transaction

Pre-fill the required amount

Offer a better user experience


*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:8899"),
        CommitmentConfig::confirmed(),
    );

    let data_len = 1500;
    let rent_exemption_amount = client
        .get_minimum_balance_for_rent_exemption(data_len)
        .await?;

    println!("{rent_exemption_amount}");

    Ok(())
}