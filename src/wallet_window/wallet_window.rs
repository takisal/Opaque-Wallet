use crate::helpers;
use crate::rpc_methods;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
pub struct WalletWindow {
    pub(crate) name: String,
    pub(crate) age: u32,
    pub(crate) balance: f64,
    pub(crate) wallet_loaded: bool,
    pub(crate) rec_addrs: Vec<String>,
    pub(crate) current_recipient: String,
    pub(crate) current_amount: f64,
    pub(crate) sent_txs: Vec<String>,
    pub(crate) current_amount_string: String,
    pub(crate) check_balance: bool,
    pub(crate) rpc_url: String,
    pub(crate) check_past_txs: bool,
    pub(crate) history_view: bool,
    pub(crate) default_view: bool,
    pub(crate) greeting_view: bool,
    pub(crate) sent_show: bool,
    pub(crate) receive_show: bool,
    pub(crate) all_transactions: Vec<helpers::helpers::Transaction>,
    pub(crate) popup: bool,
    pub(crate) last_transaction: String,
    pub(crate) wash_view: bool,
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
            rpc_url: String::from(""),
            check_past_txs: false,
            history_view: false,
            default_view: false,
            greeting_view: false,
            sent_show: true,
            receive_show: true,
            all_transactions: Vec::new(),
            popup: false,
            last_transaction: String::from(""),
            wash_view: false,
        }
    }
}
impl eframe::App for WalletWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.check_past_txs == true {
            self.all_transactions = Vec::new();
            self.check_past_txs = false;
            helpers::helpers::initialize_wallet(
                self.rpc_url.clone(),
                self.name.to_string(),
                "".to_string(),
                100,
                false,
                &mut self.all_transactions,
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
        egui::TopBottomPanel::top("top_section")
            .exact_height(100.0)
            .show(ctx, |ui| {
                if self.greeting_view {
                    ui.heading("Load/Create Wallet:");
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Wallet name: ");

                        ui.text_edit_singleline(&mut self.name)
                            .labelled_by(name_label.id);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Load").clicked() {
                            let mut rt = tokio::runtime::Runtime::new().unwrap();

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
                                    self.greeting_view = false;
                                    self.default_view = true;
                                }
                                None => {
                                    println!("not loaded");
                                }
                            }
                        }

                        if ui.button("Create").clicked() {
                            let mut rt = tokio::runtime::Runtime::new().unwrap();

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
                                    self.greeting_view = false;
                                    self.default_view = true;
                                }
                            }
                        }
                    });
                    if ui.button("Proceed for to testing").clicked() {
                        self.wallet_loaded = true;
                        self.check_balance = true;
                        self.check_past_txs = true;
                        self.greeting_view = false;
                        self.default_view = true;
                    }
                } else {
                    ui.horizontal(|ui| {
                        if ui.button("Refresh").clicked() {
                            self.check_past_txs = true;
                            self.check_balance = true;
                        }
                        if ui.button("Home").clicked() {
                            self.default_view = true;
                            self.history_view = false;
                        }
                        if ui.button("History").clicked() {
                            self.default_view = false;
                            self.history_view = true;
                        }
                    });
                }
                if !self.greeting_view {
                    ui.add_space(5.0);
                    ui.label(
                        crate::egui::RichText::new(format!("Wallet: {}", self.name))
                            .font(crate::egui::FontId::proportional(20.0)),
                    );
                    ui.add_space(2.5);
                    ui.label(
                        crate::egui::RichText::new(format!("Balance: {} BTC", self.balance))
                            .font(crate::egui::FontId::proportional(20.0)),
                    );
                }
            });
        if self.greeting_view {
            egui::CentralPanel::default().show(ctx, |_ui| {});
        }
        if self.default_view {
            egui::CentralPanel::default().show(ctx, |ui| {
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

                if self.rec_addrs.len() > 1 {
                    ui.label("Your addresses for receiving BTC:");
                } else if self.rec_addrs.len() == 1 {
                    ui.label("Your address for receiving BTC:");
                } else {
                    ui.label("");
                }
                egui::ScrollArea::vertical()
                    .enable_scrolling(true)
                    .min_scrolled_height(100.0)
                    .max_height(100.0)
                    .show(ui, |ui| {
                        let mut rows_count = 0;
                        for addr in &self.rec_addrs {
                            rows_count += 1;
                            ui.horizontal(|row_ui| {
                                let addr_row = row_ui.label(format!("Address: '{}'", addr.clone()));

                                if row_ui
                                    .button("📋")
                                    .on_hover_text("Click to copy")
                                    .labelled_by(addr_row.id)
                                    .clicked()
                                {
                                    row_ui.output_mut(|po| {
                                        po.copied_text = addr.clone();
                                    });
                                }
                            });
                        }
                        if rows_count < 10 {
                            for _ in rows_count..10 {
                                ui.label("            ");
                            }
                        }
                    });

                ui.horizontal(|ui| {
                    let recipient_label = ui.label("Send to: ");
                    ui.text_edit_singleline(&mut self.current_recipient)
                        .labelled_by(recipient_label.id);
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
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
                            self.sent_txs.push(x);
                            self.popup = true
                        }
                    }
                }
            });
        }
        crate::egui::Window::new("Sent Transaction")
            .fixed_pos(&[100.0, 100.0])
            .resizable(true)
            .title_bar(true)
            .open(&mut self.popup)
            .collapsible(false)
            .show(ctx, |ui2| {
                ui2.heading("Transaction Sent");
                ui2.label("Transaction hash: ".to_string() + &self.last_transaction)
            });
        if self.history_view {
            egui::CentralPanel::default().show(ctx, |ui| {
                #[derive(PartialEq)]
                enum Enum {
                    First,
                    Second,
                    Third,
                }
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::RadioButton::new(self.sent_show == true, "Outgoing"))
                        .clicked()
                    {
                        self.sent_show = !self.sent_show;
                    }
                    if ui
                        .add(egui::RadioButton::new(
                            self.receive_show == true,
                            "Incoming",
                        ))
                        .clicked()
                    {
                        self.receive_show = !self.receive_show;
                    }
                });

                ui.label(format!("Transaction History: "));
                ui.push_id(25321, |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::initial(480.0).resizable(true))
                        .column(Column::initial(300.0).resizable(true))
                        .column(Column::initial(100.0).resizable(true))
                        .column(Column::initial(200.0).resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("");
                            });
                            header.col(|ui| {
                                ui.heading("Hash:");
                            });
                            header.col(|ui| {
                                ui.heading("To:");
                            });
                            header.col(|ui| {
                                ui.heading("Amount:");
                            });
                            header.col(|ui| {
                                ui.heading("Time Received:");
                            });
                        })
                        .body(|mut body| {
                            for tx in &self.all_transactions {
                                if (tx.category == "send" && self.sent_show)
                                    || (tx.category == "receive" && self.receive_show)
                                {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            if tx.category == "send" {
                                                ui.label("Sent TX: ");
                                            } else {
                                                ui.label("Recieved TX: ");
                                            }
                                        });
                                        row.col(|ui| {
                                            ui.label(tx.txid.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(tx.address.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(tx.amount.to_string());
                                        });
                                        row.col(|ui| {
                                            ui.label(tx.timereceived.to_string());
                                        });
                                    });
                                }
                            }
                        });
                });
            });
        }
    }
}
