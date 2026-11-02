use crate::error::AQRError;
use regex::regex;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl TryInto<Mode> for char {
    type Error = AQRError;

    fn try_into(self) -> Result<Mode, Self::Error> {
        match self {
            'A' => Ok(Mode::Alphanumeric),
            'N' => Ok(Mode::Numeric),
            'B' => Ok(Mode::Byte),
            'K' => Ok(Mode::Kanji),
            _ => Err(Self::Error::UnsupportedMode),
        }
    }
}

impl TryInto<Mode> for &str {
    type Error = AQRError;

    fn try_into(self) -> Result<Mode, Self::Error> {
        if regex!(r"^[0-9]+$").is_match(self) {
            return Ok(Mode::Numeric);
        }

        if regex!(r"^[0-9A-Z $%*+\-.\/:]*$").is_match(self) {
            return Ok(Mode::Alphanumeric);
        }

        if regex!(r"^[\x00-\xFF]+$").is_match(self) {
            return Ok(Mode::Byte);
        }

        if regex!(r"^[\u4E00-\u9FFF]+$").is_match(self) {
            return Ok(Mode::Kanji);
        }

        Err(Self::Error::UnsupportedMode)
    }
}

impl Into<u8> for Mode {
    fn into(self) -> u8 {
        match self {
            Mode::Numeric => 0b0001,
            Mode::Alphanumeric => 0b0010,
            Mode::Byte => 0b0100,
            Mode::Kanji => 0b1000,
        }
    }
}
