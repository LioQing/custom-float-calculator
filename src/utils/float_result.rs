use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatErr {
    pub format: Format,
    pub msg: String,
}

pub type FloatResult = Result<Float, FloatErr>;

pub trait FloatResultExt {
    fn float(&self) -> Float;
    fn format(&self) -> &Format;

    fn ok_zero(format: Format) -> Self;
}

impl FloatResultExt for FloatResult {
    /// Get the float, if it is Err, return 0.
    fn float(&self) -> Float {
        match self {
            Ok(f) => f.clone(),
            Err(FloatErr { format, .. }) => Float::from_str(format.clone(), "0").unwrap(),
        }
    }

    /// Get the format.
    fn format(&self) -> &Format {
        match self {
            Ok(f) => &f.format,
            Err(FloatErr { format, .. }) => format,
        }
    }

    /// Get a FloatResult with Ok(Float::from_str(format, "0").unwrap()).
    fn ok_zero(format: Format) -> Self {
        Ok(Float::from_str(format, "0").unwrap())
    }
}