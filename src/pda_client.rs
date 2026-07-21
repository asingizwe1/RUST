
let program_id = pubkey!("7ZP42kRwUQ2zgbqXoaXzAFaiQnDyp6swNktTSv8mNQGN");
let from_keypair = Keypair::new();
let (pda_address, bump) =
    Pubkey::find_program_address(&[&from_keypair.pubkey().to_bytes()], &program_id);
let data_size = 0;

let ix_data = vec![data_size, bump];
let accounts = vec![
    AccountMeta::new(from_keypair.pubkey(), true),
    AccountMeta::new(pda_address, false),
    AccountMeta::new(SYSVAR_RENT_ID, false),
    AccountMeta::new(SYSTEM_PROGRAM_ID, false),
];

let create_pda_ix = Instruction::new_with_bytes(program_id, &ix_data, accounts);


let mut transaction =
    Transaction::new_with_payer(&[create_pda_ix], Some(&from_keypair.pubkey()));
transaction.sign(&[&from_keypair], client.get_latest_blockhash().await?);

match client.send_and_confirm_transaction(&transaction).await {
    Ok(signature) => println!("Transaction Signature: {}", signature),
    Err(err) => eprintln!("Error sending transaction: {}", err),
}

Ok(())
}