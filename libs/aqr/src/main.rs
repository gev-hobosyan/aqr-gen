mod cli;
mod app;
mod output;

use qr_lib::error::AQRError;
use crate::app::run;

pub type Result<T> = core::result::Result<T, AQRError>;

fn main() {
    let result = run();

    match result {
        Ok(_) => {
            println!("Success")
        }
        Err(err) => {
            println!("{err}");
        }
    }
}