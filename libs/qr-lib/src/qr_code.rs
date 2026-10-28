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

    pub(crate) fn get_size(&self) -> u8 {
        self.version * 4 + 17
    }

    pub(crate) fn get_matrix(&self) {
        let size = self.get_size() as usize;

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; size]; size];

        // Top-left square
        Self::fill_area(&mut matrix, 0, 0, 9, 9, 1);
        // Top-right square
        Self::fill_area(&mut matrix, 0, size - 8, 8, 9, 1);
        // Bottom-left square
        Self::fill_area(&mut matrix, size - 8, 0, 9, 8, 1);
        // Alignment pattern
        Self::fill_area(&mut matrix, size - 9, size - 9, 5, 5, 1);

        // Timing patterns
        Self::fill_area(&mut matrix, 6, 9, self.version as usize * 4, 1, 1);
        Self::fill_area(&mut matrix, 9, 6, 1, self.version as usize * 4, 1);

        matrix[size - 8][8] = 1;

        let mut row_step: isize = -1;
        let mut row: isize = (size - 1) as isize;
        let mut column: isize = (size - 1) as isize;
        let mut sequence: Vec<(usize, usize)> = Vec::new();
        let mut index = 0;

        while column >= 0 {
            if matrix[row as usize][column as usize] == 0 {
                sequence.push((row as usize, column as usize));
            }

            if (index & 1) != 0 {
                row += row_step;
                if row == -1 || row == size as isize {
                    row_step = -row_step;
                    row += row_step;
                    column -= if column == 7 { 2 } else { 1 };
                } else {
                    column += 1;
                }
            } else {
                column -= 1;
            }
            index += 1;
        }

        let mut index = 0;
        for codeword in &self.data {
            for shift in (0..=7).rev() {
                let bit = (codeword >> shift) & 1;
                let (row, col) = sequence[index];
                index += 1;
                matrix[row][col] = bit;
            }
        }

        for row in matrix {
            print!("[");
            for val in row {
                print!("{},", val); // Right-aligns elements with width 6
            }
            println!("]");
        }
    }

    pub(crate) fn fill_area(matrix: &mut Vec<Vec<u8>>, row: usize, column: usize, width: usize, height: usize, fill: u8) {
        for r in row..(row + height) {
            if r < matrix.len() {
                for c in column..(column + width) {
                    if c < matrix[r].len() {
                        matrix[r][c] = fill;
                    }
                }
            }
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
