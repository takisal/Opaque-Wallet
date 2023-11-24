use serde_json::{Map, Value};
use std::collections::HashMap;
pub async fn get_block_count(url: String) -> i32 {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "1.0");
    map.insert("id", "curltest");
    map.insert("method", "getblockcount");
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).json(&map).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let sman = i32::try_from(obj["result"].as_i64().unwrap());
    map.remove(&"method");
    return sman.ok().unwrap();
}
pub async fn get_best_block_hash(url: String) -> String {
    let mut map = HashMap::new();
    map.insert("jsonrpc", "1.0");
    map.insert("id", "curltest");
    map.insert("method", "getbestblockhash");
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).json(&map).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let sman = obj["result"].as_str().unwrap();
    map.remove(&"method");
    let dd = sman.to_string();
    return dd;
}
pub async fn get_block(url: String, block_hash: String) -> Map<String, Value> {
    let newstr = (r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getblock", "params": [""#
        .to_owned()
        + &block_hash
        + r#""]}"#)
        .to_string();

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    return obj;
}
