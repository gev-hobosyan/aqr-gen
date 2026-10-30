use std::fmt::format;
use std::fs::File;
use std::io::Write;
use qr_lib::*;

fn main() {
    let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

    let matrix = qr_code.get_matrix();

    let mut file = File::create("qr.ppm").unwrap();

    file.write("P3\n".as_bytes()).unwrap();
    file.write(format!("{} {}\n", matrix.len(), matrix[0].len()).as_bytes()).unwrap();
    file.write("2\n".as_bytes()).unwrap();

    for row in matrix {
        let mut color_row = String::from("");
        for (i, col) in row.iter().enumerate() {
            let c = if *col == 0 { 2 } else if *col == 2 { 1 } else { 0 };

            if i < row.len() - 1 {
                color_row.push_str(format!("{} {} {} ", c, c, c).as_str());
            } else {
                color_row.push_str(format!("{} {} {}\n", c, c, c).as_str());
            }
        }

        file.write(color_row.as_bytes());
    }
}