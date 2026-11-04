use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::io::Write;
use clap::ValueEnum;
use image::{ImageBuffer, ImageError, ImageResult, Rgb};
use svg::Document;
use svg::node::element::Rectangle;
use qr_lib::error::AQRError;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputFormat {
    Png,
    Ppm,
    Jpg,
    Jpeg,
    Svg,
    Terminal
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Ppm => write!(f, "ppm"),
            OutputFormat::Jpg => write!(f, "jpg"),
            OutputFormat::Jpeg => write!(f, "jpeg"),
            OutputFormat::Svg => write!(f, "svg"),
            OutputFormat::Terminal => write!(f, "terminal")
        }
    }
}

impl OutputFormat {
    pub(crate) fn write_file(self, matrix: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        match self {
            OutputFormat::Png => {
                PNG::write_file(matrix, file_name)?;
            }
            OutputFormat::Ppm => {
                PPM::write_file(matrix, file_name)?;
            }
            OutputFormat::Jpg => {
                JPG::write_file(matrix, file_name)?;
            }
            OutputFormat::Jpeg => {
                JPEG::write_file(matrix, file_name)?;
            }
            OutputFormat::Svg => {
                SVG::write_file(matrix, file_name)?;
            }
            OutputFormat::Terminal => {
                Terminal::write_file(matrix, file_name)?;
            }
        }

        Ok(())
    }
}

pub(crate) trait Output {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError>;
}

pub(crate) struct PPM;
pub(crate) struct JPG;
pub(crate) struct JPEG;
pub(crate) struct PNG;
pub(crate) struct SVG;
pub(crate) struct Terminal;

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


impl Output for PNG {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        match matrix_to_img_buf(data).save(format!("{file_name}.png")){
            Ok(_) => Ok(()),
            Err(err) => Err(AQRError::Error(err.to_string()))
        }
    }
}

impl Output for JPEG {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        match matrix_to_img_buf(data).save(format!("{file_name}.jpeg")){
            Ok(_) => Ok(()),
            Err(err) => Err(AQRError::Error(err.to_string()))
        }
    }
}

impl Output for JPG {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        match matrix_to_img_buf(data).save(format!("{file_name}.jpg")){
            Ok(_) => Ok(()),
            Err(err) => Err(AQRError::Error(err.to_string()))
        }
    }
}

impl Output for SVG {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        let mut document = Document::new()
            .set("viewBox", (0, 0, data.len(), data.len()))
            .set("width", data.len())
            .set("height", data.len());

        for (y, row) in data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let rect = Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", 1)
                    .set("height", 1)
                    .set("fill", if *col == 1 { "#000" } else { "#fff" });

                document = document.add(rect);
            }
        }

        match svg::save(format!("{file_name}.svg"), &document) {
            Ok(_) => Ok(()),
            Err(err) => Err(AQRError::Error(err.to_string()))
        }
    }
}

impl Output for Terminal {
    fn write_file(data: Vec<Vec<u8>>, file_name: &String) -> Result<(), AQRError> {
        // To compensate for additional columns
        for _ in 0..=data.len() + 1 {
            print!("⬜");
        }
        println!();

        for row in &data {
            print!("⬜");
            for col in row {
                print!("{}", if *col == 1 { "⬛" } else {"⬜"})
            }
            println!("⬜");
        }

        // To compensate for additional columns
        for _ in 0..=data.len() + 1 {
            print!("⬜");
        }
        println!();

        Ok(())
    }
}

fn matrix_to_img_buf(matrix: Vec<Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(matrix.len() as u32, matrix.len() as u32);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = if matrix[x as usize][y as usize] == 1 {
            0u8
        } else {
            255u8
        };

        *pixel = Rgb([ color; 3 ]);
    }

    image
}