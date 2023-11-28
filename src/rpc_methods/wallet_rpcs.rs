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
    println!("Rawtxt: {}", raw_text);
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
    println!("newstr: {}", newstr);
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
    if fee_rate == 0 {
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
pub async fn dump_priv_key(mut url: String, wallet_name: String, address: String) -> String {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "dumpprivkey", "params":{"address": ""#
            .to_owned()
            + &address.to_string()
            + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}
pub async fn dump_wallet(mut url: String, wallet_name: String, filename: String) -> String {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "dumpwallet", "params":{"filename": ""#
            .to_owned()
            + &filename.to_string()
            + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}
pub async fn encrypt_wallet(mut url: String, wallet_name: String, passphrase: String) -> String {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "encryptwallet", "params":{"passphrase": ""#
            .to_owned()
            + &passphrase.to_string()
            + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}
pub async fn get_addresses_by_label(
    mut url: String,
    wallet_name: String,
    label: String,
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getaddressesbylabel", "params":{"label": ""#
            .to_owned()
            + &label.to_string()
            + r#""}}"#;

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
pub async fn get_address_info(
    mut url: String,
    wallet_name: String,
    address: String,
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getaddressinfo", "params":{"address": ""#
            .to_owned()
            + &address.to_string() + r#""}}"#;

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
pub async fn get_balance(
    mut url: String,
    wallet_name: String,
    minconf: u32,
    include_watchonly: bool,
    avoid_reuse: bool,
) -> f64 {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getbalance", "params":{"minconf": "#
            .to_owned()
            + &minconf.to_string()
            + r#", "include_watchonly": "#
            + &include_watchonly.to_string()
            + r#", "avoid_reuse": "#
            + &avoid_reuse.to_string()
            + r#"}}"#;
    println!("newstr: {}", (newstr.clone()));
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    println!("raw_text: {}", (raw_text.clone()));
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    match obj["result"].as_f64() {
        Some(x) => x,
        None => {
            println!("Error fetching balance");
            0.0
        }
    }
}
pub async fn get_balances(mut url: String, wallet_name: String) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr = r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getbalances", "params":[]}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return match obj["result"].as_object() {
        Some(x) => Some(x.clone()),
        None => {
            println!("Error fetching balance");
            None
        }
    };
}
pub async fn get_raw_change_address(
    mut url: String,
    wallet_name: String,
    address_type: String,
) -> String {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getrawchangeaddress", "params":{"address_type": ""#
            .to_owned()
            + &address_type.to_string()
            + r#""}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    return obj["result"].to_string();
}
pub async fn get_received_by_address(
    mut url: String,
    wallet_name: String,
    address: String,
    minconf: u32,
) -> f64 {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getreceivedbyaddress", "params":{"address": ""#
            .to_owned()
            + &address.to_string()
            + r#"", "minconf": "# +&minconf.to_string() + r#"}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    match obj["result"].as_f64() {
        Some(x) => return x,
        None => {
            println!("Error fetching received by address");
            return 0.0;
        }
    }
}
pub async fn get_received_by_label(
    mut url: String,
    wallet_name: String,
    label: String,
    minconf: u32,
) -> f64 {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getreceivedbylabel", "params":{"label": ""#
            .to_owned()
            + &label.to_string()
            + r#"", "minconf": "# +&minconf.to_string() + r#"}}"#;

    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    match obj["result"].as_f64() {
        Some(x) => return x,
        None => {
            println!("Error fetching received by label");
            return 0.0;
        }
    }
}
pub async fn get_transaction(
    mut url: String,
    wallet_name: String,
    txid: String,
    include_watchonly: bool,
    verbose: bool,
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getaddressinfo", "params":{"txid": ""#
            .to_owned()
            + &txid.to_string()
            + r#"", "include_watchonly": "#
            + &include_watchonly.to_string()
            + r#", "verbose": "#
            + &verbose.to_string()
            + r#"}}"#;

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
//get_unconfirmed_balance is DEPRECATED
pub async fn get_wallet_info(mut url: String, wallet_name: String) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr = r#"{"jsonrpc": "1.0", "id": "curltest", "method": "getwalletinfo", "params":[]}"#;

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
pub async fn import_address(
    mut url: String,
    wallet_name: String,
    address: String,
    label: String,
    rescan: bool,
    p2sh: bool,
) {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "importaddress", "params":{"address": ""#
            .to_owned()
            + &address.to_string()
            + r#"", "label": ""#
            + &label.to_string()
            + r#"", "rescan": "#
            + &rescan.to_string()
            + r#", "p2sh": "#
            + &p2sh.to_string()
            + r#"}}"#;
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
}
pub async fn import_descriptors(
    mut url: String,
    wallet_name: String,
    descriptors: String, //descriptors must be in JSON string format. See https://developer.bitcoin.org/reference/rpc/importdescriptors.html.
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr = r#"{"jsonrpc": "1.0", "id": "curltest", "method": "importdescriptors", "params":{"descriptors": "#.to_owned()
        + &descriptors.to_string()
        + r#"}}"#;

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
pub async fn import_multi(
    mut url: String,
    wallet_name: String,
    requests: String, //requests must be in JSON string format. See https://developer.bitcoin.org/reference/rpc/importmulti.html.
    options: String, //options must be in JSON string format. See https://developer.bitcoin.org/reference/rpc/importmulti.html.
) -> Option<Map<String, Value>> {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "importmulti", "params":{"requests": "#
            .to_owned()
            + &requests.to_string()
            + r#", "options": "#
            + &options.to_string()
            + r#"}}"#;

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
pub async fn import_priv_key(
    mut url: String,
    wallet_name: String,
    privkey: String,
    label: String,
    rescan: bool,
) {
    url = url + "/wallet/" + &wallet_name;
    let newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "importprivkey", "params":{"privkey": ""#
            .to_string()
            + &privkey.to_string()
            + r#"", "label": ""#
            + &label.to_string()
            + r#"", "rescan": "#
            + &rescan.to_string()
            + r#"}}"#;
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    return;
}
pub async fn list_transactions(
    mut url: String,
    wallet_name: String,
    label: String,
    count: u32,
    include_watchonly: bool,
) -> Vec<crate::helpers::helpers::Transaction> {
    url = url + "/wallet/" + &wallet_name;
    let mut newstr =
        r#"{"jsonrpc": "1.0", "id": "curltest", "method": "listtransactions", "params":{"#
            .to_string();
    let label_section = r#""label": ""#.to_owned() + &label + r#"""#;
    let count_section = r#""count": "#.to_owned() + &count.to_string();
    if label != "" && count != 0 {
        newstr = newstr + &label_section + ", " + &count_section + ", "
    } else if label != "" {
        newstr = newstr + &label_section + ", "
    } else if count != 0 {
        newstr = newstr + &count_section + ", "
    }
    newstr = newstr + r#""include_watchonly": "# + &include_watchonly.to_string() + r#"}}"#;
    println!("newstr: {}", newstr);
    println!("walletname: {}", wallet_name);
    let client = reqwest::Client::new();
    let _resp0 = client.post(&url).body(newstr).send().await.unwrap();
    let raw_text = _resp0.text().await.unwrap();
    println!("raw_text: {}", raw_text);
    let parsed: Value = serde_json::from_str(&raw_text).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let k = <Vec<crate::helpers::helpers::Transaction> as serde::Deserialize>::deserialize(
        obj["result"].clone(),
    );
    return match k {
        Ok(mut x) => {
            x.reverse();
            x
        }
        Err(e) => {
            println!("Failed decode {}", e);
            let mut curated_vector = Vec::<crate::helpers::helpers::Transaction>::new();
            let p = <Vec<crate::helpers::helpers::PendingTransaction> as serde::Deserialize>::deserialize(
                obj["result"].clone(),
            );
            match p {
                Ok(y) => {
                    for ptx in y {
                        curated_vector.push(crate::helpers::helpers::Transaction {
                            address: ptx.address,
                            amount: ptx.amount,
                            blockheight: 0,
                            blockindex: 0,
                            blocktime: 0,
                            category: ptx.category,
                            confirmations: 0,
                            label: "".to_string(),
                            time: ptx.time,
                            timereceived: ptx.timereceived,
                            txid: ptx.txid,
                            vout: ptx.vout,
                            walletconflicts: ptx.walletconflicts,
                        })
                    }
                    curated_vector.reverse();
                    return curated_vector;
                }
                Err(e) => {
                    println!("Failed pending decode {}", e);
                    return Vec::<crate::helpers::helpers::Transaction>::new();
                }
            }
        } /*
          return match obj["result"].as_array() {
              Some(x) => Some(x.clone()),
              None => None,
          };*/
    };
}
