use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum AQRError {
    UnsupportedErrorCorrectionLevel,
    UnsupportedVersion,
    UnsupportedMode,
}

impl Display for AQRError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
