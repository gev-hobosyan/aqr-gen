use crate::error::AQRError;

#[derive(Debug, PartialEq)]
pub enum ErrorCorrection {
    L,
    M,
    Q,
    H,
}

impl TryInto<ErrorCorrection> for char {
    type Error = AQRError;

    fn try_into(self) -> Result<ErrorCorrection, Self::Error> {
        match self {
            'L' => Ok(ErrorCorrection::L),
            'M' => Ok(ErrorCorrection::M),
            'Q' => Ok(ErrorCorrection::Q),
            'H' => Ok(ErrorCorrection::H),
            _ => Err(Self::Error::UnsupportedErrorCorrectionLevel),
        }
    }
}
impl Into<usize> for ErrorCorrection {
    fn into(self) -> usize {
        match self {
            ErrorCorrection::L => 0,
            ErrorCorrection::M => 1,
            ErrorCorrection::Q => 2,
            ErrorCorrection::H => 3,
        }
    }
}

impl Into<usize> for &ErrorCorrection {
    fn into(self) -> usize {
        match self {
            ErrorCorrection::L => 0,
            ErrorCorrection::M => 1,
            ErrorCorrection::Q => 2,
            ErrorCorrection::H => 3,
        }
    }
}
