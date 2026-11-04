mod cli;
mod app;
mod output;

use colored::Colorize;
use qr_lib::error::AQRError;
use crate::app::run;

pub type Result<T> = core::result::Result<T, AQRError>;

fn main() {
    let result = run();

    match result {
        Ok(msg) => {
            println!("{} {msg}", "[SUCCESS]".bold().green())
        }
        Err(err) => {
            println!("{} {err}", "[ERROR]".bold().red());
        }
    }
}