use serde_json::{Map, Value};
pub async fn create_wallet(
    url: String,
    wallet_name: String,
    disable_private_keys: bool,
    blank: bool,
    passphrase: String,
    avoid_reuse: bool,
    descriptors: bool,
    load_on_startup: bool,
) -> String {
    let newstr = (r#"{"jsonrpc": "1.0", "id": "curltest", "method": "createwallet", "params": [""#
        .to_owned()
        + &wallet_name
        + r#"", "#
        + &disable_private_keys.to_string()
        + ", "
        + &blank.to_string())
        .to_string()
        + r#", ""#
        + &(&passphrase).to_string()
        + r#"""#
        + ", "
        + &avoid_reuse.to_string()
        + ", "
        + &descriptors.to_string()
        + ", "
        + &load_on_startup.to_string()
        + r#"]}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let pepe = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&pepe).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    return obj["result"]["name"].to_string();
}
pub async fn abandon_transaction(mut url: String, txid: String, walletname: String) {
    url = url + "/wallet/" + &walletname;
    let newstr =
        (r#"{"jsonrpc": "1.0", "id": "curltest", "method": "abandontransaction", "params": [""#
            .to_owned()
            + &txid
            + r#""]}"#)
            .to_string();

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
}
pub async fn load_wallet(url: String, filename: String, load_on_startup: bool) -> String {
    //let nonLoad = r#"", "#.to_owned() + &load_on_startup.to_string() + r#"]}"#;
    let newstr = (r#"{"jsonrpc": "1.0", "id": "curltest", "method": "loadwallet", "params": [""#
        .to_owned()
        + &filename
        + r#"", "#
        + &load_on_startup.to_string()
        + r#"]}"#)
        .to_string();

    println!("Newstr: {}", newstr);
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let pepe = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&pepe).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    return obj["result"]["name"].to_string();
}
