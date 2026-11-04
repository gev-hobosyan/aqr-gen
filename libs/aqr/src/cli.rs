use clap::{Parser};
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "aqr")]
#[command(version = "0.1")]
#[command(about = "A simple CLI that give full control over your QR Codes")]
#[command(disable_version_flag = true)]
pub struct CLI {
    pub(crate) data: String,

    #[arg(short, long)]
    pub(crate) version: Option<u8>,

    #[arg(short, long)]
    pub(crate) error_correction: Option<char>,

    #[arg(short, long)]
    pub(crate) mode: Option<char>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Ppm)]
    pub(crate) output: OutputFormat,

    #[arg(short, long)]
    pub(crate) name: Option<String>,
}

impl CLI {
}