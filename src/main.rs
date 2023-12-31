#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]

mod helpers;
mod wallet_window;
use eframe::egui;
use std::env;
use std::fs;
mod rpc_methods;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1180.0, 420.0]),
        ..Default::default()
    };
    let res = env::current_dir();
    let mut file_path_stem = String::from("");
    match res {
        Ok(x) => {
            file_path_stem = x.to_str().unwrap().to_owned();
        }
        Err(e) => {
            println!("Error getting working directory {}", e)
        }
    }
    let file_path = file_path_stem + "/.cookie";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rpc_url: String = "http://".to_string() + &contents + "@127.0.0.1:8332";
    //query list transactions and split into sent/recieved

    eframe::run_native(
        "Opaque Wallet",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(wallet_window::wallet_window::WalletWindow {
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
                history_view: false,
                default_view: false,
                greeting_view: true,
                sent_show: true,
                receive_show: true,
                all_transactions: Vec::new(),
                popup: false,
                last_transaction: String::from(""),
                wash_view: false,
            })
        }),
    )
}
