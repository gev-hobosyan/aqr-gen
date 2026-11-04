use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum AQRError {
    UnsupportedErrorCorrectionLevel,
    UnsupportedVersion,
    UnsupportedMode,
    TemporallyUnsupportedErrorCorrectionLevel,
    TemporallyUnsupportedVersion,
    TemporallyUnsupportedMode,
    Error(String)
}

impl Display for AQRError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();

        match self {
            AQRError::UnsupportedErrorCorrectionLevel => {
                msg = "Error correction level is not supported".to_string();
            }
            AQRError::UnsupportedVersion => {
                msg = "The QR code version is not supported.".to_string();
            }
            AQRError::UnsupportedMode => {
                msg = "The QR code mode is not supported.".to_string();
            }
            AQRError::TemporallyUnsupportedErrorCorrectionLevel => {
                msg = "Error correction level is not supported. Support will be added later".to_string();

            }
            AQRError::TemporallyUnsupportedVersion => {
                msg = "The QR code version is not supported. Support will be added later".to_string();

            }
            AQRError::TemporallyUnsupportedMode => {
                msg = "The QR code mode is not supported. Support will be added later".to_string();
            }
            AQRError::Error(err) => {
                msg = err.to_string()
            }
        }

        write!(f, "{}", msg)
    }
}
