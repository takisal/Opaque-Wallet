use crate::rpc_methods;
use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    address: String,
    amount: f64,
    blockhash: String,
    blockheight: u64,
    blockindex: u64,
    blocktime: u64,
    category: String,
    confirmations: u64,
    label: String,
    time: u64,
    timereceived: u64,
    txid: String,
    vout: u64,
    walletconflicts: Vec<String>,
}
pub fn initialize_wallet(
    url: String,
    wallet_name: String,
    label: String,
    count: u32,
    include_watchonly: bool,
    sends: &mut Vec<Transaction>,
    receives: &mut Vec<Transaction>,
) {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(rpc_methods::wallet_rpcs::list_transactions(
        url.clone(),
        wallet_name.to_string(),
        label,
        count,
        include_watchonly,
    )) {
        x => {
            let k = serde_json::to_string(&x);
            println!("Loaded {}", k.unwrap());
            for tx in x {
                println!("categyor: {}", tx.category);
                if tx.category == "send" {
                    println!("category Send: {}", tx.category);
                    sends.push(tx);
                } else if tx.category == "receive" {
                    println!("category Receive: {}", tx.category);
                    receives.push(tx);
                }
            }
        }
    }
}
