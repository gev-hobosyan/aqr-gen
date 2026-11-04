use clap::Parser;
use qr_lib::{ErrorCorrection, Mode, QRCode, Version};
use crate::cli::CLI;
use super::Result;

pub(crate) fn run() -> Result<String> {
    let cli = CLI::parse();

    let version = match cli.version {
        Some(v) => { Some(Version::from(v)?) }
        None => { None }
    };

    let mode = match cli.mode {
        Some(m) => {
            let mode: Mode = m.try_into()?;

            Some(mode)
        }
        None => { None }
    };

    let error_correction = match cli.error_correction {
        Some(ec) => {
            let error_correction: ErrorCorrection = ec.try_into()?;

            Some(error_correction)
        }
        None => { None }
    };

    let file_name = cli.name.unwrap_or("qr".to_string());

    let qr_code = QRCode::encode(cli.data.clone(), version, error_correction, mode)?;

    let matrix = qr_code.get_matrix();

    cli.output.write_file(matrix, &file_name)?;

    Ok(format!("A {file_name}.{} has been generated", cli.output))
}
