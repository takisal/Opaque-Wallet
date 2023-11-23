use log::*;
use std::collections::HashMap;
use std::io::Write;

pub mod rpc_methods;
fn main() {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let new_url: String = "http://__cookie__:ff8b581b7c300397ed956feb515dee30d255005502e983fcb169ede8b65c79b1@127.0.0.1:8332".to_string();
    let mut map = HashMap::new();
    map.insert("jsonrpc", "1.0");
    map.insert("id", "curltest");
    match rt.block_on(rpc_methods::blockchain_rpcs::get_block_count(
        new_url.clone(),
        map.clone(),
    )) {
        0 => println!("Error"),
        x @ _ => println!("Block Number: {}", x),
    }
    /*
        match rt.block_on(rpc_methods::wallet_rpcs::abandon_transaction(
            new_url.clone(),
            "1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d".to_string(),
            "rusttestie".to_string(),
        )) {
            _ => println!("Called"),
        }
        //When wallet is created, it is saved in this format: rusttestie/wallet.dat

        match rt.block_on(rpc_methods::wallet_rpcs::load_wallet(
            new_url.clone(),
            "rusttestie".to_string(),
            false,
        )) {
            x @ _ => println!("Loaded: {}", x),
        }
          match rt.block_on(rpc_methods::wallet_rpcs::abandon_transaction(
              new_url,
              "1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d".to_string(),
          )) {
              _ => println!("Called"),
          }

    match rt.block_on(rpc_methods::wallet_rpcs::create_wallet(
        new_url,
        "rusttestie".to_string(),
        false,
        false,
        "".to_string(),
        false,
        false,
        false,
    )) {
        x @ _ => println!("New wallet name: {}", x),
    } */
}
