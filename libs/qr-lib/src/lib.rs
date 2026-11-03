pub mod error;
mod error_correction;
mod gf_math;
mod mode;
mod qr_constants;
mod qr_code;
mod version;

pub use error_correction::ErrorCorrection;
pub use mode::Mode;
pub use qr_code::QRCode;
pub use version::Version;

#[cfg(test)]
mod tests {
    use crate::QRCode;
    use crate::error::AQRError;
    use crate::error_correction::ErrorCorrection;
    use crate::gf_math::GFMath;
    use crate::mode::Mode;
    use crate::version::Version;
    // #[test]
    // fn test_encoding_mode() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let mode = qr_code.mode();
    //
    //     // Mode: Byte
    //     // as u8: 0b0100
    //     // shifted to take up 8 bits: 0b01000000
    //     assert_eq!(mode, 0b01000000);
    // }

    // #[test]
    // fn test_version() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let version = qr_code.version();
    //
    //     //Version: 2
    //     assert_eq!(version, 2);
    // }

    // #[test]
    // fn test_codewords() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let codewords = qr_code.codewords();
    //
    //     //Codewords: 48
    //     //for version 2
    //     assert_eq!(codewords, 28);
    // }

    // #[test]
    // fn test_string_length() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let string_length = qr_code.string_length();
    //
    //     //String length: 23
    //     assert_eq!(string_length, 23);
    // }

    // #[test]
    // fn test_data() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let data = qr_code.data();
    //
    //     // "https://www.qrcode.com" encoded using v2 and latin-1
    //     assert_eq!(data,
    //                vec![0b01000001, 0b01110110, 0b10000111, 0b01000111, 0b01000111, 0b00000111, 0b00110011, 0b10100010,
    //                     0b11110010, 0b11110111, 0b01110111, 0b01110111, 0b01110010, 0b11100111, 0b00010111, 0b00100110,
    //                     0b00110110, 0b11110110, 0b01000110, 0b01010010, 0b11100110, 0b00110110, 0b11110110, 0b11010010,
    //                     0b11110000, 0b11101100, 0b00010001, 0b11101100]);
    // }

    #[test]
    fn test_gf_multiply() {
        let math = GFMath::init();

        let res = math.multiply(3, 5);

        assert_eq!(res, 15);
    }

    #[test]
    fn test_gf_division() {
        let math = GFMath::init();

        let res = math.divide(16, 4);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_pol_multiplication() {
        let math = GFMath::init();

        let pol1 = vec![1, 3, 2];
        let pol2 = vec![2, 1, 7];

        let res = math.pol_multiply(pol1, pol2);

        assert_eq!(res, vec![2, 7, 0, 11, 14]);
    }

    #[test]
    fn test_pol_divide() {
        let math = GFMath::init();

        let pol1 = vec![4, 4, 7, 5];
        let pol2 = vec![2, 1];

        let res = math.pol_divide(pol1, pol2);

        assert_eq!(res, vec![7]);
    }

    // #[test]
    // fn test_size() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let size = qr_code.get_size();
    //
    //     assert_eq!(size, 25);
    // }

    #[test]
    fn test_fill() {
        let mut matrix = vec![vec![0; 6]; 6];

        QRCode::fill_area(&mut matrix, 1, 2, 2, 3, 3);

        assert_eq!(
            matrix,
            vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 3, 3, 0, 0],
                vec![0, 0, 3, 3, 0, 0],
                vec![0, 0, 3, 3, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ]
        );
    }

    // #[test]
    // fn test_get_matrix() {
    //     let qr_code = QRCode::encode(2, "https://www.qrcode.com/".to_string());
    //
    //     let matrix = qr_code.get_matrix();
    // }

    #[test]
    fn test_get_mode() {
        let numeric: Mode = "1234561234342".try_into().unwrap();
        let alphanumeric: Mode = "ADFG23123".try_into().unwrap();
        let latin: Mode = "Fds gds 123 sas".try_into().unwrap();
        let kanji: Mode = "日本語".try_into().unwrap();

        let error: Result<Mode, AQRError> = "Բաֆդասֆ".try_into();

        assert_eq!(numeric, Mode::Numeric);
        assert_eq!(alphanumeric, Mode::Alphanumeric);
        assert_eq!(latin, Mode::Byte);
        assert_eq!(kanji, Mode::Kanji);

        assert!(error.is_err());

        let numeric: Mode = 'N'.try_into().unwrap();
        let alphanumeric: Mode = 'A'.try_into().unwrap();
        let latin: Mode = 'B'.try_into().unwrap();
        let kanji: Mode = 'K'.try_into().unwrap();

        let error: Result<Mode, AQRError> = 'V'.try_into();

        assert_eq!(numeric, Mode::Numeric);
        assert_eq!(alphanumeric, Mode::Alphanumeric);
        assert_eq!(latin, Mode::Byte);
        assert_eq!(kanji, Mode::Kanji);

        assert!(error.is_err());
    }

    #[test]
    fn test_get_error_correction() {
        let l: ErrorCorrection = 'L'.try_into().unwrap();
        let m: ErrorCorrection = 'M'.try_into().unwrap();
        let q: ErrorCorrection = 'Q'.try_into().unwrap();
        let h: ErrorCorrection = 'H'.try_into().unwrap();

        let error: Result<ErrorCorrection, AQRError> = 'V'.try_into();

        assert_eq!(l, ErrorCorrection::L);
        assert_eq!(m, ErrorCorrection::M);
        assert_eq!(q, ErrorCorrection::Q);
        assert_eq!(h, ErrorCorrection::H);

        assert!(error.is_err());
    }

    #[test]
    fn test_version() {
        let version: Version = 40.try_into().unwrap();
        let error: Result<Version, AQRError> = (-70).try_into();

        assert_eq!(version, Version(40));
        assert!(error.is_err());
    }

    #[test]
    fn test_choose_version() {
        let version = Version::choose(23, &Mode::Byte, &ErrorCorrection::M);
        let unsupported_version = Version::choose(4000, &Mode::Byte, &ErrorCorrection::L);

        assert_eq!(version.unwrap().0, 2);
        assert!(unsupported_version.is_err());
    }
}
