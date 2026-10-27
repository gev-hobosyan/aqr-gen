struct QRCode {
    mode: u8,
    version: u8,
    string_length: usize,
    codewords: u8,
    data: Vec<u8>
}

impl QRCode {
    pub fn encode(version: u8, string: String) -> QRCode {
        // TODO: use string_length to decide the version
        let string_length = string.len();
        // TODO: dynamic mode choosing
        let mode = 0b0100 << 4;
        // TODO: assign codewords dependant on the version
        let codewords = 28u8;

        let mut data = vec![0; codewords as usize];

        let mut byte;

        byte = mode | (string_length as u8 >> 4);

        data[0] = byte;
        data[1] = (string_length as u8) << 4;

        let chars: Vec<char> = string.chars().collect();

        for (i, ch) in string.chars().enumerate() {
            if i == 0 {
                data[1] = data[1] | (ch as u8 >> 4);
                continue;
            }

            let lch = chars[i - 1] as u8;

            data[i + 1] = (lch << 4) | (ch as u8 >> 4);

            if i + 1 == string_length {
                data[string_length + 1] = (ch as u8) << 4;
            }
        }

        let rsi = string_length + 2;
        let constant = rsi % 2;
        let placeholders = [236, 17];

        for i in rsi..(codewords as usize) {
            let index = (i + constant) % 2;

            data[i] = placeholders[index];
        }

        QRCode {
            mode,
            version,
            string_length,
            codewords,
            data
        }
    }

    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn codewords(&self) -> u8 {
        self.codewords
    }

    pub fn string_length(&self) -> usize {
        self.string_length
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::QRCode;

    #[test]
    fn test_encoding_mode() {
        let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

        let mode = qr_code.mode();

        // Mode: Byte
        // as u8: 0b0100
        // shifted to take up 8 bits: 0b01000000
        assert_eq!(mode, 0b01000000);
    }

    #[test]
    fn test_version() {
        let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

        let version = qr_code.version();

        //Version: 2
        assert_eq!(version, 2);
    }

    #[test]
    fn test_codewords() {
        let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

        let codewords = qr_code.codewords();

        //Codewords: 48
        //for version 2
        assert_eq!(codewords, 28);
    }

    #[test]
    fn test_string_length() {
        let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

        let string_length = qr_code.string_length();

        //String length: 23
        assert_eq!(string_length, 23);
    }

    #[test]
    fn test_data() {
        let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());

        let data = qr_code.data();

        // "https://www.qrcode.com" encoded using v2 and latin-1
        assert_eq!(data,
            vec![0b01000001, 0b01110110, 0b10000111, 0b01000111, 0b01000111, 0b00000111, 0b00110011, 0b10100010,
                 0b11110010, 0b11110111, 0b01110111, 0b01110111, 0b01110010, 0b11100111, 0b00010111, 0b00100110,
                 0b00110110, 0b11110110, 0b01000110, 0b01010010, 0b11100110, 0b00110110, 0b11110110, 0b11010010,
                 0b11110000, 0b11101100, 0b00010001, 0b11101100]);
    }

}
