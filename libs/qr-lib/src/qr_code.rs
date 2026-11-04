use crate::error::AQRError;
use crate::error_correction::ErrorCorrection;
use crate::gf_math::GFMath;
use crate::mode::Mode;
use crate::qr_constants::{EC_PER_BLOCK, NUM_BLOCKS, QR_CAPACITY};
use crate::version::Version;

type Result<T> = core::result::Result<T, AQRError>;

#[derive(Debug)]
pub struct QRCode {
    mode: u8,
    version: u8,
    string_length: usize,
    error_correction: usize,
    codewords: usize,
    data: Vec<u8>
}

impl QRCode {
    pub fn encode(string: String, version: Option<Version>, error_correction: Option<ErrorCorrection>, mode: Option<Mode>) -> Result<QRCode> {
        // To be removed later
        let supported_versions = vec![
            Version::from(1).unwrap(),
            Version::from(2).unwrap(),
            Version::from(3).unwrap(),
            Version::from(4).unwrap(),
            Version::from(5).unwrap(),
            Version::from(6).unwrap(),
        ];
        let supported_modes = vec![
            Mode::Byte
        ];
        let supported_ec = vec![
            ErrorCorrection::M
        ];

        let string_length = string.len();

        let mode = match mode {
            Some(mode) => mode,
            None => string.as_str().try_into()?
        };
        let error_correction = error_correction.unwrap_or_else(|| ErrorCorrection::M);
        let version = match version {
            Some(version) => version,
            None => Version::choose(string_length, &mode, &error_correction)?
        };

        if !supported_versions.contains(&version) {
            return Err(AQRError::TemporallyUnsupportedVersion);
        }

        if !supported_modes.contains(&mode) {
            return Err(AQRError::TemporallyUnsupportedMode);
        }

        if !supported_ec.contains(&error_correction) {
            return Err(AQRError::TemporallyUnsupportedErrorCorrectionLevel);
        }

        let mode: u8 = mode.into();
        let mode = mode << 4;

        let codewords = QR_CAPACITY[version.0 as usize][error_correction.clone() as usize];

        let error_correction: usize = error_correction.into();

        let mut data = vec![0; codewords];

        Self::encode_data(&mut data, mode, string_length, &string, codewords);

        Ok(QRCode {
            mode,
            version: version.0,
            string_length,
            codewords,
            data,
            error_correction
        })
    }

    fn total_codewords(version: usize) -> usize {
        let mut raw = (16 * version + 128) * version + 64;
        if version >= 2 {
            let a = version / 7 + 2;
            raw -= (25 * a - 10) * a - 55;
            if version >= 7 { raw -= 36; }
        }

        raw / 8
    }

    fn ec_codeword_count(version: usize, ec: usize) -> usize {
        Self::total_codewords(version) - QR_CAPACITY[version][ec]
    }

    fn encode_data(data: &mut Vec<u8>, mode: u8, string_length: usize, string: &String, codewords: usize){
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

        for i in rsi..codewords {
            let index = (i + constant) % 2;

            data[i] = placeholders[index];
        }
    }

    pub(crate) fn get_size(&self) -> u8 {
        self.version * 4 + 17
    }

    fn edc_codewords(data: Vec<u8>, codewords: usize) -> Vec<u8>{
        let math = GFMath::init();

        let degree = codewords - data.len();
        let mut message_poly = vec![0; codewords];
        message_poly[0..data.len()].copy_from_slice(data.as_slice());

        math.pol_divide(message_poly, math.get_pol_generator(degree))
    }

    pub fn get_matrix(&self) -> Vec<Vec<u8>> {
        let math = GFMath::init();

        let size = self.get_size() as usize;

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; size]; size];

        let codewords = self.build_codewords(&math);
        // codewords[..self.data.len()].copy_from_slice(self.data.as_slice());
        // codewords[self.data.len()..].copy_from_slice(Self::edc_codewords(self.data.clone(), total_codewords).as_slice());

        // Top-left square
        Self::fill_area(&mut matrix, 0, 0, 9, 9, 1);
        // Top-right square
        Self::fill_area(&mut matrix, 0, size - 8, 8, 9, 1);
        // Bottom-left square
        Self::fill_area(&mut matrix, size - 8, 0, 9, 8, 1);
        // Alignment pattern

        if self.version != 1 {
            Self::fill_area(&mut matrix, size - 9, size - 9, 5, 5, 1);
        }

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

        // Finder patterns
        // Top-left Top-right Bottom-left
        [(0, 0), (size - 7, 0), (0, size - 7)].iter().for_each(|(row, col)| {
            Self::fill_area(&mut matrix, *row, *col, 7, 7, 1);
            Self::fill_area(&mut matrix, row + 1, col + 1, 5, 5, 0);
            Self::fill_area(&mut matrix, row + 2, col + 2, 3, 3, 1);
        });

        Self::fill_area(&mut matrix, 8, 0, 9, 1, 0);
        Self::fill_area(&mut matrix, 0, 8, 1, 9, 0);
        Self::fill_area(&mut matrix, 8, size - 8, 8, 1, 0);
        Self::fill_area(&mut matrix, size - 8, 8, 1, 8, 0);

        // Separators
        Self::fill_area(&mut matrix, 7, 0, 8, 1, 0);
        Self::fill_area(&mut matrix, 0, 7, 1, 7, 0);
        Self::fill_area(&mut matrix, size - 8, 0, 8, 1, 0);
        Self::fill_area(&mut matrix, 0, size - 8, 1, 7, 0);
        Self::fill_area(&mut matrix, 7, size - 8, 8, 1, 0);
        Self::fill_area(&mut matrix, size - 7, 7, 1, 7, 0);

        // Alignment pattern
        if self.version != 1 {
            Self::fill_area(&mut matrix, size - 9, size - 9, 5, 5, 1);
            Self::fill_area(&mut matrix, size - 8, size - 8, 3, 3, 0);
            matrix[size - 7][size - 7] = 1;
        }

        // Timing patterns
        for pos in (8..size-8).step_by(2) {
            matrix[6][pos] = 1;
            matrix[6][pos + 1] = 0;
            matrix[pos][6] = 1;
            matrix[pos + 1][6] = 0;
        }
        matrix[6][size - 7] = 1;
        matrix[size - 7][6] = 1;

        // Dark module
        matrix[size - 8][8] = 1;

        let mut index = 0;

        for codeword in codewords.clone() {
            for shift in (0..=7).rev() {
                let bit = (codeword >> shift) & 1;
                let (row, col) = sequence[index];
                index += 1;
                matrix[row][col] = bit;
            }
        }

        self.mask_matrix(&sequence, &mut matrix, &codewords);

        let mut format_poly: Vec<u8> = vec![0; 15];

        format_poly[0] = 0 >> 1;
        format_poly[1] = 0 & 1;
        format_poly[2] = 0 >> 2;
        format_poly[3] = (0 >> 1) & 1;
        format_poly[4] = 0 & 1;

        let rest = math.pol_divide(format_poly.clone(), vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1]);

        format_poly[5..].copy_from_slice(&rest);

        let format_mask: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0];

        let format_mask: Vec<u8> = format_poly.iter().enumerate().map(
            |(index, bit)| { bit ^ format_mask[index] }
        ).collect();

        matrix[8][0..6].copy_from_slice(&format_mask[0..6]);
        matrix[8][7..9].copy_from_slice(&format_mask[7..9]);
        matrix[8][size - 8..].copy_from_slice(&format_mask[7..]);
        matrix[7][8] = format_mask[8];

        format_mask[0..7].iter().enumerate().for_each(|(index, cell)| {
           matrix[size - index - 1][8] = *cell;
        });
        format_mask[9..].iter().enumerate().for_each(|(index, cell)| {
            matrix[5 - index][8] = *cell;
        });

        matrix
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

    fn mask_matrix(&self, sequence: &Vec<(usize, usize)>, matrix: &mut Vec<Vec<u8>>, codewords: &Vec<u8>) {
        sequence.iter().enumerate().for_each(|(index, (row, col))| {
            if index >> 3 < codewords.len() {
                let codeword = codewords[index >> 3];
                let bit_shift = 7 - (index & 7);
                let module_bit = (codeword >> bit_shift) & 1;
                matrix[*row][*col] = module_bit ^ (if (row + col) & 1 == 0 { 1 } else { 0 });
            }
        });
    }

    fn build_codewords(&self, math: &GFMath) -> Vec<u8> {
        let version = self.version() as usize;
        let ec = self.error_correction;
        let raw = Self::total_codewords(version);
        let n_blocks = NUM_BLOCKS[ec][version - 1];
        let ec_len = EC_PER_BLOCK[ec][version - 1];
        let n_short = n_blocks - raw % n_blocks;
        let short_len = raw / n_blocks;

        let mut blocks: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        let mut k = 0;
        for i in 0..n_blocks {
            let data_len = short_len - ec_len + if i < n_short { 0 } else { 1 };
            let d = self.data[k..k + data_len].to_vec();
            k += data_len;

            let mut msg = d.clone();
            msg.extend(std::iter::repeat(0).take(ec_len));
            let rem = math.pol_divide(msg, math.get_pol_generator(ec_len));
            blocks.push((d, rem));
        }

        let mut out = Vec::with_capacity(raw);
        let max_data = short_len - ec_len + 1;
        for i in 0..max_data {
            for (d, _) in &blocks {
                if i < d.len() {
                    out.push(d[i]);
                }
            }
        }

        for i in 0..ec_len {
            for (_, e) in &blocks {
                out.push(e[i]);
            }
        }

        out
    }

    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn codewords(&self) -> usize {
        self.codewords
    }

    pub fn string_length(&self) -> usize {
        self.string_length
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
