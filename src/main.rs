use log::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io::Write;

fn main() {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    info!("Hi");
    println!("Done");
    let new_url: String = "http://__cookie__:ff8b581b7c300397ed956feb515dee30d255005502e983fcb169ede8b65c79b1@127.0.0.1:8332".to_string();

    match rt.block_on(query_block_number(new_url)) {
        0 => println!("YOOO"),
        x @ _ => println!("No match: {}", x),
    }
}

async fn query_block_number(url: String) -> i32 {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "1.0");
    map.insert("id", "curltest");
    map.insert("method", "getblockcount");
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).json(&map).send().await.unwrap();
    println!("Starting program!");
    let pepe = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&pepe).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    println!("{}", obj["result"]);
    let sman = i32::try_from(obj["result"].as_i64().unwrap());
    return sman.ok().unwrap();
}
