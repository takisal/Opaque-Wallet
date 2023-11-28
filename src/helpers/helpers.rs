use crate::rpc_methods;
use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub(crate) address: String,
    pub(crate) amount: f64,
    pub(crate) blockheight: u64,
    pub(crate) blockindex: u64,
    pub(crate) blocktime: u64,
    pub(crate) category: String,
    pub(crate) confirmations: u64,
    pub(crate) label: String,
    pub(crate) time: u64,
    pub(crate) timereceived: u64,
    pub(crate) txid: String,
    pub(crate) vout: u64,
    pub(crate) walletconflicts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingTransaction {
    pub(crate) address: String,
    pub(crate) amount: f64,
    pub(crate) category: String,
    confirmations: u64,
    pub(crate) time: u64,
    pub(crate) timereceived: u64,
    pub(crate) txid: String,
    pub(crate) vout: u64,
    pub(crate) walletconflicts: Vec<String>,
}
pub fn initialize_wallet(
    url: String,
    wallet_name: String,
    label: String,
    count: u32,
    include_watchonly: bool,
    all_transactions: &mut Vec<Transaction>,
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
                all_transactions.push(tx);
            }
        }
    }
}
