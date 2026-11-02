use crate::error::AQRError;
use crate::qr_capacity::QR_CAPACITY;
use crate::{ErrorCorrection, Mode};

#[derive(Debug, PartialEq)]
pub struct Version(pub(crate) u8);

impl Version {
    pub fn from(version: u8) -> Self {
        Self(version)
    }

    pub fn choose(
        data_count: usize,
        mode: &Mode,
        error_correction: &ErrorCorrection,
    ) -> Result<Version, AQRError> {
        let ec: usize = error_correction.into();
        let version =
            (0..=40).find(|&v| Self::codewords_needed(v, data_count, mode) <= QR_CAPACITY[v][ec]);

        match version {
            Some(v) => Ok(Self::from(v as u8)),
            None => Err(AQRError::UnsupportedVersion),
        }
    }

    fn size_bits(version: usize, mode: &Mode) -> usize {
        let mode = match mode {
            Mode::Numeric => 0,
            Mode::Alphanumeric => 1,
            Mode::Byte => 2,
            Mode::Kanji => 3,
        };

        match version {
            0..=9 => [10, 9, 8, 8][mode],
            10..=26 => [12, 11, 16, 10][mode],
            _ => [14, 13, 16, 12][mode],
        }
    }

    fn payload_bits(n: usize, mode: &Mode) -> usize {
        match mode {
            Mode::Numeric => 10 * (n / 3) + [0, 4, 7][n % 3],
            Mode::Alphanumeric => 11 * (n / 2) + 6 * (n % 2),
            Mode::Byte => 8 * n,
            Mode::Kanji => 13 * n,
        }
    }

    fn codewords_needed(version: usize, n: usize, mode: &Mode) -> usize {
        (4 + Self::size_bits(version, mode) + Self::payload_bits(n, mode) + 7) / 8
    }
}

impl TryInto<Version> for i32 {
    type Error = AQRError;

    fn try_into(self) -> Result<Version, Self::Error> {
        if self >= 0 && self <= 40 {
            Ok(Version(self as u8))
        } else {
            Err(AQRError::UnsupportedVersion)
        }
    }
}
