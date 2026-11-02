use qr_lib::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let qr_code = QRCode::encode("Hello world there".to_string(), None, None, None).unwrap();

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
}
