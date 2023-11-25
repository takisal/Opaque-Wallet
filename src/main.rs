#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]

use eframe::egui;
mod rpc_methods;
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Opaque Wallet",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<WalletWindow>::default()
        }),
    )
}

struct WalletWindow {
    name: String,
    age: u32,
    balance: f64,
    wallet_loaded: bool,
    rec_addrs: Vec<String>,
    current_recipient: String,
    current_amount: f64,
    sent_txs: Vec<String>,
    current_amount_string: String,
    check_balance: bool,
}

impl Default for WalletWindow {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl eframe::App for WalletWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let rpcurl = String::from("");

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
                    rpcurl.clone(),
                    self.name.to_string(),
                    false,
                )) {
                    Some(x) => {
                        let d = x["name"].to_string();
                        println!("Loaded: {}", d.clone());
                        self.wallet_loaded = true;
                        self.check_balance = true;
                    }
                    None => {
                        println!("not loaded");
                        self.wallet_loaded = false;
                    }
                }
            }
            if self.check_balance == true {
                self.check_balance = false;
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(rpc_methods::wallet_rpcs::get_balance(
                    rpcurl.clone(),
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
                    rpcurl.clone(),
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
                    }
                }
            }
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Show new receive address").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                match rt.block_on(rpc_methods::wallet_rpcs::get_new_address(
                    rpcurl.clone(),
                    self.name.to_string(),
                    "".to_string(),
                    String::from(""),
                )) {
                    x @ _ => {
                        println!("New address: {}", x);
                        self.rec_addrs.push(x);
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
                    rpcurl,
                    self.name.to_string(),
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
        });
    }
}
