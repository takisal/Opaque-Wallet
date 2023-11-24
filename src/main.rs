#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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
}

impl Default for WalletWindow {
    fn default() -> Self {
        Self {
            name: "rusttestie".to_owned(),
            age: 42,
            balance: 0.0,
            wallet_loaded: false,
            rec_addrs: Vec::new(),
        }
    }
}

impl eframe::App for WalletWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Load Wallet:");
            ui.horizontal(|ui| {
                let name_label = ui.label("Directory Wallet file is in: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            if ui.button("Load Wallet").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();

                //call load wallet function
                let rpcurl = String::from("");
                match rt.block_on(rpc_methods::wallet_rpcs::load_wallet(
                    rpcurl,
                    "rusttestie".to_string(),
                    false,
                )) {
                    Some(x) => {
                        let d = x["name"].to_string();
                        println!("Loaded: {}", d.clone());
                        self.name = d.clone();
                        self.wallet_loaded = true;
                    }
                    None => {
                        println!("not loaded");
                        self.wallet_loaded = false;
                    }
                }
            }
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Show new receive address").clicked() {
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                let rpcurl = String::from("");
                match rt.block_on(rpc_methods::wallet_rpcs::get_new_address(
                    rpcurl,
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
                ui.label(format!("Address '{}'", addr.clone()));

                if ui.button("📋").on_hover_text("Click to copy").clicked() {
                    ui.output_mut(|po| {
                        po.copied_text = addr.clone();
                    });
                }
            }
        });
    }
}
