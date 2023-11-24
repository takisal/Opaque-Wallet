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
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
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
//fee rate is mutually-exclusive with conf_target and estimate_mode
pub async fn send_to_address(
    mut url: String,
    wallet_name: String,
    address: String,
    amount: f64,
    comment: String,
    comment_to: String,
    subtractfeefromamount: bool,
    replaceable: bool,
    conf_target: u32,
    estimate_mode: String,
    avoid_reuse: bool,
    fee_rate: u32,
    verbose: bool,
) -> String {
    url = url + "/wallet/" + &wallet_name;

    let newstr;
    //{"jsonrpc": "1.0", "id": "curltest", "method": "sendtoaddress", "params": {"address":"3ATLibS1aRMtAyigk9HbUtsx1DFSWDbDw1", "amount": 0.002, "comment": "testsend", "comment_to": "selfmultisig", "subtractfeefromamount": false, "replaceable": true, "conf_target": 0, "estimate_mode":"conservative","avoid_reuse": false, "fee_rate":25,"verbose": false}}
    if fee_rate == 0 {
        newstr =  r#"{"jsonrpc": "1.0", "id": "curltest", "method": "sendtoaddress", "params": {"address":""#
        .to_owned()
        + &address.to_string()
        + r#"", "amount": "#
        + &amount.to_string()
        + r#", "comment": ""#
        + &comment.to_string()
        + r#"", "comment_to": ""#
        + &comment_to.to_string()
        + r#"", "subtractfeefromamount": "#
        + &subtractfeefromamount.to_string()
        + r#", "replaceable": "#
        + &replaceable.to_string()
        + r#", "conf_target": "#
        + &conf_target.to_string()
        + r#", "estimate_mode":""#
        + &estimate_mode.to_string()
        + r#"","avoid_reuse": "#
        + &avoid_reuse.to_string()
        + r#","verbose": "#
        + &verbose.to_string()
        + r#"}}"#;
    } else {
        newstr =  r#"{"jsonrpc": "1.0", "id": "curltest", "method": "sendtoaddress", "params": {"address":""#
        .to_owned()
        + &address.to_string()
        + r#"", "amount": "#
        + &amount.to_string()
        + r#", "comment": ""#
        + &comment.to_string()
        + r#"", "comment_to": ""#
        + &comment_to.to_string()
        + r#"", "subtractfeefromamount": "#
        + &subtractfeefromamount.to_string()
        + r#", "replaceable": "#
        + &replaceable.to_string()
        + r#","avoid_reuse": "#
        + &avoid_reuse.to_string()
        + r#", "fee_rate":"#
        + &fee_rate.to_string()
        + r#","verbose": "#
        + &verbose.to_string()
        + r#"}}"#;
    }
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    return obj["result"]["txid"].to_string();
}
pub async fn load_wallet(
    url: String,
    filename: String,
    load_on_startup: bool,
) -> Option<Map<String, Value>> {
    //let nonLoad = r#"", "#.to_owned() + &load_on_startup.to_string() + r#"]}"#;
    let newstr = (r#"{"jsonrpc": "1.0", "id": "curltest", "method": "loadwallet", "params": [""#
        .to_owned()
        + &filename
        + r#"", "#
        + &load_on_startup.to_string()
        + r#"]}"#)
        .to_string();

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return match obj["result"].as_object() {
        Some(x) => Some(x.clone()),
        None => None,
    };
}
pub async fn get_new_address(
    mut url: String,
    wallet_name: String,
    label: String,
    address_type: String,
) -> String {
    url = url + "/wallet/" + &wallet_name;
    let mut newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getnewaddress", "params": {"#.to_owned();
    if label != "" && address_type != "" {
        newstr =
            newstr + r#""label": ""# + &label + r#"", "address_type": ""# + &address_type + r#"""#
    } else if label != "" && address_type == "" {
        newstr = newstr + r#""label": ""# + &label + r#"""#
    } else if label == "" && address_type != "" {
        newstr = newstr + r#""address_type": ""# + &address_type + r#"""#
    }
    newstr = newstr + r#"}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}
pub async fn abort_rescan(mut url: String, wallet_name: String) -> String {
    url = url + "/wallet/" + &wallet_name;
    let newstr = r#"{"jsonrpc": "1.0", "id": "curltest", "method": "abortrescan"}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}

/*
Returns:
{                            (json object)
  "address" : "str",         (string) The value of the new multisig address
  "redeemScript" : "hex",    (string) The string value of the hex-encoded redemption script
  "descriptor" : "str"       (string) The descriptor for this multisig
}
 */
pub async fn add_multisig_address(
    mut url: String,
    wallet_name: String,
    nrequired: u32,
    keys: Vec<String>,
    label: String,
    address_type: String,
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let mut newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "addmultisigaddress", "params": {"#
            .to_owned();
    newstr = newstr + r#""nrequired":"# + &nrequired.to_string() + r#", "keys": ["#;
    let mut p = 0;
    for i in keys.iter() {
        if p > 0 {
            newstr = newstr + ", ";
        }
        newstr = newstr + r#"""# + i + r#"""#;
        p += 1;
    }
    newstr = newstr + "]";
    if label != "" && address_type != "" {
        newstr =
            newstr + r#", "label": ""# + &label + r#"", "address_type": ""# + &address_type + r#"""#
    } else if label != "" && address_type == "" {
        newstr = newstr + r#", "label": ""# + &label + r#"""#
    } else if label == "" && address_type != "" {
        newstr = newstr + r#", "address_type": ""# + &address_type + r#"""#
    }
    newstr = newstr + r#"}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr.clone()).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return match obj["result"].as_object() {
        Some(x) => Some(x.clone()),
        None => None,
    };
}
pub async fn backup_wallet(mut url: String, wallet_name: String, destination: String) {
    url = url + "/wallet/" + &wallet_name;
    let newstr = r#"{"jsonrpc": "1.0", "id": "curltest", "method": "backupwallet", "params":{"destination": ""#
        .to_owned()
        + &destination.to_string()
        + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
}
//{"jsonrpc": "1.0", "id": "curltest", "method": "bumpfee", "params": {"txid":"3ATLibS1aRMtAyigk9HbUtsx1DFSWDbDw1", "options":{"conf_target": 2, "fee_rate": 25, "replaceable": true, "estimate_mode": "economical"}}}
pub async fn bump_fee(
    mut url: String,
    wallet_name: String,
    txid: String,
    conf_target: u32,
    fee_rate: u32,
    replaceable: bool,
    estimate_mode: String,
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let mut newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "bumpfee", "params": {"txid":""#
            .to_owned()
            + &txid
            + r#"", "options":{"#;
    if (fee_rate == 0) {
        newstr = newstr
            + r#""conf_target": "#
            + &conf_target.to_string()
            + r#", "replaceable": "#
            + &replaceable.to_string()
            + r#", "estimate_mode": ""#
            + &estimate_mode
            + r#"""#;
    } else {
        newstr = newstr
            + r#""fee_rate": "#
            + &fee_rate.to_string()
            + r#", "replaceable": "#
            + &replaceable.to_string();
    }
    newstr = newstr + r#"}}}"#;
    println!("newstr: {}", newstr);
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return match obj["result"].as_object() {
        Some(x) => Some(x.clone()),
        None => None,
    };
}
pub async fn dump_priv_key(mut url: String, wallet_name: String, address: String) {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "backupwallet", "params":{"address": ""#
            .to_owned()
            + &address.to_string()
            + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
}
