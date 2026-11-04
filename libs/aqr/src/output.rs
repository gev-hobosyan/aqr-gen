use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use clap::ValueEnum;
use qr_lib::error::AQRError;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputFormat {
    Png,
    Ppm
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Ppm => write!(f, "ppm")
        }
    }
}

impl OutputFormat {
    pub(crate) fn write_file(self, matrix: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        match self {
            OutputFormat::Png => {
                todo!()
            }
            OutputFormat::Ppm => {
                PPM::write_file(matrix, file_name)?;
            }
        }
        
        Ok(())
    }
}

pub(crate) trait Output {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError>;
}

pub(crate) struct PPM;

impl Output for PPM {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        let mut file = File::create(format!("{file_name}.ppm").as_str()).unwrap();

        file.write("P3\n".as_bytes()).unwrap();
        file.write(format!("{} {}\n", data.len(), data[0].len()).as_bytes())
            .unwrap();
        file.write("1\n".as_bytes()).unwrap();

        for row in data {
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

            file.write(color_row.as_bytes()).unwrap();
        }
        
        Ok(())
    }
}