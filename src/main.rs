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

    match rt.block_on(rpc_methods::blockchain_rpcs::get_block_count(
        new_url.clone(),
    )) {
        0 => println!("Error"),
        x @ _ => println!("Block Number: {}", x),
    }
    match rt.block_on(rpc_methods::wallet_rpcs::abort_rescan(
        new_url.clone(),
        "rusttestie".to_owned(),
    )) {
        x @ _ => println!("Aborted rescan: {}", x),
    }
}
