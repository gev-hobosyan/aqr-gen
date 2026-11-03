use std::fs::File;
use std::io::Write;
use clap::Parser;
use qr_lib::{ErrorCorrection, Mode, QRCode, Version};
use crate::cli::CLI;
use super::Result;

pub(crate) fn run() -> Result<()> {
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

    let qr_code = QRCode::encode(cli.data, version, error_correction, mode)?;

    let matrix = qr_code.get_matrix();

    let mut file = File::create("qr.ppm").unwrap();

    file.write("P3\n".as_bytes()).unwrap();
    file.write(format!("{} {}\n", matrix.len(), matrix[0].len()).as_bytes())
        .unwrap();
    file.write("1\n".as_bytes()).unwrap();

    for row in matrix {
        let mut color_row = String::from("");
        for (i, col) in row.iter().enumerate() {
            let c = if *col == 0 {
                1
            } else {
                0
            };

            if i < row.len() - 1 {
                color_row.push_str(format!("{} {} {} ", c, c, c).as_str());
            } else {
                color_row.push_str(format!("{} {} {}\n", c, c, c).as_str());
            }
        }

        let _ = file.write(color_row.as_bytes());
    }

    println!("{:?}", cli.output);

    Ok(())
}
