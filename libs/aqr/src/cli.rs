use std::io;
use clap::{Parser};
use colored::Colorize;
use figlet_rs::FIGlet;
use qr_lib::error::AQRError;
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "aqr")]
#[command(version = "0.1")]
#[command(about = "A simple CLI that give full control over your QR Codes")]
#[command(disable_version_flag = true)]
#[command(disable_help_flag = true)]
pub struct CLI {
    #[arg(short, long)]
    pub(crate) help: bool,

    #[arg(required_unless_present = "help")]
    pub(crate) data: Option<String>,

    #[arg(short, long)]
    pub(crate) version: Option<u8>,

    #[arg(short, long)]
    pub(crate) error_correction: Option<char>,

    #[arg(short, long)]
    pub(crate) mode: Option<char>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Ppm)]
    pub(crate) output: OutputFormat,

    #[arg(short, long)]
    pub(crate) file_path: Option<String>,
}

impl CLI {
    pub fn print_help(&self) -> Result<(), String> {
        let font = FIGlet::from_content(include_str!("../fonts/shadow.flf"))?;
        let logo = font.convert("Argo's QR Gen");

        if let Some(logo) = logo {
            println!("\n\n{}", logo.to_string().green());
        }
        println!("{}: {} {}", "Usage".bold().green(), "aqr".bold().bright_green(), "<data> [options]".truecolor(124, 252, 124));
        println!("       {} {}", "aqr".bold().bright_green(), "--help/-h".truecolor(124, 252, 124));

        println!("{}", "\nA simple CLI to generate QR Codes\n".bright_green());

        println!("Options: ");

        println!("    -v, --version: {}", "Assigns the QR Code version".bright_black());
        println!("        {}", "Supported versions: 1-6".bright_black());

        println!("    -m, --mode: {}", "Assigns the QR Code mode".bright_black());
        println!("        {}", "Supported modes: N(Numeric), A(Alphanumeric), B(Byte), K(Kanji)".bright_black());

        println!("    -e, --error_correction: {}", "Assigns the QR Code error correction level".bright_black());
        println!("        {}", "Supported error correction levels: L, M, Q, H".bright_black());

        println!("    -o, --output: {}", "Specifies the output format".bright_black());
        println!("        {}", "Supported output formats: ppm, png, jpg, jpeg, svg, terminal".bright_black());
        println!("        {}", "Defaults to ppm".bright_black());

        println!("    -f, --file_path: The output file path, without the extension");
        println!("        {}", "Defaults to \"qr\"".bright_black());

        println!("\n{}", "Source code: https://github.com/gev-hobosyan/aqr-gen.git".truecolor(140, 245, 140));
        println!("{}", "Will be happy to see your feedback".truecolor(140, 245, 140));

        Ok(())
    }
}