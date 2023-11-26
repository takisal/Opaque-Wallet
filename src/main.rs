#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]

use eframe::egui;
use std::fs;
mod rpc_methods;
use serde::Deserialize;
use serde::Serialize;
//{"address":"bc1qwujyk6fdyz94yeypxnjjghth65y4j50569ymm3","amount":0.003,"bip125-replaceable":"no","blockhash":"00000000000000000001cb2cea4a6ffedf31324afc19474d244bedb03d267461","blockheight":818523,
//"blockindex":31,"blocktime":1700973976,"category":"receive","confirmations":4,"label":"","time":1700973857,"timereceived":1700973857,"txid":"a3f80c6411c760943ede19f231aea602f1390da72cb12a842294fd668b63f22a","vout":1,"walletconflicts":[]}
#[derive(Serialize, Deserialize)]
struct Transaction {
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
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let file_path_stem = "";
    let file_path = &(file_path_stem.to_string() + "/.cookie");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    let rpc_url: String = "http://".to_string() + &contents + "@127.0.0.1:8332";
    //query list transactions and split into sent/recieved

    eframe::run_native(
        "Opaque Wallet",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(WalletWindow {
                name: "rusttestie".to_owned(),
                age: 42,
                balance: 0.0,
                wallet_loaded: false,
                rec_addrs: Vec::new(),
                current_recipient: String::from(""),
                current_amount: 0.0,
                sent_txs: Vec::new(),
                current_amount_string: String::from(""),
                check_balance: false,
                rpc_url,
                check_past_txs: false,
                sends: Vec::new(),
                receives: Vec::new(),
            })
        }),
    )
}
fn initialize_wallet(
    url: String,
    wallet_name: String,
    label: String,
    count: u32,
    include_watchonly: bool,
    sends: &mut Vec<Transaction>,
    receives: &mut Vec<Transaction>,
) {
    println!("eyy");
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
                    println!("categorySend: {}", tx.category);
                    sends.push(tx);
                } else if tx.category == "receive" {
                    println!("categoryReceive: {}", tx.category);
                    receives.push(tx);
                }
            }
        }
    }
}
struct WalletWindow {
    name: String,
    age: u32,
    balance: f64,
    wallet_loaded: bool,
    rec_addrs: Vec<String>,
    //vec of tx structs for receive
    //vec of tx structs for sent
    current_recipient: String,
    current_amount: f64,
    sent_txs: Vec<String>,
    current_amount_string: String,
    check_balance: bool,
    rpc_url: String,
    check_past_txs: bool,
    sends: Vec<Transaction>,
    receives: Vec<Transaction>,
}

impl Default for WalletWindow {
    fn default() -> Self {
        Self {
            name: "rusttestie".to_owned(),
            age: 42,
            balance: 0.0,
            wallet_loaded: false,
            rec_addrs: Vec::new(),
            //vec of tx structs for receive
            //vec of tx structs for sent
            current_recipient: String::from(""),
            current_amount: 0.0,
            sent_txs: Vec::new(),
            current_amount_string: String::from(""),
            check_balance: false,
            rpc_url: String::from(""),
            check_past_txs: false,
            sends: Vec::new(),
            receives: Vec::new(),
        }
    }
}

impl eframe::App for WalletWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Load/Create Wallet:");
            ui.horizontal(|ui| {
                let name_label = ui.label("Wallet name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            if ui.button("Load").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                //call load wallet function

                match rt.block_on(rpc_methods::wallet_rpcs::load_wallet(
                    self.rpc_url.clone(),
                    self.name.to_string(),
                    false,
                )) {
                    Some(x) => {
                        let d = x["name"].to_string();
                        println!("Loaded: {}", d.clone());
                        self.wallet_loaded = true;
                        self.check_balance = true;
                        self.check_past_txs = true;
                    }
                    None => {
                        println!("not loaded");
                        self.wallet_loaded = false;
                        self.check_balance = false;
                    }
                }
            }
            if ui.button("Test getting past txs").clicked() {
                self.check_past_txs = true
            }

            if self.check_past_txs == true {
                self.check_past_txs = false;
                initialize_wallet(
                    self.rpc_url.clone(),
                    self.name.to_string(),
                    "".to_string(),
                    100,
                    false,
                    &mut self.sends,
                    &mut self.receives,
                );
            }

            if self.check_balance == true {
                self.check_balance = false;
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(rpc_methods::wallet_rpcs::get_balance(
                    self.rpc_url.clone(),
                    self.name.clone(),
                    1,
                    false,
                    false,
                )) {
                    x => {
                        self.balance = x;
                    }
                }
            }
            if ui.button("Refresh balance").clicked() {
                self.check_balance = true
            }
            if ui.button("Create").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                //call load wallet function

                match rt.block_on(rpc_methods::wallet_rpcs::create_wallet(
                    self.rpc_url.clone(),
                    self.name.to_string(),
                    false,
                    false,
                    "".to_string(),
                    false,
                    false,
                    false,
                )) {
                    x => {
                        println!("Loaded: {}", x.clone());
                        self.wallet_loaded = true;
                        self.check_balance = true;
                        self.check_past_txs = true;
                    }
                }
            }
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Show new receive address").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                match rt.block_on(rpc_methods::wallet_rpcs::get_new_address(
                    self.rpc_url.clone(),
                    self.name.clone(),
                    "".to_string(),
                    String::from(""),
                )) {
                    x @ _ => {
                        println!("New address: {}", x);
                        let mut chars = x.chars();
                        chars.next();

                        chars.next_back();

                        self.rec_addrs.push(String::from(chars.as_str()));
                    }
                }
            }
            ui.label(format!(
                "Wallet '{}',  balance: {}",
                self.name, self.balance
            ));
            for addr in &self.rec_addrs {
                ui.horizontal(|ui| {
                    let addr_row = ui.label(format!("Address: '{}'", addr.clone()));

                    if ui
                        .button("📋")
                        .on_hover_text("Click to copy")
                        .labelled_by(addr_row.id)
                        .clicked()
                    {
                        ui.output_mut(|po| {
                            po.copied_text = addr.clone();
                        });
                    }
                });
            }

            ui.horizontal(|ui| {
                let recipient_label = ui.label("Send to: ");
                ui.text_edit_singleline(&mut self.current_recipient)
                    .labelled_by(recipient_label.id);
                let amount_label = ui.label("Amount: ");
                ui.text_edit_singleline(&mut self.current_amount_string)
                    .labelled_by(amount_label.id);
                let parsed_amount = match self.current_amount_string.clone().parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => 0.0,
                };
                self.current_amount = parsed_amount;
            });
            if ui.button("Send").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                match rt.block_on(rpc_methods::wallet_rpcs::send_to_address(
                    self.rpc_url.clone(),
                    self.name.clone(),
                    self.current_recipient.clone(),
                    self.current_amount,
                    "".to_string(),
                    "".to_string(),
                    false,
                    true,
                    1,
                    "conservative".to_string(),
                    false,
                    0,
                    true,
                )) {
                    x => {
                        println!("TXID: {}", x);
                        self.sent_txs.push(x);
                    }
                }
            }

            for tx in &self.sent_txs {
                ui.label(format!("TX ID:  '{}'", tx.clone(),));
            }
            ui.label(format!("Sent Transaction History: "));
            for tx in &self.sends {
                let tx_display = serde_json::to_string(tx).unwrap();
                ui.label(format!("TX:  '{}'", tx_display));
            }

            ui.label(format!("Received Transaction History: "));
            for tx in &self.receives {
                let tx_display = serde_json::to_string(tx).unwrap();
                ui.label(format!("TX:  '{}'", tx_display));
            }
        });
    }
}
