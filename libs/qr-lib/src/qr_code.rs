pub struct QRCode {
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

        Self::encode_data(&mut data, mode, string_length, &string, codewords);

        QRCode {
            mode,
            version,
            string_length,
            codewords,
            data
        }
    }

    fn encode_data(data: &mut Vec<u8>, mode: u8, string_length: usize, string: &String, codewords: u8){
        data[0] = mode | (string_length as u8 >> 4);
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
